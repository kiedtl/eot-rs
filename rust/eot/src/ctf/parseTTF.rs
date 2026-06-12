use std::io::Cursor;
use byteorder::{BE, ReadBytesExt};
use crate::core::*;
use crate::ctf::SFNTContainer::SFNTTable;

#[derive(Copy, Clone)]
pub struct TTFheadData {
    pub indexToLocFormat: i16,
}

#[derive(Copy, Clone, Default)]
pub struct TTFmaxpData {
    pub numGlyphs: u16,
    pub maxPoints: u16,
    pub maxContours: u16,
    pub maxComponentPoints: u16,
    pub maxComponentContours: u16,
    pub maxZones: u16,
    pub maxTwilightPoints: u16,
    pub maxStorage: u16,
    pub maxFunctionDefs: u16,
    pub maxInstructionDefs: u16,
    pub maxStackElements: u16,
    pub maxSizeOfInstructions: u16,
    pub maxComponentElements: u16,
    pub maxComponentDepth: u16,
}

pub fn TTFParseHead(tbl: &SFNTTable) -> Result<TTFheadData, Error> {
    if tbl.buf.len() < 52 {
        return Err(Error::CORRUPT_FILE);
    }
    let indexToLocFormat = i16::from_be_bytes([tbl.buf[50], tbl.buf[51]]);
    Ok(TTFheadData { indexToLocFormat })
}

pub fn TTFParseMaxp(tbl: &SFNTTable) -> Result<TTFmaxpData, Error> {
    let mut out = TTFmaxpData::default();

    let mut c = Cursor::new(&tbl.buf);
    let version = c.read_u32::<BE>().map_err(|_| Error::CORRUPT_FILE)?;

    let mut ru16 = || -> Result<u16, Error> {
        c.read_u16::<BE>().map_err(|_| Error::CORRUPT_FILE)
    };

    out.numGlyphs = ru16()?;
    if version == 0x00010000 {
        out.maxPoints = ru16()?;
        out.maxContours = ru16()?;
        out.maxComponentPoints = ru16()?;
        out.maxComponentContours = ru16()?;
        out.maxZones = ru16()?;
        out.maxTwilightPoints = ru16()?;
        out.maxStorage = ru16()?;
        out.maxFunctionDefs = ru16()?;
        out.maxInstructionDefs = ru16()?;
        out.maxStackElements = ru16()?;
        out.maxSizeOfInstructions = ru16()?;
        out.maxComponentElements = ru16()?;
        out.maxComponentDepth = ru16()?;
    }

    Ok(out)
}
