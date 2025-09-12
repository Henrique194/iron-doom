use crate::lump::LumpsDirectory;
use crate::util::{bytes_to_i16, bytes_to_str};
use anyhow::{bail, Result};
use derive_more::{Deref, DerefMut, IntoIterator};

#[derive(Deref, DerefMut, IntoIterator, Debug)]
#[into_iterator(owned, ref, ref_mut)]
pub struct MapSectors(Vec<MapSector>);

pub struct MapSectorsParser;

impl MapSectorsParser {
    pub fn parse(lumps_dir: &LumpsDirectory, map_lump: usize) -> Result<MapSectors> {
        let sectors_lump = map_lump + 8;
        let Some(lump) = lumps_dir.get_index(sectors_lump) else {
            bail!("Missing Sector Lump for Map Lump #{map_lump}");
        };

        let mut sectors = Vec::with_capacity(lump.size() / 26);

        for sector_data in lump.data().chunks_exact(26) {
            let floor_height = bytes_to_i16(&sector_data[0..=1])?;
            let ceiling_height = bytes_to_i16(&sector_data[2..=3])?;
            let floor_tex = bytes_to_str(&sector_data[4..=11])?;
            let ceiling_tex = bytes_to_str(&sector_data[12..=19])?;
            let light_level = bytes_to_i16(&sector_data[20..=21])?;
            let special = bytes_to_i16(&sector_data[22..=23])?;
            let tag = bytes_to_i16(&sector_data[24..=25])?;
            sectors.push(MapSector {
                floor_height,
                ceiling_height,
                floor_tex: floor_tex.to_owned(),
                ceiling_tex: ceiling_tex.to_owned(),
                light_level,
                special,
                tag,
            })
        }

        Ok(MapSectors(sectors))
    }
}

#[derive(Debug)]
pub struct MapSector {
    #[allow(unused)]
    pub floor_height: i16,
    #[allow(unused)]
    pub ceiling_height: i16,
    #[allow(unused)]
    pub floor_tex: String,
    #[allow(unused)]
    pub ceiling_tex: String,
    #[allow(unused)]
    pub light_level: i16,
    #[allow(unused)]
    pub special: i16,
    #[allow(unused)]
    pub tag: i16,
}
