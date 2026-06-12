pub use crate::core::*;
use crate::EOT;
use crate::writeFontFile;

// pub const EOT_WARN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const TTEMBED_TTCOMPRESSED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TTEMBED_XORENCRYPTDATA: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

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
//                 b"MTX Compression has not yet been implemented in this version of libeot. The font could therefore not be converted.\n\0"
//                     as *const u8 as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//         _ => {
//             fputs(
//                 b"Unknown error: this is a bug in libeot; it does not *necessarily* indicate a corrupted font file.\n\0"
//                     as *const u8 as *const ::core::ffi::c_char,
//                 out,
//             );
//         }
//     };
// }

// pub unsafe extern "C" fn EOT2ttf_file(
//     mut font: *const u8,
//     mut fontSize: ::core::ffi::c_uint,
//     mut metadataOut: *mut EOTMetadata,
//     mut out: *mut FILE,
// ) -> EOTError {
//     let mut result: EOTError = EOTfillMetadata(font, fontSize, metadataOut);
//     if result as ::core::ffi::c_uint >= EOT_WARN as ::core::ffi::c_uint {
//         EOTprintError(result, stderr);
//     } else if result as ::core::ffi::c_uint
//         != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
//     {
//         return result
//     }
//     let mut writeResult: EOTError = writeFontFile(
//         font.offset((*metadataOut).fontDataOffset as isize),
//         (*metadataOut).fontDataSize as ::core::ffi::c_uint,
//         (*metadataOut).flags & TTEMBED_TTCOMPRESSED as u32 != 0,
//         (*metadataOut).flags & TTEMBED_XORENCRYPTDATA as u32 != 0,
//         out,
//     );
//     if writeResult as ::core::ffi::c_uint
//         != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
//     {
//         return writeResult;
//     }
//     return EOT_SUCCESS;
// }

pub unsafe fn EOT2ttf_buffer(data: &[u8]) -> Result<(EOTMetadata, Vec<u8>), Error> {
    let meta = EOT::read_metadata(data)?;
    let fontOut = writeFontFile::writeFontBuffer(
        &data[meta.fontDataOffset as usize..(meta.fontDataOffset + meta.fontDataSize) as usize],
        meta.flags & TTEMBED_TTCOMPRESSED as u32 != 0,
        meta.flags & TTEMBED_XORENCRYPTDATA as u32 != 0,
    )?;
    Ok((meta, fontOut))
}
