#![allow(dangerous_implicit_autorefs)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

pub mod core;
pub mod EOT;
pub mod ctf {
    pub mod SFNTContainer;
    pub mod parseCTF;
    pub mod parseTTF;
}
pub mod libeot;

pub mod writeFontFile;

mod stream;
mod triplet_encodings;
mod lzcomp {
    pub mod ahuff;
    pub mod bitio;
    pub mod lzcomp;
}
