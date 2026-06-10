fn main() {
    //println!("cargo:rerun-if-changed=legacy");

    cc::Build::new()
        .cargo_warnings(false) // Silence bullshit
        .file("../../src/EOT.c")
        .file("../../src/ctf/SFNTContainer.c")
        .file("../../src/ctf/parseCTF.c")
        .file("../../src/ctf/parseTTF.c")
        .file("../../src/eot2ttf.c")
        .file("../../src/libeot.c")
        .file("../../src/lzcomp/ahuff.c")
        .file("../../src/lzcomp/bitio.c")
        .file("../../src/lzcomp/liblzcomp.c")
        .file("../../src/lzcomp/lzcomp.c")
        .file("../../src/lzcomp/mtxmem.c")
        .file("../../src/triplet_encodings.c")
        .file("../../src/util/stream.c")
        .file("../../src/writeFontFile.c")
        .include("../../src/")
        .include("../../inc/")
        .define("DECOMPRESS_ON", Some(""))
        .compile("libeot_legacy");
}
