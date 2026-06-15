fn main() {
    //println!("cargo:rerun-if-changed=legacy");

    cc::Build::new()
        .cargo_warnings(false) // Silence bullshit
        .file("../legacy/libeot/src/EOT.c")
        .file("../legacy/libeot/src/ctf/SFNTContainer.c")
        .file("../legacy/libeot/src/ctf/parseCTF.c")
        .file("../legacy/libeot/src/ctf/parseTTF.c")
        .file("../legacy/libeot/src/eot2ttf.c")
        .file("../legacy/libeot/src/libeot.c")
        .file("../legacy/libeot/src/lzcomp/ahuff.c")
        .file("../legacy/libeot/src/lzcomp/bitio.c")
        .file("../legacy/libeot/src/lzcomp/liblzcomp.c")
        .file("../legacy/libeot/src/lzcomp/lzcomp.c")
        .file("../legacy/libeot/src/lzcomp/mtxmem.c")
        .file("../legacy/libeot/src/triplet_encodings.c")
        .file("../legacy/libeot/src/util/stream.c")
        .file("../legacy/libeot/src/writeFontFile.c")
        .include("../legacy/libeot/src/")
        .include("../legacy/libeot/inc/")
        .define("DECOMPRESS_ON", Some(""))
        .compile("libeot_legacy");
}
