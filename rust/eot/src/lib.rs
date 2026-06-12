#![allow(dangerous_implicit_autorefs)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod core;
pub mod EOT;
pub mod ctf {
    pub mod SFNTContainer;
    pub mod parseCTF;
    pub mod parseTTF;
} // mod ctf
// pub mod eot2ttf;
pub mod libeot;
pub mod lzcomp {
    pub mod ahuff;
    pub mod bitio;
    pub mod liblzcomp;
    pub mod lzcomp;
    pub mod mtxmem;
} // mod lzcomp
pub mod triplet_encodings;
pub mod util {
    pub mod stream;
    pub mod stream2;
} // mod util
pub mod writeFontFile;
