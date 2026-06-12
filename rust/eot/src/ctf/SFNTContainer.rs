use std::io::{Read, Cursor};

use crate::util::stream::*;
use crate::core::Error;
use crate::util::stream2::{Error as StreamError, Stream as Stream2};

pub type EOTError = ::core::ffi::c_uint;
pub const EOT_SUCCESS: EOTError = 0;

#[derive(Clone)]
pub struct SFNTTable {
    pub tag: [u8; 4],
    pub buf: Box<[u8]>,
    pub offset: usize,
    pub checksum: u32,
}

impl SFNTTable {
    pub fn new(tag: &[u8; 4]) -> Self {
        Self {
            tag: *tag,
            buf: Box::new([]),
            offset: 0,
            checksum: 0,
        }
    }
}

#[derive(Clone)]
pub struct SFNTContainer {
    pub tables: Vec<SFNTTable>,
}

impl SFNTContainer {
    pub fn new(cap: usize) -> Self {
        Self {
            tables: Vec::with_capacity(cap)
        }
    }

    pub fn add_table(&mut self, tag: &[u8; 4]) -> &mut SFNTTable {
        self.tables.push(SFNTTable::new(tag));
        let l = self.tables.len() - 1;
        &mut self.tables[l]
    }
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();

fn be_read_rest_as_u32(cursor: &mut Cursor<&Box<[u8]>>) -> Result<u32, StreamError> {
    let pos = cursor.position() as usize;
    let len = cursor.get_ref().len();

    if pos >= len {
        return Err(StreamError::NOT_ENOUGH_DATA);
    }

    let remaining = len - pos;
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf[..remaining.min(4)])
        .map_err(|_| StreamError::NOT_ENOUGH_DATA)?;

    Ok(match remaining {
        1 => (buf[0] as u32) << 24,
        2 => (u16::from_be_bytes([buf[0], buf[1]]) as u32) << 16,
        3 => ((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32) << 8,
        _ => u32::from_be_bytes(buf),
    })
}

fn _writeTblCheckingSum(tbl: &mut SFNTTable, out: &mut Stream2) -> Result<(), Error> {
    tbl.checksum = 0;
    tbl.offset = out.pos;

    let mut c = Cursor::new(&tbl.buf);

    loop {
        match be_read_rest_as_u32(&mut c) {
            Ok(chunk) => {
                tbl.checksum = tbl.checksum.wrapping_add(chunk);
                out.be_write_u32(chunk)?;
            },
            Err(StreamError::NOT_ENOUGH_DATA) => break,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn _writeTableDirectory(ctr: &mut SFNTContainer, out: &mut Stream2) -> Result<(), Error> {
    for i in 0..ctr.tables.len() {
        let mut tbl = &mut ctr.tables[i];

        for iTag in 0..4 {
            out.be_write_u8(tbl.tag[iTag])?;
        }

        out.be_write_u32(tbl.checksum)?;
        out.be_write_u32(tbl.offset as u32)?;
        out.be_write_u32(tbl.buf.len() as u32)?;
    }
    Ok(())
}

/* log_2(largest power of 2 <= n) */
fn _lgflr(mut n: u32) -> u32 {
    let mut ret = 0u32;
    while n > 1 {
        n /= 2;
        ret += 1;
    }
    ret
}

/* largest power of 2 <= n */
fn _maxpw(mut n: u32) -> u32 {
    let mut ret = 1u32;
    while n > 1 {
        ret *= 2;
        n /= 2;
    }
    ret
}

fn _writeOffsetTable(ctr: &mut SFNTContainer, s: &mut Stream2) -> Result<(), Error> {
    let mut scalerType: u32 = 0x10000 as u32;
    let mut numTables: u16 = ctr.tables.len() as u16;
    let mut searchRange: u16 = (_maxpw(ctr.tables.len() as u32) * 16) as u16;
    let mut entrySelector: u16 = _lgflr(ctr.tables.len() as u32) as u16;
    let mut rangeShift: u16 = (numTables as i32 * 16i32 - (searchRange as i32)) as u16;

    s.be_write_u32(scalerType)?;
    s.be_write_u16(numTables)?;
    s.be_write_u16(searchRange)?;
    s.be_write_u16(entrySelector)?;
    s.be_write_u16(rangeShift)?;

    Ok(())
}

fn _getTableDirectorySize(ctr: &SFNTContainer) -> usize {
    ctr.tables.len() * 16
}

fn _getRequiredSize(ctr: &SFNTContainer) -> usize {
    let mut ret = 12 + _getTableDirectorySize(ctr);
    for tbl in &ctr.tables {
        ret += ((tbl.buf.len() + 3) / 4) * 4;
    }
    ret
}

pub fn dumpContainer(ctr: &mut SFNTContainer) -> Result<Vec<u8>, Error> {
    let mut s = Stream2::new(_getRequiredSize(ctr));
    _writeOffsetTable(ctr, &mut s)?;

    let tableDirectoryOffset = s.pos;
    s.seek_relative_through_reserve(_getTableDirectorySize(ctr) as isize)?;

    let mut head = None;
    let mut chk = 0u32;
    for i in 0..ctr.tables.len() {
        let tbl = &mut (*ctr).tables[i];
        if &tbl.tag == b"head" {
            head = Some(i);
        }
        tbl.offset = s.pos;
        _writeTblCheckingSum(tbl, &mut s)?;
        chk = chk.wrapping_add(tbl.checksum);
    }
    let Some(head) = head else {
        /* should have already caught the lack of a head table! */
        return Err(Error::LOGIC_ERROR);
    };

    s.seek_absolute(tableDirectoryOffset as usize)?;
    _writeTableDirectory(ctr, &mut s)?;

    let beginning_chk = s.be_checksum32(0, s.pos)?;
    chk = chk.wrapping_add(beginning_chk);

    // now put in the global checksum. It's OK that this will make the head checksum incorrect!
    // this mystical number 0xB1B0AFBA is defined by the TTF standard, dunno why they picked this
    // value.
    let finalChecksum = 0xb1b0afbau32.wrapping_sub(chk);
    s.seek_absolute((ctr.tables[head].offset + 8) as usize)?;
    s.be_write_u32(finalChecksum)?;

    Ok(s.buf)
}

pub unsafe fn loadTableFromStream(
    tbl: &mut SFNTTable,
    mut s: *mut Stream,
) -> Result<(), Error> {
    let mut sResult: StreamResult = seekAbsolute(s, (*tbl).offset as _);
    if sResult != EOT_STREAM_OK {
        return Err(Error::CORRUPT_FILE);
    }
    let slice = std::slice::from_raw_parts((*s).buf, (*s).size as _);
    let end = ((*s).pos as usize) + tbl.buf.len();
    if end > slice.len() {
        return Err(Error::CORRUPT_FILE);
    }
    tbl.buf[..].copy_from_slice(&slice[(*s).pos as _..end]);
    Ok(())
}
