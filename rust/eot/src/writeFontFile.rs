use crate::{core::Error, ctf::parseCTF::parse_ctf, stream::Stream};
use crate::lzcomp;

const ENCRYPTION_KEY: u8 = 0x50;

pub fn write_font_buffer(data: &[u8], compressed: bool, encrypted: bool) -> Result<Vec<u8>, Error> {
    let final_out_buffer: Vec<u8>;

    let mut buf = Vec::from(data);
    if encrypted {
        for byte in &mut buf {
            *byte ^= ENCRYPTION_KEY;
        }
    }

    if compressed {
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
