pub mod EOT;
pub mod core;
pub mod libeot;
pub mod writeFontFile;

mod stream;
mod triplet_encodings;
mod lzcomp;

pub mod ctf {
    pub mod SFNTContainer;
    pub mod parseCTF;
    pub mod parseTTF;
}
