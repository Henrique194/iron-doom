use super::MapObject;
use crate::components::LineSegment;
use bevy::prelude::*;

/// A 90° Field of View.
const FOV: f32 = std::f32::consts::FRAC_PI_2;
const HALF_FOV: f32 = FOV / 2.0;

#[derive(Component)]
pub struct Camera {
    position: Vec2,
    x_axis: Dir2,
    y_axis: Dir2,
    view_frustum: ViewFrustum,
}

impl Camera {
    pub(crate) fn new(player: MapObject) -> Self {
        let normal = player.dir;
        Self {
            position: player.pos,
            x_axis: Rot2::radians(-FOV) * normal,
            y_axis: normal,
            view_frustum: ViewFrustum::new(),
        }
    }

    pub fn update(&mut self, player: MapObject) {
        *self = Self::new(player);
    }

    pub fn projection_plane(&self) -> ProjectionPlane {
        self.view_frustum.projection_plane
    }

    pub fn find_scale(&self, point: Vec2) -> f32 {
        let proj_plane = self.projection_plane();
        let numerator = proj_plane.origin.dot(*proj_plane.normal);
        let denominator = proj_plane.normal.dot(point);
        numerator / denominator
    }

    pub fn viewport_to_world(&self, segment: &LineSegment, x: usize) -> Vec2 {
        let proj_plane = self.projection_plane();
        let origin = proj_plane.origin;
        let end = proj_plane.end;

        let v1 = origin + (x as f32 / 320.0) * (end - origin);

        let p1 = Vec2::ZERO;
        let p2 = v1;
        let p3 = self.world_to_camera(segment.v1());
        let p4 = self.world_to_camera(segment.v2());

        let numerator = ((p1.x - p3.x) * (p3.y - p4.y)) - ((p1.y - p3.y) * (p3.x - p4.x));
        let denominator = ((p1.x - p2.x) * (p3.y - p4.y)) - ((p1.y - p2.y) * (p3.x - p4.x));
        let distance = numerator / denominator;

        distance * v1
    }

    pub fn world_to_viewport(&self, segment: &LineSegment) -> Option<(usize, usize)> {
        if (segment.v1() - self.position).dot(*segment.normal()) > 0.0 {
            // Backface culling:
            // Segment is facing away the player, so don't render it.
            return None;
        }
        let v1 = self.world_to_camera(segment.v1());
        let v2 = self.world_to_camera(segment.v2());
        self.view_frustum.world_to_viewport(v1, v2)
    }

    pub fn world_to_camera(&self, mut point: Vec2) -> Vec2 {
        point -= self.position;
        let x = point.dot(*self.x_axis);
        let y = point.dot(*self.y_axis);
        point.x = x;
        point.y = y;
        (x, y).into()
    }
}

struct ViewFrustum {
    projection_plane: ProjectionPlane,
    left_clip_plane: Plane2d,
    right_clip_plane: Plane2d,
}

impl ViewFrustum {
    fn new() -> Self {
        let normal = Dir2::Y;
        let origin_dir = Rot2::radians(HALF_FOV) * normal;
        let end_dir = Rot2::radians(-HALF_FOV) * normal;
        let distance = 160.0 / f32::cos(HALF_FOV);

        let projection_plane = ProjectionPlane {
            normal,
            origin: distance * origin_dir,
            end: distance * end_dir,
        };
        let left_clip_plane = Plane2d {
            normal: Rot2::radians(-FOV) * origin_dir,
        };
        let right_clip_plane = Plane2d {
            normal: Rot2::radians(FOV) * end_dir,
        };

        Self {
            projection_plane,
            left_clip_plane,
            right_clip_plane,
        }
    }

    fn world_to_viewport(&self, v1: Vec2, v2: Vec2) -> Option<(usize, usize)> {
        let (v1, v2) = self.clip_line(v1, v2)?;
        self.project_line(v1, v2)
    }

    fn clip_line(&self, v1: Vec2, v2: Vec2) -> Option<(Vec2, Vec2)> {
        if v1.y <= 0.0 && v2.y <= 0.0 {
            // Fully clipped: segment is behind camera.
            return None;
        }
        let clipped_v1 = self.clip_point(v1);
        let clipped_v2 = self.clip_point(v2);
        if clipped_v1 == clipped_v2 {
            return None;
        }
        Some((clipped_v1, clipped_v2))
    }

    fn clip_point(&self, point: Vec2) -> Vec2 {
        if point.x < 0.0 && point.dot(*self.left_clip_plane.normal) < 0.0 {
            return self.projection_plane.origin;
        }
        if point.x > 0.0 && point.dot(*self.right_clip_plane.normal) < 0.0 {
            return self.projection_plane.end;
        }
        point
    }

    fn project_line(&self, v1: Vec2, v2: Vec2) -> Option<(usize, usize)> {
        let x1 = self.projection_plane.project(v1).ceil() as usize;
        let x2 = self.projection_plane.project(v2).ceil() as usize;
        if x1 >= x2 {
            return None;
        }
        Some((x1, x2 - 1))
    }
}

#[derive(Clone, Copy)]
pub struct ProjectionPlane {
    pub normal: Dir2,
    pub origin: Vec2,
    pub end: Vec2,
}

impl ProjectionPlane {
    fn project(&self, point: Vec2) -> f32 {
        // We are projecting a point onto the projection plane. The point may either already
        // intersect the plane or lie on an infinite line that passes through the origin and
        // the point. We want to find the intersection of this line with the projection plane,
        // which gives us a position between the origin and the end of the plane. The parameter
        // 'distance' represents how far along the line from the origin the intersection occurs.
        // This value will range from 0.0 to 1.0:
        // - A value of 0.0 means the intersection is at the origin.
        // - A value of 1.0 means the intersection is at the end of the plane.
        // The parameter 'distance' is actually just the first degree Bézier parameter,
        // calculated using the formula for the line-line intersection.
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
        let p1 = self.origin;
        let p2 = self.end;
        let numerator = (p1.y * point.x) - (p1.x * point.y);
        let denominator = (p1.y - p2.y) * point.x - (p1.x - p2.x) * point.y;
        let mut distance = numerator / denominator;
        // Make sure we don't exceed the screen space bounds.
        distance = distance.clamp(0.0, 1.0);
        320.0 * distance
    }
}
