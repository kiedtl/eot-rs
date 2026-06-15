use std::io::Cursor;

use byteorder::{BE, ReadBytesExt};

use crate::{core::*, ctf::SFNTContainer::SFNTTable};

#[derive(Copy, Clone)]
pub struct TtfHeadData {
    pub index_to_loc_format: i16,
}

#[derive(Copy, Clone, Default)]
pub struct TtfMaxpData {
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_component_points: u16,
    pub max_component_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

pub fn ttf_parse_head(tbl: &SFNTTable) -> Result<TtfHeadData, Error> {
    if tbl.buf.len() < 52 {
        return Err(Error::CORRUPT_FILE);
    }
    let index_to_loc_format = i16::from_be_bytes([tbl.buf[50], tbl.buf[51]]);
    Ok(TtfHeadData { index_to_loc_format })
}

pub fn ttf_parse_maxp(tbl: &SFNTTable) -> Result<TtfMaxpData, Error> {
    let mut out = TtfMaxpData::default();

    let mut c = Cursor::new(&tbl.buf);
    let version = c.read_u32::<BE>().map_err(|_| Error::CORRUPT_FILE)?;

    let mut ru16 = || -> Result<u16, Error> { c.read_u16::<BE>().map_err(|_| Error::CORRUPT_FILE) };

    out.num_glyphs = ru16()?;
    if version == 0x00010000 {
        out.max_points = ru16()?;
        out.max_contours = ru16()?;
        out.max_component_points = ru16()?;
        out.max_component_contours = ru16()?;
        out.max_zones = ru16()?;
        out.max_twilight_points = ru16()?;
        out.max_storage = ru16()?;
        out.max_function_defs = ru16()?;
        out.max_instruction_defs = ru16()?;
        out.max_stack_elements = ru16()?;
        out.max_size_of_instructions = ru16()?;
        out.max_component_elements = ru16()?;
        out.max_component_depth = ru16()?;
    }

    Ok(out)
}
