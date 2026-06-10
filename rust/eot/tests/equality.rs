use eot::libeot::{EOT2ttf_buffer, EOTfreeBuffer, EOTMetadata};

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
    ];

    for inp in files {
        let data = std::fs::read(inp).unwrap();

        let mut n: u32 = 0;
        let slice;
        let meta: EOTMetadata;
        let mut out: *mut u8;

        unsafe {
            out = std::ptr::null_mut();
            meta = EOT2ttf_buffer(&data, &mut out, &mut n).unwrap();
            _ = meta;
            assert!(!out.is_null());
            slice = std::slice::from_raw_parts(out, n as usize);
        }

        let face = ttf_parser::Face::parse(slice, 0).expect("ttf::Face::parse failed");
        println!("ttf_parser OK: {} glyphs, italic={}, weight={}",
            face.number_of_glyphs(), face.is_italic(), face.weight().to_number());

        let c_out_file = tempfile::NamedTempFile::new().unwrap();
        let c_out_path = c_out_file.path();
        std::process::Command::new("../../eot2ttf")
            .args(&[inp, &c_out_path.to_string_lossy()])
            .output()
            .unwrap();

        let c_out_data = std::fs::read(c_out_path).unwrap();
        assert_eq!(c_out_data, slice);

        unsafe {
            EOTfreeBuffer(out);
        }
    }
}
