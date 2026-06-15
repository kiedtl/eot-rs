pub use crate::core::*;
use crate::{EOT, writeFontFile};

const TTEMBED_TTCOMPRESSED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
const TTEMBED_XORENCRYPTDATA: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

// pub unsafe extern "C" fn EOTprintError(mut error: EOTError, mut out: *mut FILE) {
//     match error as ::core::ffi::c_uint {
//         0 => {}
//         1 => {
//             fputs(
//                 b"The font file appears truncated.\n\0" as *const u8
//                     as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//         3 | 4 => {
//             fputs(
//                 b"The font file appears corrupt.\n\0" as *const u8
//                     as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//         7 => {
//             fputs(
//                 b"Couldn't allocate sufficient memory.\n\0" as *const u8
//                     as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//         8 => {
//             fputs(
//                 b"There was an unknown system error.\n\0" as *const u8
//                     as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//         10 => {
//             fputs(
//                 b"MTX Compression has not yet been implemented in this version of libeot. The
// font could therefore not be converted.\n\0"                     as *const u8 as *const
// ::core::ffi::c_char,                 out,
//             );
//         }
//         _ => {
//             fputs(
//                 b"Unknown error: this is a bug in libeot; it does not *necessarily* indicate a
// corrupted font file.\n\0"                     as *const u8 as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//     };
// }

pub fn EOT2ttf_buffer(data: &[u8]) -> Result<(EOTMetadata, Vec<u8>), Error> {
    let meta = EOT::read_metadata(data)?;
    let fontOut = writeFontFile::writeFontBuffer(
        &data[meta.fontDataOffset as usize..(meta.fontDataOffset + meta.fontDataSize) as usize],
        meta.flags & TTEMBED_TTCOMPRESSED as u32 != 0,
        meta.flags & TTEMBED_XORENCRYPTDATA as u32 != 0,
    )?;
    Ok((meta, fontOut))
}
