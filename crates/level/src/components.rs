use crate::resources::LevelMap;
use bevy::{ecs::system::SystemParam, prelude::*};
use moonshine_kind::Instance;
use std::rc::Rc;

#[derive(Component)]
pub struct Sector {
    #[allow(unused)]
    pub floor_height: f32,
    #[allow(unused)]
    pub ceiling_height: f32,
    #[allow(unused)]
    pub floor_tex: usize,
    #[allow(unused)]
    pub ceiling_tex: usize,
    #[allow(unused)]
    pub light_level: i16,
    #[allow(unused)]
    pub special: i16,
    #[allow(unused)]
    pub tag: i16,
}

#[derive(Component)]
pub struct SubSector {
    pub segments: Vec<Instance<LineSegment>>,
}

#[derive(Component)]
pub struct SideDef {
    #[allow(unused)]
    pub x_offset: f32,
    #[allow(unused)]
    pub y_offset: f32,
    #[allow(unused)]
    pub top_texture: usize,
    #[allow(unused)]
    pub lower_texture: usize,
    #[allow(unused)]
    pub middle_texture: usize,
    #[allow(unused)]
    pub sector: Instance<Sector>,
}

#[derive(Component)]
pub enum Line {
    Wall(WallLine),
    Portal(PortalLine),
}

impl Line {
    pub fn v1(&self) -> Vec2 {
        match self {
            Line::Wall(WallLine { v1, .. }) => *v1,
            Line::Portal(PortalLine { v1, .. }) => *v1,
        }
    }

    pub fn v2(&self) -> Vec2 {
        match self {
            Line::Wall(WallLine { v2, .. }) => *v2,
            Line::Portal(PortalLine { v2, .. }) => *v2,
        }
    }

    pub fn flags(&self) -> i16 {
        match self {
            Line::Wall(WallLine { flags, .. }) => *flags,
            Line::Portal(PortalLine { flags, .. }) => *flags,
        }
    }

    pub fn front_sector(&self) -> Instance<Sector> {
        match self {
            Line::Wall(WallLine { front_sector, .. }) => *front_sector,
            Line::Portal(PortalLine { front_sector, .. }) => *front_sector,
        }
    }
}

pub struct WallLine {
    #[allow(unused)]
    pub v1: Vec2,
    #[allow(unused)]
    pub v2: Vec2,
    #[allow(unused)]
    pub flags: i16,
    #[allow(unused)]
    pub special: i16,
    #[allow(unused)]
    pub tag: i16,
    #[allow(unused)]
    pub front_sector: Instance<Sector>,
}

pub struct PortalLine {
    #[allow(unused)]
    pub v1: Vec2,
    #[allow(unused)]
    pub v2: Vec2,
    #[allow(unused)]
    pub flags: i16,
    #[allow(unused)]
    pub special: i16,
    #[allow(unused)]
    pub tag: i16,
    #[allow(unused)]
    pub front_sector: Instance<Sector>,
    #[allow(unused)]
    pub back_sector: Instance<Sector>,
}

#[derive(Component, Debug)]
pub enum LineSegment {
    Wall(WallSegment),
    Portal(PortalSegment),
}

impl LineSegment {
    pub fn is_portal(&self) -> bool {
        matches!(self, LineSegment::Portal(..))
    }

    pub fn v1(&self) -> Vec2 {
        match self {
            LineSegment::Wall(WallSegment { v1, .. }) => *v1,
            LineSegment::Portal(PortalSegment { v1, .. }) => *v1,
        }
    }

    pub fn v2(&self) -> Vec2 {
        match self {
            LineSegment::Wall(WallSegment { v2, .. }) => *v2,
            LineSegment::Portal(PortalSegment { v2, .. }) => *v2,
        }
    }

    pub fn normal(&self) -> Dir2 {
        match self {
            LineSegment::Wall(WallSegment { normal, .. }) => *normal,
            LineSegment::Portal(PortalSegment { normal, .. }) => *normal,
        }
    }

    pub fn line(&self) -> Instance<Line> {
        match self {
            LineSegment::Wall(WallSegment { line, .. }) => *line,
            LineSegment::Portal(PortalSegment { line, .. }) => *line,
        }
    }

    pub fn side(&self) -> Instance<SideDef> {
        match self {
            LineSegment::Wall(WallSegment { side, .. }) => *side,
            LineSegment::Portal(PortalSegment { side, .. }) => *side,
        }
    }

    pub fn offset(&self) -> f32 {
        match self {
            LineSegment::Wall(WallSegment { offset, .. }) => *offset,
            LineSegment::Portal(PortalSegment { offset, .. }) => *offset,
        }
    }

    pub fn front_sector(&self) -> Instance<Sector> {
        match self {
            LineSegment::Wall(WallSegment { front_sector, .. }) => *front_sector,
            LineSegment::Portal(PortalSegment { front_sector, .. }) => *front_sector,
        }
    }
}

#[derive(Debug)]
pub struct WallSegment {
    #[allow(unused)]
    pub v1: Vec2,
    #[allow(unused)]
    pub v2: Vec2,
    #[allow(unused)]
    pub normal: Dir2,
    #[allow(unused)]
    pub line: Instance<Line>,
    #[allow(unused)]
    pub side: Instance<SideDef>,
    #[allow(unused)]
    pub offset: f32,
    pub front_sector: Instance<Sector>,
}

#[derive(Debug)]
pub struct PortalSegment {
    #[allow(unused)]
    pub v1: Vec2,
    #[allow(unused)]
    pub v2: Vec2,
    #[allow(unused)]
    pub normal: Dir2,
    #[allow(unused)]
    pub line: Instance<Line>,
    #[allow(unused)]
    pub side: Instance<SideDef>,
    #[allow(unused)]
    pub offset: f32,
    pub front_sector: Instance<Sector>,
    pub back_sector: Instance<Sector>,
}

#[derive(Clone, Debug)]
pub enum BspNode {
    Branch(Rc<BranchNode>),
    Leaf(Instance<SubSector>),
}

#[derive(Clone, Debug)]
pub struct BranchNode {
    /// Origin point of the partition line.
    pub origin: Vec2,
    /// Direction of the partition line (defines the front and back sides).
    pub direction: Vec2,
    /// Right child node in the BSP tree.
    pub right_child: Rc<BspNode>,
    /// Left child node in the BSP tree.
    pub left_child: Rc<BspNode>,
}

impl BranchNode {
    /// Determines if the given `point` lies on the back side
    /// or front side of the partition line.
    fn is_on_back_side(&self, point: Vec2) -> bool {
        // 2D cross product.
        // The function leans toward the back side in cases where
        // the point is exactly on the partition line.
        self.direction.perp_dot(point - self.origin) >= 0.0
    }
}

#[derive(SystemParam)]
pub struct BspTree<'w> {
    level_map: NonSend<'w, LevelMap>,
}

impl BspTree<'_> {
    pub fn iter(&self, view_point: Vec2) -> BspTreeIter {
        let root_node = Rc::clone(&self.level_map.root_node);
        let num_nodes = self.level_map.num_bsp_nodes;
        BspTreeIter::new(root_node, num_nodes, view_point)
    }

    /// Finds the sub-sector that the point belongs to.
    pub fn find_sub_sector(&self, point: Vec2) -> Option<Instance<SubSector>> {
        for node in self.iter(point) {
            if let BspNode::Leaf(sub_sector) = node.as_ref() {
                return Some(*sub_sector);
            }
        }
        None
    }
}

pub struct BspTreeIter {
    node_stack: Vec<Rc<BspNode>>,
    view_point: Vec2,
}

impl BspTreeIter {
    fn new(root_node: Rc<BspNode>, num_nodes: usize, view_point: Vec2) -> Self {
        // Avoid allocating memory while iterating.
        let mut node_stack = Vec::with_capacity(num_nodes);
        node_stack.push(root_node);
        Self {
            node_stack,
            view_point,
        }
    }
}

impl Iterator for BspTreeIter {
    type Item = Rc<BspNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node_stack.pop()?;
        if let BspNode::Branch(branch_node) = node.as_ref() {
            // Rc::clone only increments the reference count, which doesn’t take much time.
            let right_child = Rc::clone(&branch_node.right_child);
            let left_child = Rc::clone(&branch_node.left_child);
            if branch_node.is_on_back_side(self.view_point) {
                // Traverse left child first.
                self.node_stack.push(right_child);
                self.node_stack.push(left_child);
            } else {
                // Traverse right child first.
                self.node_stack.push(left_child);
                self.node_stack.push(right_child);
            };
        }
        Some(node)
    }
}
