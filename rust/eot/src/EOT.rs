use crate::core::*;

extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type EOTError = ::core::ffi::c_uint;
pub const EOT_WARN_NOT_ENOUGH_GLYPHS: EOTError = 1002;
pub const EOT_WARN_BAD_VERSION: EOTError = 1001;
pub const EOT_WARN_NOT_ENOUGH_SPACE_RESERVED: EOTError = 1000;
pub const EOT_MALFORMED_HEAD_TABLE: EOTError = 19;
pub const EOT_MTX_ERROR: EOTError = 18;
pub const EOT_UNKNOWN_BUFFER_WRITE_ERROR: EOTError = 17;
pub const EOT_CORRUPT_HOPCODE_DATA: EOTError = 16;
pub const EOT_NO_HDMX_TABLE: EOTError = 15;
pub const EOT_NO_HMTX_TABLE: EOTError = 14;
pub const EOT_NO_HEAD_TABLE: EOTError = 13;
pub const EOT_NO_MAXP_TABLE: EOTError = 12;
pub const EOT_LOGIC_ERROR: EOTError = 11;
pub const EOT_COMPRESSION_NOT_YET_IMPLEMENTED: EOTError = 10;
pub const EOT_FWRITE_ERROR: EOTError = 9;
pub const EOT_OTHER_STDLIB_ERROR: EOTError = 8;
pub const EOT_CANT_ALLOCATE_MEMORY: EOTError = 7;
pub const EOT_THIRD_STREAM_INCOMPLETE: EOTError = 6;
pub const EOT_SECOND_STREAM_INCOMPLETE: EOTError = 5;
pub const EOT_CORRUPT_FILE: EOTError = 4;
pub const EOT_BOGUS_STRING_SIZE: EOTError = 3;
pub const EOT_HEADER_TOO_BIG: EOTError = 2;
pub const EOT_INSUFFICIENT_BYTES: EOTError = 1;
pub const EOT_SUCCESS: EOTError = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut EDITING_MASK: uint16_t = 0x8 as uint16_t;

pub unsafe extern "C" fn EOTreadU32LE(mut bytes: *const uint8_t) -> uint32_t {
    return *bytes.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*bytes.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*bytes.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*bytes.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
}

fn read_u32_le(bytes: &[u8]) -> Result<u32, Error> {
    if let Some(&[a, b, c, d]) = bytes.get(..4) {
        Ok(u32::from_le_bytes([a, b, c, d]))
    } else {
        Err(Error::INSUFFICIENT_BYTES)
    }
}

pub unsafe extern "C" fn EOTreadU16LE(mut bytes: *const uint8_t) -> uint16_t {
    return (*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t
        as ::core::ffi::c_int
        | (*bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
}

pub unsafe extern "C" fn EOTgetMetadataLength(
    mut bytes: *const uint8_t,
) -> ::core::ffi::c_uint {
    let mut totalLength: uint32_t = EOTreadU32LE(bytes);
    let mut fontLength: uint32_t = EOTreadU32LE(
        bytes.offset(4 as ::core::ffi::c_int as isize),
    );
    return (totalLength as ::core::ffi::c_uint)
        .wrapping_sub(fontLength as ::core::ffi::c_uint);
}

pub fn EOTgetMetadataLength2(bytes: &[u8]) -> Result<usize, Error> {
    if bytes.len() < 8 {
        return Err(Error::INSUFFICIENT_BYTES);
    }
    let total_length = read_u32_le(bytes)?;
    let font_length = read_u32_le(&bytes[4..])?;
    if let Some(diff) = total_length.checked_sub(font_length) {
        Ok(diff as usize)
    } else {
        Err(Error::CORRUPT_FILE)
    }
}

pub unsafe fn EOTgetString2(bytes: &[u8], scanner: &mut usize) -> Result<Vec<uint16_t>, Error> {
    if *scanner + 2 > bytes.len() {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    let size = EOTreadU16LE((&bytes[*scanner..]).as_ptr()) as usize;
    *scanner += 2;

    if size % 2 != 0 {
        return Err(Error::BOGUS_STRING_SIZE);
    }

    if *scanner + size > bytes.len() {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    let mut buf = Vec::with_capacity(size / 2);
    for _ in 0..size / 2 {
        buf.push(EOTreadU16LE((&bytes[*scanner..]).as_ptr()));
        *scanner += 2;
    }

    Ok(buf)
}

pub unsafe fn EOTgetByteArray2(bytes: &[u8], scanner: &mut usize,) -> Result<Vec<u8>, Error> {
    if *scanner + 4 > bytes.len() {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    let size = read_u32_le(&bytes[*scanner..])? as usize;
    *scanner += 4;

    if *scanner + size > bytes.len() {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    let mut buf = Vec::with_capacity(size);
    for _ in 0..size {
        buf.push(bytes[*scanner]);
        *scanner += 1;
    }

    Ok(buf)
}

pub unsafe fn EOTfillMetadataSpecifyingVersion(
    bytes: &[u8],
    out: &mut EOTMetadata,
    version: EOTVersion,
    currIndex: ::core::ffi::c_int,
) -> Result<(), Error> {
    out.version = version;

    let mut scanner = 0;

    macro_rules! ensure {
        ($val:expr) => {
            if scanner + $val >= bytes.len() {
                return Err(Error::INSUFFICIENT_BYTES);
            }
        }
    }

    ensure!(4);
    out.flags = read_u32_le(&bytes[scanner..])?;
    scanner += 4;

    ensure!(10);
    memcpy(
        &raw mut (*out).panose as *mut ::core::ffi::c_void,
        (&bytes[scanner..]).as_ptr() as *const ::core::ffi::c_void,
        10 as size_t,
    );
    scanner += 10;

    ensure!(1);
    out.charset = bytes[scanner] as EOTCharset;
    scanner += 1;

    ensure!(1);
    out.italic = bytes[scanner] != 0;
    scanner += 1;

    ensure!(4);
    out.weight = read_u32_le(&bytes[scanner..])?;
    scanner += 4;

    ensure!(2);
    out.permissions = EOTreadU16LE((&bytes[scanner..]).as_ptr());
    scanner += 2;

    ensure!(2);
    if EOTreadU16LE((&bytes[scanner..]).as_ptr()) != 0x504c {
        return Err(Error::CORRUPT_FILE);
    }
    scanner += 2;

    for i in 0..4 {
        ensure!(4);
        out.unicodeRange[i] = read_u32_le(&bytes[scanner..])?;
        scanner += 4;
    }

    for i in 0..2 {
        ensure!(4);
        out.codePageRange[i] = read_u32_le(&bytes[scanner..])?;
        scanner += 4;
    }

    ensure!(4);
    out.checkSumAdjustment = read_u32_le(&bytes[scanner..])?;
    scanner += 22;

    out.familyName = EOTgetString2(bytes, &mut scanner)?;
    scanner += 2;
    out.styleName = EOTgetString2(bytes, &mut scanner)?;
    scanner += 2;
    out.versionName = EOTgetString2(bytes, &mut scanner)?;
    scanner += 2;
    out.fullName = EOTgetString2(bytes, &mut scanner)?;

    if out.version > VERSION_1 {
        scanner += 2;
        out.do_not_use = EOTgetString2(bytes, &mut scanner)?;

        if out.version == VERSION_3 {
            ensure!(4);
            _ = read_u32_le(&bytes[scanner..]);
            scanner += 4;

            ensure!(4);
            out.eudcInfo.codePage = read_u32_le(&bytes[scanner..])?;
            scanner += 6;

            ensure!(2);
            let mut signatureSize: uint16_t = EOTreadU16LE((&bytes[scanner..]).as_ptr());
            scanner += 2;

            ensure!(signatureSize as usize);
            scanner += signatureSize as usize;
            // signature is reserved, so do nothing with this.

            ensure!(4);
            out.eudcInfo.flags = read_u32_le(&bytes[scanner..])?;
            scanner += 4;

            out.eudcInfo.fontData = EOTgetByteArray2(bytes, &mut scanner)?;
            out.eudcInfo.exists = out.eudcInfo.fontData.len() > 0;
        }
    }

    out.fontDataOffset = scanner as u32 + currIndex as u32;
    let expected_header_size = out.totalSize.wrapping_sub(out.fontDataSize);
    if out.fontDataOffset < expected_header_size {
        return Err(Error::HEADER_TOO_BIG);
    }
    Ok(())
}

pub unsafe fn EOTfillMetadata(bytes: &[u8]) -> Result<EOTMetadata, Error> {
    let len = bytes.len();
    let mut scanner = 0;

    macro_rules! ensure {
        ($val:expr) => {
            if scanner + $val >= bytes.len() {
                return Err(Error::INSUFFICIENT_BYTES);
            }
        }
    }

    if len < EOTgetMetadataLength2(bytes)? {
        return Err(Error::INSUFFICIENT_BYTES);
    }

    ensure!(4);
    let total_size = read_u32_le(&bytes[scanner..])?;
    scanner += 4;

    ensure!(4);
    let font_data_size = read_u32_le(&bytes[scanner..])?;
    scanner += 4;

    ensure!(4);
    let coded_version = match read_u32_le(&bytes[scanner..])? {
        65536 => VERSION_1,
        131073 => VERSION_2,
        131074 => VERSION_3,
        _ => return Err(Error::CORRUPT_FILE),
    };
    scanner += 4;

    let mut tryVersion = coded_version;
    let mut bumpedUp = false;
    let mut knockedDown = false;

    loop {
        let mut met = EOTMetadata::ZERO;
        met.totalSize = total_size as uint32_t;
        met.fontDataSize = font_data_size as uint32_t;

        if bytes.len() < met.fontDataSize as usize + scanner {
            return Err(Error::CORRUPT_FILE);
        }

        let sub = &bytes[scanner..bytes.len() - met.fontDataSize as usize];
        match EOTfillMetadataSpecifyingVersion(sub, &mut met, tryVersion, scanner as i32) {
            Ok(()) => {
                if tryVersion == coded_version {
                    return Ok(met);
                } else {
                    return Err(Error::WARN_BAD_VERSION);
                }
            },
            Err(Error::HEADER_TOO_BIG) => {
                if knockedDown || tryVersion == VERSION_3 {
                    return Err(Error::CORRUPT_FILE);
                }
                knockedDown = false;
                bumpedUp = true;
                tryVersion += 1;
            },
            Err(Error::INSUFFICIENT_BYTES) => {
                if bumpedUp || tryVersion == VERSION_1 {
                    return Err(Error::CORRUPT_FILE);
                }
                knockedDown = true;
                bumpedUp = false;
                tryVersion -= 1;
            },
            Err(e) => return Err(e),
        }
    };
}

pub unsafe extern "C" fn EOTcanLegallyEdit(mut metadata: *const EOTMetadata) -> bool {
    return (*metadata).permissions as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*metadata).permissions as ::core::ffi::c_int
            & EDITING_MASK as ::core::ffi::c_int != 0 as ::core::ffi::c_int;
}
