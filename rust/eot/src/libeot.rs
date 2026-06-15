pub use crate::core::*;
use crate::{EOT, writeFontFile};

const TTEMBED_TTCOMPRESSED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
const TTEMBED_XORENCRYPTDATA: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

pub fn eot2ttf_buffer(data: &[u8]) -> Result<(EOTMetadata, Vec<u8>), Error> {
    let meta = EOT::read_metadata(data)?;
    let font_out = writeFontFile::write_font_buffer(
        &data[meta.font_data_offset as usize..(meta.font_data_offset + meta.font_data_size) as usize],
        meta.flags & TTEMBED_TTCOMPRESSED as u32 != 0,
        meta.flags & TTEMBED_XORENCRYPTDATA as u32 != 0,
    )?;
    Ok((meta, font_out))
}
