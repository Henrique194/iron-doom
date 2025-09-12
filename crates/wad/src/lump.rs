use crate::header::WadHeader;
use crate::util::{bytes_to_i32, bytes_to_str};
use anyhow::{bail, Result};
use std::slice::SliceIndex;

#[derive(Debug)]
pub struct Lump<'a> {
    data: &'a [u8],
    name: String,
}

impl Lump<'_> {
    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn is_marker(&self) -> bool {
        self.size() == 0
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

struct LumpParser;

impl LumpParser {
    fn parse<'a>(lump_data: &[u8], wad_data: &'a [u8]) -> Result<Lump<'a>> {
        let position_bytes = &lump_data[0..4];
        let position = LumpParser::parse_position(position_bytes)?;

        let size_bytes = &lump_data[4..8];
        let size = LumpParser::parse_size(size_bytes)?;

        let name_bytes = &lump_data[8..16];
        let name = LumpParser::parse_name(name_bytes)?;

        let start_offset = position;
        let end_offset = start_offset + size;
        let wad_size = wad_data.len();
        if start_offset >= wad_size {
            bail!("Lump {} has invalid pointer to lump data", name);
        }
        if end_offset > wad_size {
            bail!("Lump {} size exceeds WAD file size", name);
        }
        let data = &wad_data[start_offset..end_offset];

        Ok(Lump { data, name })
    }

    fn parse_position(position_bytes: &[u8]) -> Result<usize> {
        let position = bytes_to_i32(position_bytes)?;
        if position < 0 {
            bail!("Directory entry has invalid lump position");
        }
        Ok(position as usize)
    }

    fn parse_size(size_bytes: &[u8]) -> Result<usize> {
        let size = bytes_to_i32(size_bytes)?;
        if size < 0 {
            bail!("Directory entry has invalid lump size");
        }
        Ok(size as usize)
    }

    fn parse_name(name_bytes: &[u8]) -> Result<String> {
        let name = bytes_to_str(name_bytes)?;
        Ok(name.to_owned())
    }
}

#[derive(Debug)]
pub struct LumpsDirectory<'a> {
    lumps: Vec<Lump<'a>>,
}

impl<'a> LumpsDirectory<'a> {
    /// Get a lump by name.
    pub fn get(&self, lump_name: &str) -> Option<&Lump> {
        let index = self.get_index_of(lump_name)?;
        self.lumps.get(index)
    }

    /// Get a lump by index.
    pub fn get_index<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[Lump<'a>]>,
    {
        self.lumps.get(index)
    }

    /// Get the index of the lump named `lump_name`.
    pub fn get_index_of(&self, lump_name: &str) -> Option<usize> {
        for (i, lump) in self.lumps.iter().enumerate() {
            if lump.name.eq_ignore_ascii_case(lump_name) {
                return Some(i);
            }
        }
        None
    }
}

pub struct LumpsDirectoryParser;

impl LumpsDirectoryParser {
    pub fn parse(wad_data: &[u8], header: WadHeader) -> Result<LumpsDirectory> {
        let start_offset = header.dir_offset;
        let end_offset = start_offset + (16 * header.num_lumps);
        let dir_data = &wad_data[start_offset..end_offset];

        let mut lumps = Vec::with_capacity(header.num_lumps);
        for lump_data in dir_data.chunks_exact(16) {
            let lump = LumpParser::parse(lump_data, wad_data)?;
            lumps.push(lump);
        }

        Ok(LumpsDirectory { lumps })
    }
}
