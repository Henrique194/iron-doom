use crate::map_object::prelude::Camera;
use crate::map_object::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use moonshine_kind::{Instance, SpawnInstance};
use std::f32::consts;
use wad::prelude::*;

pub trait SpawnMapObjects {
    fn spawn_map_objects(&mut self, map: &Map) -> Vec<Instance<MapObject>>;
}

impl SpawnMapObjects for Commands<'_, '_> {
    fn spawn_map_objects(&mut self, map: &Map) -> Vec<Instance<MapObject>> {
        let mut map_objs = Vec::with_capacity(map.things.len());
        for thing in &map.things {
            if thing.thing_type >= 2 && thing.thing_type <= 4 {
                // Skip net players.
                continue;
            }

            let map_obj = create_map_object(thing);
            let mut instance_cmds = self.spawn_instance(map_obj);
            if thing.thing_type == 1 {
                instance_cmds.insert((
                    Player,
                    Camera::new(map_obj),
                    InputManagerBundle::with_map(PlayerAction::default_map()),
                ));
            }
            let instance = instance_cmds.instance();

            map_objs.push(instance);
        }
        map_objs
    }
}

fn create_map_object(thing: &MapThing) -> MapObject {
    let angle = consts::FRAC_PI_4 * (thing.angle / 45) as f32;
    MapObject {
        pos: Vec2::new(thing.x.into(), thing.y.into()),
        velocity: Vec2::ZERO,
        dir: Rot2::radians(angle) * Dir2::X,
        thing_type: thing.thing_type,
        options: thing.options,
    }
}
