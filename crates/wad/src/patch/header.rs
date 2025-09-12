use crate::util::{bytes_to_i16, bytes_to_i32};
use anyhow::{bail, Result};

pub struct PatchHeader {
    pub width: usize,
    pub height: usize,
    pub left_offset: i16,
    pub top_offset: i16,
    pub column_offsets: Vec<usize>,
}

pub struct PatchHeaderParser;

impl PatchHeaderParser {
    pub fn parse(lump_data: &[u8]) -> Result<PatchHeader> {
        let width_bytes = &lump_data[0..2];
        let width = Self::parse_width(width_bytes)?;

        let height_bytes = &lump_data[2..4];
        let height = Self::parse_height(height_bytes)?;

        let left_ofs_bytes = &lump_data[4..6];
        let left_offset = bytes_to_i16(left_ofs_bytes)?;

        let top_ofs_bytes = &lump_data[6..8];
        let top_offset = bytes_to_i16(top_ofs_bytes)?;

        let offsets_bytes = &lump_data[8..];
        let column_offsets = Self::parse_offsets(offsets_bytes, width)?;

        Ok(PatchHeader {
            width,
            height,
            left_offset,
            top_offset,
            column_offsets,
        })
    }

    fn parse_width(bytes: &[u8]) -> Result<usize> {
        let width = bytes_to_i16(bytes)?;
        if width < 0 {
            bail!("Patch has invalid width");
        }
        Ok(width as usize)
    }

    fn parse_height(bytes: &[u8]) -> Result<usize> {
        let height = bytes_to_i16(bytes)?;
        if height < 0 {
            bail!("Patch has invalid height");
        }
        Ok(height as usize)
    }

    fn parse_offsets(bytes: &[u8], width: usize) -> Result<Vec<usize>> {
        let mut offsets = Vec::with_capacity(width);
        for ofs_bytes in bytes.chunks_exact(4).take(width) {
            let offset = bytes_to_i32(ofs_bytes)?;
            if offset < 0 {
                bail!("Patch header has invalid column offset");
            }
            offsets.push(offset as usize);
        }
        Ok(offsets)
    }
}
