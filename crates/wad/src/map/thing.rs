use crate::lump::LumpsDirectory;
use crate::util::bytes_to_i16;
use anyhow::{bail, Result};
use derive_more::{Deref, DerefMut, IntoIterator};

#[derive(Deref, DerefMut, IntoIterator, Debug)]
#[into_iterator(owned, ref, ref_mut)]
pub struct MapThings(Vec<MapThing>);

pub struct MapThingsParser;

impl MapThingsParser {
    pub fn parse(lumps_dir: &LumpsDirectory, map_lump: usize) -> Result<MapThings> {
        let things_lump = map_lump + 1;
        let Some(lump) = lumps_dir.get_index(things_lump) else {
            bail!("Missing Things Lump for Map Lump #{map_lump}");
        };

        let mut things = Vec::with_capacity(lump.size() / 10);

        for thing_data in lump.data().chunks_exact(10) {
            let x = bytes_to_i16(&thing_data[0..=1])?;
            let y = bytes_to_i16(&thing_data[2..=3])?;
            let angle = bytes_to_i16(&thing_data[4..=5])?;
            let thing_type = bytes_to_i16(&thing_data[6..=7])?;
            let options = bytes_to_i16(&thing_data[8..=9])?;
            things.push(MapThing {
                x,
                y,
                angle,
                thing_type,
                options,
            })
        }

        Ok(MapThings(things))
    }
}

/// Things represent players, monsters, pick-ups, and projectiles.
/// They also represent obstacles, certain decorations, player start
/// positions and teleport landing sites.
#[derive(Debug)]
pub struct MapThing {
    /// x position
    pub x: i16,

    /// y position
    pub y: i16,

    /// Thing angles represent arc degrees increasing counterclockwise from
    /// the east. The possible values are:
    ///
    /// |  Compass point | Degrees |
    /// |----------------|---------|
    /// | East           | 0°      |
    /// | North East     | 45°     |
    /// | North          | 90°     |
    /// | North West     | 135°    |
    /// | West           | 180°    |
    /// | South West     | 225°    |
    /// | South          | 270°    |
    /// | South East     | 315°    |
    pub angle: i16,

    /// DoomEd thing type
    pub thing_type: i16,

    /// Flags
    pub options: i16,
}
