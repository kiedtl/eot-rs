use eot::libeot::EOT2ttf_buffer;

#[test]
fn checks() {
    let files = [
        "../testdata/font1.fntdata",
        "../testdata/font2.fntdata",
        "../testdata/font3.fntdata",
        "../testdata/font4.fntdata",
        "../testdata/Lato-bold.fntdata",
        "../testdata/Lato-boldItalic.fntdata",
        "../testdata/Lato-italic.fntdata",
        "../testdata/Lato-regular.fntdata",
        "../testdata/Raleway-bold.fntdata",
        "../testdata/Raleway-boldItalic.fntdata",
        "../testdata/Raleway-italic.fntdata",
        "../testdata/Raleway-regular.fntdata",
        "../testdata/7/compressed.eot",
        "../testdata/7/uncompressed.eot",
        "../testdata/4/font1.eot",
    ];

    for inp in files {
        let data = std::fs::read(inp).unwrap();

        let out;

        unsafe {
            (_, out) = EOT2ttf_buffer(&data).unwrap();
        }

        let face = ttf_parser::Face::parse(&out, 0).expect("ttf::Face::parse failed");
        println!("ttf_parser OK: {} glyphs, italic={}, weight={}",
            face.number_of_glyphs(), face.is_italic(), face.weight().to_number());

        let c_out_file = tempfile::NamedTempFile::new().unwrap();
        let c_out_path = c_out_file.path();
        std::process::Command::new("/home/kiedtl/src/libeot/eot2ttf")
            .args(&[inp, &c_out_path.to_string_lossy()])
            .output()
            .unwrap();

        let c_out_data = std::fs::read(c_out_path).unwrap();
        assert_eq!(&c_out_data, &out);
    }
}
