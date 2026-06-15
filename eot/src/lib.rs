pub mod core;
pub mod libeot;
pub mod metadata;

mod lzcomp;
mod stream;
mod triplet_encodings;

mod ctf {
    pub(crate) mod sfnt_container;
    pub(crate) mod ctf_parse;
    pub(crate) mod ttf_parse;
}

use core::Error;

use ctf::ctf_parse::parse_ctf;
use metadata::Metadata;
use stream::Stream;

const TTEMBED_TTCOMPRESSED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
const TTEMBED_XORENCRYPTDATA: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;

pub fn eot_to_ttf(data: &[u8]) -> Result<(Metadata, Vec<u8>), Error> {
    let meta = metadata::read_metadata(data)?;
    let font_out = decode(
        &data[meta.font_data_offset as usize..(meta.font_data_offset + meta.font_data_size) as usize],
        meta.flags & TTEMBED_TTCOMPRESSED as u32 != 0,
        meta.flags & TTEMBED_XORENCRYPTDATA as u32 != 0,
    )?;
    Ok((meta, font_out))
}

pub fn decode(data: &[u8], is_compressed: bool, is_encrypted: bool) -> Result<Vec<u8>, Error> {
    const ENCRYPTION_KEY: u8 = 0x50;

    let final_out_buffer: Vec<u8>;

    let mut buf = Vec::from(data);
    if is_encrypted {
        for byte in &mut buf {
            *byte ^= ENCRYPTION_KEY;
        }
    }

    if is_compressed {
        let len = buf.len();
        let mut s_buf = Stream::new(0);
        s_buf.buf = buf;
        let ctfs = lzcomp::unpack_mtx(&mut s_buf, len as _)?;

        let mut streams: [Stream; 3] = [Stream::new2(0, 0), Stream::new2(0, 0), Stream::new2(0, 0)];
        for (stream, ctf) in streams.iter_mut().zip(ctfs.into_iter()) {
            stream.buf = ctf;
        }
        let mut ctr = parse_ctf(&mut streams)?;
        final_out_buffer = ctr.dump_to_vec()?;
    } else {
        final_out_buffer = buf;
    }

    Ok(final_out_buffer)
}
