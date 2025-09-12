use crate::lump::LumpsDirectory;
use crate::util::bytes_to_i16;
use anyhow::{bail, Result};
use derive_more::{Deref, DerefMut, IntoIterator};

#[derive(Deref, DerefMut, IntoIterator, Debug)]
#[into_iterator(owned, ref, ref_mut)]
pub struct MapVertexes(Vec<MapVertex>);

pub struct MapVertexesParser;

impl MapVertexesParser {
    pub fn parse(lumps_dir: &LumpsDirectory, map_lump: usize) -> Result<MapVertexes> {
        let vertexes_lump = map_lump + 4;
        let Some(lump) = lumps_dir.get_index(vertexes_lump) else {
            bail!("Missing Vertexes Lump for Map Lump #{map_lump}");
        };

        let mut vertexes = Vec::with_capacity(lump.size() / 4);

        for vertex_data in lump.data().chunks_exact(4) {
            let x = bytes_to_i16(&vertex_data[0..=1])?;
            let y = bytes_to_i16(&vertex_data[2..=3])?;
            vertexes.push(MapVertex { x, y })
        }

        Ok(MapVertexes(vertexes))
    }
}

#[derive(Debug)]
pub struct MapVertex {
    #[allow(unused)]
    pub x: i16,
    #[allow(unused)]
    pub y: i16,
}
