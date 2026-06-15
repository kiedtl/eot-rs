use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LE, ReadBytesExt};

use crate::core::*;

fn skip(c: &mut Cursor<&[u8]>, amount: u16) -> Result<(), Error> {
    c.seek(SeekFrom::Current(amount as i64)).map_err(|_| Error::INSUFFICIENT_BYTES)?;
    Ok(())
}

fn skip_padding(c: &mut Cursor<&[u8]>, amount: u16) -> Result<(), Error> {
    for _ in 0..(amount as usize) {
        if c.read_u8().map_err(|_| Error::INSUFFICIENT_BYTES)? != 0 {
            return Err(Error::CORRUPT_FILE_PADDING_NOT_ZERO);
        }
    }
    Ok(())
}

fn read_u32_le2(c: &mut Cursor<&[u8]>) -> Result<u32, Error> {
    c.read_u32::<LE>().map_err(|_| Error::INSUFFICIENT_BYTES)
}

fn read_u16_le2(c: &mut Cursor<&[u8]>) -> Result<u16, Error> {
    c.read_u16::<LE>().map_err(|_| Error::INSUFFICIENT_BYTES)
}

/// Returns (total_length, metadata_length, font_length)
fn read_metadata_length(c: &mut Cursor<&[u8]>) -> Result<(u32, u32, u32), Error> {
    let total_length = read_u32_le2(c)?;
    let font_length = read_u32_le2(c)?;
    if let Some(diff) = total_length.checked_sub(font_length) {
        Ok((total_length, diff, font_length))
    } else {
        Err(Error::CORRUPT_FILE)
    }
}

fn read_u16_array(c: &mut Cursor<&[u8]>) -> Result<Vec<u16>, Error> {
    let size = read_u16_le2(c)? as usize;

    if !size.is_multiple_of(2) {
        return Err(Error::BOGUS_STRING_SIZE);
    }

    let mut buf = Vec::with_capacity(size / 2);
    for _ in 0..size / 2 {
        buf.push(read_u16_le2(c)?);
    }

    Ok(buf)
}

fn read_byte_array(c: &mut Cursor<&[u8]>) -> Result<Vec<u8>, Error> {
    let size = read_u32_le2(c)? as usize;

    let mut buf = Vec::with_capacity(size);
    for _ in 0..size {
        buf.push(c.read_u8().map_err(|_| Error::INSUFFICIENT_BYTES)?);
    }

    Ok(buf)
}

fn read_metadata_with_version(
    c: &mut Cursor<&[u8]>, meta: &mut EOTMetadata, version: EOTVersion,
) -> Result<(), Error> {
    meta.version = version;

    meta.flags = read_u32_le2(c)?;
    c.read_exact(&mut meta.panose).map_err(|_| Error::INSUFFICIENT_BYTES)?;
    meta.charset = c.read_u8().map_err(|_| Error::INSUFFICIENT_BYTES)? as u32;
    meta.italic = c.read_u8().map_err(|_| Error::INSUFFICIENT_BYTES)? != 0;
    meta.weight = read_u32_le2(c)?;
    meta.permissions = read_u16_le2(c)?;

    if read_u16_le2(c)? != 0x504c {
        return Err(Error::CORRUPT_FILE);
    }

    for i in 0..4 {
        meta.unicodeRange[i] = read_u32_le2(c)?;
    }

    for i in 0..2 {
        meta.codePageRange[i] = read_u32_le2(c)?;
    }

    meta.checkSumAdjustment = read_u32_le2(c)?;
    skip(c, 16)?; // Reserved

    skip_padding(c, 2)?;
    meta.familyName = read_u16_array(c)?;

    skip_padding(c, 2)?;
    meta.styleName = read_u16_array(c)?;

    skip_padding(c, 2)?;
    meta.versionName = read_u16_array(c)?;

    skip_padding(c, 2)?;
    meta.fullName = read_u16_array(c)?;

    if meta.version > VERSION_1 {
        skip_padding(c, 2)?;
        meta.do_not_use = read_u16_array(c)?;

        if meta.version == VERSION_3 {
            skip(c, 4)?; // RootStringChecksum: unused
            meta.eudcInfo.codePage = read_u32_le2(c)?;

            skip_padding(c, 2)?;

            // Signature is reserved and not used (must be zeroed), so do nothing with this.
            let signatureSize = read_u16_le2(c)?;
            skip_padding(c, signatureSize)?;

            meta.eudcInfo.flags = read_u32_le2(c)?;
            meta.eudcInfo.fontData = read_byte_array(c)?;
            meta.eudcInfo.exists = !meta.eudcInfo.fontData.is_empty();
        }
    }

    // The cursor spans the whole file, so its position is already the absolute
    // offset of the font data; no base offset needs to be added here.
    meta.fontDataOffset = c.position() as u32;
    let expected_header_size = meta.totalSize.wrapping_sub(meta.fontDataSize);
    if meta.fontDataOffset < expected_header_size {
        return Err(Error::HEADER_TOO_BIG);
    }

    Ok(())
}

pub fn read_metadata(bytes: &[u8]) -> Result<EOTMetadata, Error> {
    let mut c = Cursor::new(bytes);
    let (total_size, metadata_size, font_data_size) = read_metadata_length(&mut c)?;

    if bytes.len() < metadata_size as usize {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    let coded_version = match read_u32_le2(&mut c)? {
        0x00010000 => VERSION_1,
        0x00020001 => VERSION_2,
        0x00020002 => VERSION_3,
        _ => return Err(Error::CORRUPT_FILE),
    };

    let mut tryVersion = coded_version;
    let mut bumpedUp = false;
    let mut knockedDown = false;

    loop {
        let mut met = EOTMetadata::ZERO;
        met.totalSize = total_size;
        met.fontDataSize = font_data_size;
        let pos = c.position() as usize;

        if bytes.len() < met.fontDataSize as usize + pos {
            return Err(Error::CORRUPT_FILE);
        }

        match read_metadata_with_version(&mut c, &mut met, tryVersion) {
            Ok(()) =>
                if tryVersion == coded_version {
                    return Ok(met);
                } else {
                    return Err(Error::WARN_BAD_VERSION);
                },
            Err(Error::HEADER_TOO_BIG) => {
                if knockedDown || tryVersion == VERSION_3 {
                    return Err(Error::CORRUPT_FILE);
                }
                knockedDown = false;
                bumpedUp = true;
                tryVersion += 1;
            }
            Err(Error::INSUFFICIENT_BYTES) => {
                if bumpedUp || tryVersion == VERSION_1 {
                    return Err(Error::CORRUPT_FILE);
                }
                knockedDown = true;
                bumpedUp = false;
                tryVersion -= 1;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Please think twice before circumventing this function. Does your personal sense of morality
/// really let you take others' work without their permission?
///
/// I'm not suggesting any system of morality is right or wrong; I'm merely asking that you reflect
/// on it before changing anything here.
pub fn can_legally_edit(metadata: &EOTMetadata) -> bool {
    const EDITING_MASK: u16 = 0x8;
    metadata.permissions == 0 || ((metadata.permissions & EDITING_MASK) != 0)
}
