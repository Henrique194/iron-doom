use anyhow::{Error, Result};
use bevy::prelude::*;
use cli::CommandLine;
use directory::IwadDirs;
use exit::macros::sys_fail;
use flat::{Flats, FlatsParser};
use header::WadHeaderParser;
use lump::LumpsDirectoryParser;
use map::Maps;
use map::MapsParser;
use palette::{Palette, Palettes, PalettesParser};
use patch::PatchParser;
use prelude::*;
use std::{fs::File, io::Read};
use wall_texture::{WallTextures, WallTexturesParser};

pub mod prelude {
    pub use super::{
        map::{
            Map, MapLine, MapNode, MapNodes, MapSector, MapSegment, MapSideDef, MapSubSector,
            MapThing, MapVertex,
        },
        palette::PaletteVariant,
        patch::{DrawPath, Patch},
        wall_texture::{WallTexture, WallTextures},
        WadFile,
    };
}

mod directory;
mod flat;
mod header;
mod index_map;
mod lump;
mod map;
mod palette;
mod patch;
mod sys;
mod util;
mod wall_texture;

#[derive(Default)]
pub struct WadPlugin;

impl Plugin for WadPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WadLoadState>()
            .add_event::<WadLoaded>()
            .add_systems(Startup, load_iwad)
            .add_systems(OnEnter(WadLoadState::Loaded), finished_loading);
    }
}

#[sys_fail]
fn load_iwad(cli: Res<CommandLine>, mut commands: Commands) {
    let dirs = IwadDirs::try_new()?;
    let iwad_path = if let Some(name) = &cli.iwad {
        dirs.find_iwad(name)
    } else {
        dirs.search_iwads()
    };
    let Some(iwad_path) = iwad_path else {
        let error = Error::msg(
            "Game mode indeterminate. No IWAD file was found. Try\n\
            specifying one with the '--iwad' command line parameter.\n",
        );
        return Err(error);
    };

    debug!("Loading IWAD: {iwad_path:?}");
    let mut wad_handle = File::open(iwad_path)?;
    let mut wad_data = Vec::new();
    wad_handle.read_to_end(&mut wad_data)?;
    let wad_file = WadFileParser::parse(&wad_data)?;

    commands.insert_resource(wad_file);
    commands.set_state(WadLoadState::Loaded);
}

fn finished_loading(mut loaded_events: EventWriter<WadLoaded>) {
    debug!("Finished Loading IWAD");
    loaded_events.send_default();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum WadLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Event, Debug, Default)]
pub struct WadLoaded;

#[derive(Resource, Debug)]
pub struct WadFile {
    maps: Maps,
    flats: Flats,
    wall_textures: WallTextures,
    palettes: Palettes,
    title_screen: Patch,
}

impl WadFile {
    #[must_use]
    pub fn get_palette(&self, palette: PaletteVariant) -> &Palette {
        let pal_num: usize = palette.into();
        &self.palettes[pal_num]
    }

    pub fn title_screen(&self) -> &Patch {
        &self.title_screen
    }

    pub fn flats(&self) -> &Flats {
        &self.flats
    }

    pub fn wall_textures(&self) -> &WallTextures {
        &self.wall_textures
    }

    pub fn map(&self, episode: usize, map: usize) -> Option<&Map> {
        self.maps.map(episode, map)
    }
}

struct WadFileParser;

impl WadFileParser {
    fn parse(wad_data: &[u8]) -> Result<WadFile> {
        let wad_header = WadHeaderParser::parse(wad_data)?;
        let lumps_dir = LumpsDirectoryParser::parse(wad_data, wad_header)?;

        let maps = MapsParser::parse(&lumps_dir)?;
        let flats = FlatsParser::parse(&lumps_dir)?;
        let wall_textures = WallTexturesParser::parse(&lumps_dir)?;
        let palettes = PalettesParser::parse(&lumps_dir)?;

        let lump = lumps_dir.get("TITLEPIC").unwrap();
        let title_screen = PatchParser::parse(lump.data())?;

        Ok(WadFile {
            maps,
            flats,
            wall_textures,
            palettes,
            title_screen,
        })
    }
}
