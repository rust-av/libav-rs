extern crate bindgen;
extern crate metadeps;

use std::fs::File;
use std::io::Write;

fn format_write(builder: bindgen::Builder, output: &str) {
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = File::create(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(deprecated)]")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn main() {
    let libs = metadeps::probe().unwrap();

    for e in ["avcodec", "avformat"].iter() {
        let headers = libs.get(&format!("lib{}", e)).unwrap().include_paths.clone();

        let mut builder = common_builder().header(format!("data/lib{}.h", e));

        for header in headers.iter() {
            builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
        }

        // Manually fix the comment so rustdoc won't try to pick them
        format_write(builder, &format!("src/{}.rs", e));
    }
}
