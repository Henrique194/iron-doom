use crate::lump::LumpsDirectory;
use anyhow::{bail, Result};
use bevy::utils::HashMap;
use block_map::{MapBlockMap, MapBlockMapParser};
use derive_more::{Deref, DerefMut};
use line::{MapLines, MapLinesParser};
use node::MapNodesParser;
use reject::{MapRejectMatrix, MapRejectMatrixParser};
use sector::{MapSectors, MapSectorsParser};
use segment::{MapSegments, MapSegmentsParser};
use side::{MapSideDefs, MapSideDefsParser};
use sub_sector::{MapSubSectors, MapSubSectorsParser};
use thing::{MapThings, MapThingsParser};
use vertex::{MapVertexes, MapVertexesParser};

pub use line::MapLine;
pub use node::{MapNode, MapNodes};
pub use sector::MapSector;
pub use segment::MapSegment;
pub use side::MapSideDef;
pub use sub_sector::MapSubSector;
pub use thing::MapThing;
pub use vertex::MapVertex;

mod block_map;
mod line;
mod node;
mod reject;
mod sector;
mod segment;
mod side;
mod sub_sector;
mod thing;
mod vertex;

#[derive(Deref, DerefMut, Debug)]
pub struct Maps(HashMap<String, Map>);

impl Maps {
    pub fn map(&self, episode: usize, map: usize) -> Option<&Map> {
        let name = format!("E{episode}M{map}");
        self.get(&name)
    }
}

pub struct MapsParser;

impl MapsParser {
    pub fn parse(lumps_dir: &LumpsDirectory) -> Result<Maps> {
        let mut maps = HashMap::new();

        for episode in 1..=4 {
            for map in 1..=9 {
                let map_name = format!("E{episode}M{map}");
                let Ok(map) = MapParser::parse(lumps_dir, &map_name) else {
                    break;
                };
                maps.insert(map_name, map);
            }
        }

        Ok(Maps(maps))
    }
}

#[derive(Debug)]
pub struct Map {
    #[allow(unused)]
    pub things: MapThings,
    #[allow(unused)]
    pub lines: MapLines,
    #[allow(unused)]
    pub side_defs: MapSideDefs,
    #[allow(unused)]
    pub vertexes: MapVertexes,
    #[allow(unused)]
    pub segments: MapSegments,
    #[allow(unused)]
    pub sub_sectors: MapSubSectors,
    #[allow(unused)]
    pub nodes: MapNodes,
    #[allow(unused)]
    pub sectors: MapSectors,
    #[allow(unused)]
    reject_matrix: MapRejectMatrix,
    #[allow(unused)]
    block_map: MapBlockMap,
}

pub struct MapParser;

impl MapParser {
    pub fn parse(lumps_dir: &LumpsDirectory, map_name: &str) -> Result<Map> {
        let Some(lump) = lumps_dir.get_index_of(map_name) else {
            bail!("Missing Map {map_name}");
        };
        let things = MapThingsParser::parse(lumps_dir, lump)?;
        let lines = MapLinesParser::parse(lumps_dir, lump)?;
        let side_defs = MapSideDefsParser::parse(lumps_dir, lump)?;
        let vertexes = MapVertexesParser::parse(lumps_dir, lump)?;
        let segments = MapSegmentsParser::parse(lumps_dir, lump)?;
        let sub_sectors = MapSubSectorsParser::parse(lumps_dir, lump)?;
        let nodes = MapNodesParser::parse(lumps_dir, lump)?;
        let sectors = MapSectorsParser::parse(lumps_dir, lump)?;
        let reject_matrix = MapRejectMatrixParser::parse(lumps_dir, lump)?;
        let block_map = MapBlockMapParser::parse(lumps_dir, lump)?;
        Ok(Map {
            things,
            lines,
            side_defs,
            vertexes,
            segments,
            sub_sectors,
            nodes,
            sectors,
            reject_matrix,
            block_map,
        })
    }
}
