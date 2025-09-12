use crate::util::{bytes_to_i16, bytes_to_i32, bytes_to_str};
use anyhow::bail;
use bevy::prelude::Deref;

#[derive(Deref)]
pub struct WallTextureDefinitions(pub Vec<WallTextureDefinition>);

pub struct WallTextureDefinitionsParser;

impl WallTextureDefinitionsParser {
    pub fn parse(lump_data: &[u8]) -> anyhow::Result<WallTextureDefinitions> {
        let num_texture_bytes = &lump_data[0..4];
        let num_textures = Self::parse_num_textures(num_texture_bytes)?;

        let texture_defs = Self::parse_texture_definitions(lump_data, num_textures)?;

        Ok(WallTextureDefinitions(texture_defs))
    }

    fn parse_num_textures(bytes: &[u8]) -> anyhow::Result<usize> {
        let num_textures = bytes_to_i32(bytes)?;
        if num_textures < 0 {
            bail!("Texture definitions has invalid number of textures");
        }
        Ok(num_textures as usize)
    }

    fn parse_texture_definitions(
        lump_data: &[u8],
        num_textures: usize,
    ) -> anyhow::Result<Vec<WallTextureDefinition>> {
        let mut texture_defs = Vec::with_capacity(num_textures);
        let max_offset = lump_data.len();
        let offsets_bytes = &lump_data[4..];
        for offset_bytes in offsets_bytes.chunks_exact(4).take(num_textures) {
            let offset = bytes_to_i32(offset_bytes)?;
            if offset < 0 || (offset as usize) > max_offset {
                bail!("Texture definitions has invalid texture offset");
            }
            let offset = offset as usize;
            let texture_def_bytes = &lump_data[offset..];
            let texture_def = WallTextureDefinitionParser::parse(texture_def_bytes)?;
            texture_defs.push(texture_def);
        }

        Ok(texture_defs)
    }
}

pub struct WallTextureDefinition {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub patch_descriptors: Vec<PatchDescriptor>,
}

struct WallTextureDefinitionParser;

impl WallTextureDefinitionParser {
    fn parse(lump_data: &[u8]) -> anyhow::Result<WallTextureDefinition> {
        let texture_name_bytes = &lump_data[0..8];
        let name = Self::parse_name(texture_name_bytes)?;

        let width_bytes = &lump_data[12..14];
        let width = Self::parse_width(width_bytes)?;

        let height_bytes = &lump_data[14..16];
        let height = Self::parse_height(height_bytes)?;

        let num_patches_bytes = &lump_data[20..22];
        let num_descriptors = Self::parse_num_descriptors(num_patches_bytes)?;

        let patch_descriptors_bytes = &lump_data[22..];
        let patch_descriptors =
            Self::parse_patch_descriptors(patch_descriptors_bytes, num_descriptors)?;

        Ok(WallTextureDefinition {
            name,
            width,
            height,
            patch_descriptors,
        })
    }

    fn parse_name(bytes: &[u8]) -> anyhow::Result<String> {
        let name = bytes_to_str(bytes)?;
        Ok(name.to_owned())
    }

    fn parse_width(bytes: &[u8]) -> anyhow::Result<usize> {
        let width = bytes_to_i16(bytes)?;
        if width < 0 {
            bail!("Texture has invalid width");
        }
        Ok(width as usize)
    }

    fn parse_height(bytes: &[u8]) -> anyhow::Result<usize> {
        let height = bytes_to_i16(bytes)?;
        if height < 0 {
            bail!("Texture has invalid height");
        }
        Ok(height as usize)
    }

    fn parse_num_descriptors(bytes: &[u8]) -> anyhow::Result<usize> {
        let num_patches = bytes_to_i16(bytes)?;
        if num_patches < 0 {
            bail!("Texture has invalid number of patches");
        }
        Ok(num_patches as usize)
    }

    fn parse_patch_descriptors(
        bytes: &[u8],
        num_descriptors: usize,
    ) -> anyhow::Result<Vec<PatchDescriptor>> {
        let mut patch_descriptors = Vec::with_capacity(num_descriptors);
        for patch_descriptor_bytes in bytes.chunks_exact(10).take(num_descriptors) {
            let patch_descriptor = PatchDescriptorParser::parse(patch_descriptor_bytes)?;
            patch_descriptors.push(patch_descriptor);
        }
        Ok(patch_descriptors)
    }
}

pub struct PatchDescriptor {
    /// Horizontal offset of the patch relative to the upper-left of the texture.
    pub origin_x: i16,
    /// Vertical offset of the patch relative to the upper-left of the texture.
    pub origin_y: i16,
    /// Patch number (as listed in PNAMES) to draw.
    pub patch_num: usize,
}

struct PatchDescriptorParser;

impl PatchDescriptorParser {
    fn parse(data: &[u8]) -> anyhow::Result<PatchDescriptor> {
        let origin_x_bytes = &data[0..2];
        let origin_x = bytes_to_i16(origin_x_bytes)?;

        let origin_y_bytes = &data[2..4];
        let origin_y = bytes_to_i16(origin_y_bytes)?;

        let patch_num_bytes = &data[4..6];
        let patch_num = Self::parse_patch_num(patch_num_bytes)?;

        Ok(PatchDescriptor {
            origin_x,
            origin_y,
            patch_num,
        })
    }

    fn parse_patch_num(bytes: &[u8]) -> anyhow::Result<usize> {
        let patch_num = bytes_to_i16(bytes)?;
        if patch_num < 0 {
            bail!("Missing patch in texture");
        }
        Ok(patch_num as usize)
    }
}
