use crate::components::{Sector, SideDef};
use crate::resources::LevelMap;
use anyhow::{bail, Result};
use bevy::prelude::*;
use moonshine_kind::{Instance, SpawnInstance};
use wad::prelude::*;

pub trait SpawnLinesSides {
    fn spawn_lines_sides(
        &mut self,
        map: &Map,
        level_map: &LevelMap,
        wad: &WadFile,
    ) -> Result<Vec<Instance<SideDef>>>;
}

impl SpawnLinesSides for Commands<'_, '_> {
    fn spawn_lines_sides(
        &mut self,
        map: &Map,
        level_map: &LevelMap,
        wad: &WadFile,
    ) -> Result<Vec<Instance<SideDef>>> {
        let mut sides = Vec::with_capacity(map.side_defs.len());
        for side in &map.side_defs {
            let side = create_line_side(side, level_map, wad)?;
            let instance = self.spawn_instance(side).instance();
            sides.push(instance);
        }
        Ok(sides)
    }
}

fn create_line_side(side: &MapSideDef, level_map: &LevelMap, wad: &WadFile) -> Result<SideDef> {
    let sector = get_sector(side, level_map)?;
    let top_texture = get_texture(&side.top_texture, wad)?;
    let lower_texture = get_texture(&side.lower_texture, wad)?;
    let middle_texture = get_texture(&side.middle_texture, wad)?;

    Ok(SideDef {
        x_offset: side.x_offset.into(),
        y_offset: side.y_offset.into(),
        top_texture,
        lower_texture,
        middle_texture,
        sector,
    })
}

fn get_sector(side: &MapSideDef, level_map: &LevelMap) -> Result<Instance<Sector>> {
    let sector = usize::try_from(side.sector)?;
    let Some(sector) = level_map.sectors.get(sector) else {
        bail!("Line side references invalid sector.");
    };
    Ok(*sector)
}

fn get_texture(texture: &str, wad: &WadFile) -> Result<usize> {
    let textures = wad.wall_textures();
    let Some(texture) = textures.get_index_of(texture) else {
        bail!("Line side references invalid texture.");
    };
    Ok(texture)
}
