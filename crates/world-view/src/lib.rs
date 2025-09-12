use crate::depth_clipping::{DepthClipping, ScreenOcclusion, SegmentFragment};
use bevy::{
    ecs::{
        schedule::ScheduleLabel,
        system::{SystemParam, SystemState},
    },
    prelude::*,
};
use game_state::conditions::in_level_state;
use level::{prelude::Camera, prelude::*};
use moonshine_kind::Instance;
use wad::prelude::*;
use window::ScreenBuffer;

mod depth_clipping;

/// Plugin responsible for rendering the 3D view of the world.
#[derive(Default)]
pub struct WorldViewPlugin;

impl Plugin for WorldViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RenderSubSector>()
            .add_event::<RenderSegment>()
            .add_event::<RenderFragment>()
            .init_resource::<ScreenOcclusion>()
            .init_resource::<VerticalClip>()
            .init_schedule(RenderSubSectorSchedule)
            .add_systems(
                PostUpdate,
                (prepare, render_sectors).chain().run_if(in_level_state()),
            )
            .add_systems(
                RenderSubSectorSchedule,
                (render_sub_sector, render_segments, render_fragments).chain(),
            );
    }
}

fn prepare(mut depth_clipping: DepthClipping, mut vertical_clip: ResMut<VerticalClip>) {
    depth_clipping.reset();
    vertical_clip.reset();
}

/// Render solid walls and portals (two-sided lines that connect sectors).
/// These are always perpendicular to the player's ground plane and
/// define the world boundary.
fn render_sectors(world: &mut World) {
    let mut screen = world.resource_mut::<ScreenBuffer>();
    screen.fill(0);

    let mut system_state: SystemState<(BspTree, Query<&MapObject, With<Player>>)> =
        SystemState::new(world);
    let (bsp_tree, player_query) = system_state.get(world);
    let player = player_query.single();

    // PERFORMANCE: Add bounding boxes for each node to determine if it is visible.
    // PERFORMANCE: Add an early-out after filling all horizontal screen space.
    for node in bsp_tree.iter(player.pos) {
        if let BspNode::Leaf(sub_sector) = node.as_ref() {
            world.send_event(RenderSubSector(*sub_sector));
            world.run_schedule(RenderSubSectorSchedule);
        }
    }
}

fn render_sub_sector(
    mut render_sub_sector: EventReader<RenderSubSector>,
    mut render_segment: EventWriter<RenderSegment>,
    sub_sector_query: Query<&SubSector>,
) {
    let Some(RenderSubSector(sub_sector)) = render_sub_sector.read().next() else {
        return;
    };
    let sub_sector = sub_sector_query.get(sub_sector.entity()).unwrap();
    for segment in &sub_sector.segments {
        render_segment.send(RenderSegment(*segment));
    }
}

fn render_segments(
    mut render_segment: EventReader<RenderSegment>,
    mut render_fragments: EventWriter<RenderFragment>,
    camera_query: Query<&Camera, With<Player>>,
    segment_query: Query<&LineSegment>,
    mut depth_clip: DepthClipping,
) {
    let camera = camera_query.single();
    for RenderSegment(segment) in render_segment.read() {
        if depth_clip.is_fully_occluded() {
            break;
        }
        let seg_inst = *segment;
        let segment = segment_query.get(segment.entity()).unwrap();
        let Some((x1, x2)) = camera.world_to_viewport(segment) else {
            continue;
        };
        // At this point, the segment is within the view frustum and projected to screen space.
        // However, it may still be occluded by previously rendered segments.
        let fragments = depth_clip.clip(segment, x1, x2);
        for fragment in fragments {
            render_fragments.send(RenderFragment {
                fragment,
                segment: seg_inst,
            });
        }
    }
}

fn render_fragments(
    mut render_fragments: EventReader<RenderFragment>,
    info_extractor: DrawInfoExtractor,
    mut vertical_clip: ResMut<VerticalClip>,
    mut screen: ResMut<ScreenBuffer>,
) {
    for ev in render_fragments.read() {
        let fragment = ev.fragment;
        let segment = ev.segment;

        let mut info = info_extractor.extract(segment, fragment);

        let dx = (info.x2 - info.x1) as f32;
        let length = (info.v2 - info.v1).length();

        for i in info.x1..=info.x2 {
            let mut yl = info.top.ceil() as i32;
            if yl < vertical_clip.ceiling[i] + 1 {
                yl = vertical_clip.ceiling[i] + 1;
            }

            let mut yh = info.bottom.floor() as i32;
            if yh >= vertical_clip.floor[i] {
                yh = vertical_clip.floor[i] - 1;
            }

            let s = (i - info.x1) as f32 / dx;
            let t = s * info.v1.y / (s * info.v1.y + (1.0 - s) * info.v2.y);
            let offset = info.base_offset + (length * t);

            let inv_scale = info.scale1.recip();

            info.scale1 += info.scale_step;
            info.top += info.top_step;
            info.bottom += info.bottom_step;

            if let Some(mid_tex) = &info.mid_tex {
                let tex = mid_tex.tex;
                let tex_col = (offset as usize) % tex.width();
                draw_col(
                    &mut screen,
                    i,
                    tex_col,
                    yl,
                    yh,
                    100,
                    tex,
                    mid_tex.tex_mid,
                    inv_scale,
                );
                continue;
            }

            if let Some(top_tex) = &info.top_tex {
                let mut yh = info.pix_high.floor() as i32;
                info.pix_high += info.pix_high_step;

                if vertical_clip.floor[i] <= yh {
                    yh = vertical_clip.floor[i] - 1;
                }
                if yl <= yh {
                    let tex = top_tex.tex;
                    let tex_col = (offset as usize) % tex.width();
                    draw_col(
                        &mut screen,
                        i,
                        tex_col,
                        yl,
                        yh,
                        100,
                        tex,
                        top_tex.tex_mid,
                        inv_scale,
                    );
                    vertical_clip.ceiling[i] = yh;
                } else {
                    vertical_clip.ceiling[i] = yl - 1;
                }
            } else if info.mark_ceiling {
                vertical_clip.ceiling[i] = yl - 1;
            }

            if let Some(bottom_tex) = &info.bottom_tex {
                let mut yl = info.pix_low.ceil() as i32;
                info.pix_low += info.pix_low_step;

                if yl <= vertical_clip.ceiling[i] {
                    yl = vertical_clip.ceiling[i] + 1;
                }
                if yl <= yh {
                    let tex = bottom_tex.tex;
                    let tex_col = (offset as usize) % tex.width();
                    draw_col(
                        &mut screen,
                        i,
                        tex_col,
                        yl,
                        yh,
                        100,
                        tex,
                        bottom_tex.tex_mid,
                        inv_scale,
                    );
                    vertical_clip.floor[i] = yl;
                } else {
                    vertical_clip.floor[i] = yh + 1;
                }
            } else if info.mark_floor {
                vertical_clip.floor[i] = yh + 1;
            }
        }
    }
}

#[derive(SystemParam)]
struct DrawInfoExtractor<'w, 's> {
    camera_query: Query<'w, 's, &'static Camera, With<Player>>,
    line_query: Query<'w, 's, &'static Line>,
    sector_query: Query<'w, 's, &'static Sector>,
    segment_query: Query<'w, 's, &'static LineSegment>,
    side_query: Query<'w, 's, &'static SideDef>,
    wad: Res<'w, WadFile>,
}

impl DrawInfoExtractor<'_, '_> {
    fn extract(&self, segment: Instance<LineSegment>, fragment: SegmentFragment) -> DrawInfo {
        let segment = self.segment_query.get(*segment).unwrap();
        let side = self.side_query.get(*segment.side()).unwrap();
        let line = self.line_query.get(*segment.line()).unwrap();

        let front_sec = self.sector_query.get(*segment.front_sector()).unwrap();
        let back_sec = match segment {
            LineSegment::Wall(_) => None,
            LineSegment::Portal(portal) => {
                let back_sec = self.sector_query.get(*portal.back_sector).unwrap();
                Some(back_sec)
            }
        };

        let camera = self.camera_query.single();

        let mut info = DrawInfo::default();
        self.set_ends(&mut info, segment, fragment, camera);
        self.set_scales(&mut info, camera);
        self.set_world_bounds(&mut info, front_sec, back_sec);
        self.set_projection(&mut info);
        self.set_base_offset(&mut info, segment, side, camera);
        self.set_tex(&mut info, line, side, front_sec, back_sec);

        info
    }

    fn set_ends(
        &self,
        info: &mut DrawInfo,
        segment: &LineSegment,
        fragment: SegmentFragment,
        camera: &Camera,
    ) {
        info.x1 = fragment.start;
        info.x2 = fragment.end;
        info.v1 = camera.viewport_to_world(segment, info.x1);
        info.v2 = camera.viewport_to_world(segment, info.x2);
    }

    fn set_scales(&self, info: &mut DrawInfo, camera: &Camera) {
        info.scale1 = camera.find_scale(info.v1);
        info.scale2 = camera.find_scale(info.v2);
        info.scale_step = if info.x1 < info.x2 {
            let ds = info.scale2 - info.scale1;
            let dx = (info.x2 - info.x1) as f32;
            ds / dx
        } else {
            0.0
        };
    }

    fn set_world_bounds(&self, info: &mut DrawInfo, front_sec: &Sector, back_sec: Option<&Sector>) {
        info.world_top = front_sec.ceiling_height - 41.0;
        info.world_bottom = front_sec.floor_height - 41.0;

        let Some(back_sec) = back_sec else {
            info.mark_ceiling = true;
            info.mark_floor = true;
            return;
        };

        info.world_high = back_sec.ceiling_height - 41.0;
        info.world_low = back_sec.floor_height - 41.0;
        let sky = self.wad.flats().get_index_of("F_SKY1").unwrap();
        if front_sec.ceiling_tex == sky && back_sec.ceiling_tex == sky {
            // Hack to allow height changes in outdoor areas.
            info.world_top = info.world_high;
        }

        if info.world_low != info.world_bottom
            || back_sec.floor_tex != front_sec.floor_tex
            || back_sec.light_level != front_sec.light_level
        {
            info.mark_floor = true;
        } else {
            // same plane on both sides
            info.mark_floor = false;
        }
        if info.world_high != info.world_top
            || back_sec.ceiling_tex != front_sec.ceiling_tex
            || back_sec.light_level != front_sec.light_level
        {
            info.mark_ceiling = true;
        } else {
            // same plane on both sides
            info.mark_ceiling = false;
        }
        if back_sec.ceiling_height <= front_sec.floor_height
            || back_sec.floor_height >= front_sec.ceiling_height
        {
            // closed door
            info.mark_ceiling = true;
            info.mark_floor = true;
        }
        // if a floor / ceiling plane is on the wrong side
        //  of the view plane, it is definitely invisible
        //  and doesn't need to be marked.
        if front_sec.floor_height >= 41.0 {
            // above view plane
            info.mark_floor = false;
        }
        if front_sec.ceiling_height <= 41.0 && front_sec.ceiling_tex != sky {
            // below view plane
            info.mark_ceiling = false;
        }
    }

    fn set_projection(&self, info: &mut DrawInfo) {
        info.top = 100.0 - (info.world_top * info.scale1);
        info.top_step = -(info.scale_step * info.world_top);
        info.bottom = 100.0 - (info.world_bottom * info.scale1);
        info.bottom_step = -(info.scale_step * info.world_bottom);
        if info.world_high < info.world_top {
            info.pix_high = 100.0 - (info.world_high * info.scale1);
            info.pix_high_step = -(info.scale_step * info.world_high);
        }
        if info.world_low > info.world_bottom {
            info.pix_low = 100.0 - (info.world_low * info.scale1);
            info.pix_low_step = -(info.scale_step * info.world_low);
        }
    }

    fn set_base_offset(
        &self,
        info: &mut DrawInfo,
        segment: &LineSegment,
        side: &SideDef,
        camera: &Camera,
    ) {
        info.base_offset += side.x_offset;
        info.base_offset += segment.offset();
        info.base_offset += (info.v1 - camera.world_to_camera(segment.v1())).length();
    }

    fn set_tex<'a>(
        &'a self,
        info: &mut DrawInfo<'a>,
        line: &Line,
        side: &SideDef,
        front_sec: &Sector,
        back_sec: Option<&Sector>,
    ) {
        if let Some(back_sec) = back_sec {
            self.set_portal_tex(info, line, side, back_sec);
        } else {
            self.set_wall_tex(info, line, side, front_sec);
        }
    }

    fn set_wall_tex<'a>(
        &'a self,
        info: &mut DrawInfo<'a>,
        line: &Line,
        side: &SideDef,
        front_sec: &Sector,
    ) {
        let textures = self.wad.wall_textures();
        let mid_tex = textures.get_by_index(side.middle_texture).unwrap();
        let mut data = TextureInfoData::new(mid_tex);
        if line.flags() & 16 != 0 {
            let tex_height = data.tex.height() as f32;
            let vtop = front_sec.floor_height + tex_height;
            data.tex_mid = vtop - 41.0;
        } else {
            data.tex_mid = info.world_top;
        };
        data.tex_mid += side.y_offset;

        info.mid_tex = Some(data);
    }

    fn set_portal_tex<'a>(
        &'a self,
        info: &mut DrawInfo<'a>,
        line: &Line,
        side: &SideDef,
        back_sec: &Sector,
    ) {
        let textures = self.wad.wall_textures();
        let mut toptexture = None;
        let mut bottomtexture = None;
        if info.world_high < info.world_top {
            // top texture
            let top_tex = textures.get_by_index(side.top_texture).unwrap();
            let mut data = TextureInfoData::new(top_tex);
            if line.flags() & 8 != 0 {
                // top of texture at top
                data.tex_mid = info.world_top;
            } else {
                // bottom of texture
                let vtop = back_sec.ceiling_height + data.tex.height() as f32;
                data.tex_mid = vtop - 41.0;
            }
            data.tex_mid += side.y_offset;
            toptexture = Some(data);
        }
        if info.world_low > info.world_bottom {
            // bottom texture
            let bottom_tex = textures.get_by_index(side.lower_texture).unwrap();
            let mut data = TextureInfoData::new(bottom_tex);
            if line.flags() & 16 != 0 {
                // bottom of texture at bottom
                // top of texture at top
                data.tex_mid = info.world_top;
            } else {
                // top of texture at top
                data.tex_mid = info.world_low;
            }
            data.tex_mid += side.y_offset;
            bottomtexture = Some(data);
        }

        info.top_tex = toptexture;
        info.bottom_tex = bottomtexture;
    }
}

#[derive(Debug, Default)]
struct DrawInfo<'a> {
    //====================================== SCREEN PROJECTION =====================================
    x1: usize,
    x2: usize,
    //====================================== SCREEN BOUNDS =========================================

    //====================================== FRAGMENT ENDS =========================================
    v1: Vec2,
    v2: Vec2,
    //====================================== FRAGMENT ENDS =========================================

    //====================================== SCREEN PROJECTION =====================================
    world_top: f32,
    world_bottom: f32,
    world_high: f32,
    world_low: f32,
    top: f32,
    top_step: f32,
    bottom: f32,
    bottom_step: f32,
    pix_high: f32,
    pix_high_step: f32,
    pix_low: f32,
    pix_low_step: f32,
    scale1: f32,
    scale_step: f32,
    scale2: f32,
    //====================================== SCREEN PROJECTION =====================================

    //====================================== TEXTURE MAPPING =======================================
    base_offset: f32,
    //====================================== TEXTURE MAPPING =======================================

    //====================================== FLATS CALCULATION =====================================
    mark_ceiling: bool,
    mark_floor: bool,
    //====================================== FLATS CALCULATION =====================================

    //====================================== TEXTURE DATA ==========================================
    mid_tex: Option<TextureInfoData<'a>>,
    top_tex: Option<TextureInfoData<'a>>,
    bottom_tex: Option<TextureInfoData<'a>>,
    //====================================== TEXTURE DATA ==========================================
}

#[derive(Debug)]
struct TextureInfoData<'a> {
    tex: &'a WallTexture,
    tex_mid: f32,
}

impl<'a> TextureInfoData<'a> {
    fn new(tex: &'a WallTexture) -> Self {
        Self { tex, tex_mid: 0.0 }
    }
}

fn draw_col(
    screen: &mut ScreenBuffer,
    i: usize,
    tex_col: usize,
    yl: i32,
    yh: i32,
    center_y: usize,
    texture: &WallTexture,
    texture_mid: f32,
    inv_scale: f32,
) {
    for y in yl..=yh {
        // Linear interpolate texture coordinate.
        let dy = y - (center_y as i32);
        let texture_frac_y = texture_mid + ((dy as f32) * inv_scale);

        // Index texture and retrieve color.
        let texture_y = (texture_frac_y.trunc() as usize) % texture.height();
        let color = texture[(tex_col, texture_y)];

        // Draw!
        screen[(i, y as usize)] = color;
    }
}

#[derive(Resource)]
pub struct VerticalClip {
    floor: Vec<i32>,
    ceiling: Vec<i32>,
}

impl Default for VerticalClip {
    fn default() -> Self {
        Self {
            floor: vec![200; 320],
            ceiling: vec![-1; 320],
        }
    }
}

impl VerticalClip {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RenderSubSectorSchedule;

#[derive(Event, Debug)]
pub struct RenderSubSector(Instance<SubSector>);

#[derive(Event, Debug)]
pub struct RenderSegment(Instance<LineSegment>);

#[derive(Event, Clone, Copy, Debug)]
pub struct RenderFragment {
    fragment: SegmentFragment,
    segment: Instance<LineSegment>,
}
