mod utils;

use base64::prelude::*;
use image::Rgba;
use qrcode::render::svg;
use qrcode::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    let a = if hex.len() == 6 {
        0xff
    } else {
        u8::from_str_radix(&hex[6..8], 16).unwrap()
    };

    Rgba([r, g, b, a])
}

#[wasm_bindgen]
pub fn gen(
    s: &str,
    img: &str,
    level: u32,
    size: u32,
    quiet: bool,
    light: &str,
    dark: &str,
) -> String {
    let lv = match level {
        3 => EcLevel::H,
        2 => EcLevel::Q,
        1 => EcLevel::M,
        _ => EcLevel::L,
    };
    for version_num in 1..=40 {
        let version = Version::Normal(version_num);
        let code = QrCode::with_version(s.as_bytes(), version, lv);
        match code {
            Ok(code) => {
                let mut ext = "";

                let base64_string = match img {
                    ".png" | ".jpg" | ".jpeg" | ".jpe" | ".jif" | ".jfif" | ".gif" | ".webp"
                    | ".pbm" | ".pgm" | ".ppm" | ".tiff" | ".tif" | ".tga" | ".dds" | ".bmp"
                    | ".ico" | ".hdr" | ".exr" | ".ff" | ".avif" | ".qoi" => {
                        let mut buffer: Vec<u8> = Vec::new();
                        let mut writer = Cursor::new(&mut buffer);

                        code
                            .render::<Rgba<u8>>()
                            .quiet_zone(quiet)
                            .min_dimensions(size, size)
                            .dark_color(hex_to_rgba(dark))
                            .light_color(hex_to_rgba(light))
                            .build()
                            .write_to(
                                &mut writer,
                                match img {
                                    ".jpg" | ".jpeg" | ".jpe" | ".jif" | ".jfif" => {
                                        ext = ".jpeg";
                                        image::ImageFormat::Jpeg
                                    }
                                    ".gif" => {
                                        ext = ".gif";
                                        image::ImageFormat::Gif
                                    }
                                    ".webp" => {
                                        ext = ".webp";
                                        image::ImageFormat::WebP
                                    }
                                    ".pbm" | ".pgm" | ".ppm" => {
                                        ext = ".pbm";
                                        image::ImageFormat::Pnm
                                    }
                                    ".tiff" | ".tif" => {
                                        ext = ".tiff";
                                        image::ImageFormat::Tiff
                                    }
                                    ".tga" => {
                                        ext = ".tga";
                                        image::ImageFormat::Tga
                                    }
                                    ".dds" => {
                                        ext = ".dds";
                                        image::ImageFormat::Dds
                                    }
                                    ".bmp" => {
                                        ext = ".bmp";
                                        image::ImageFormat::Bmp
                                    }
                                    ".ico" => {
                                        ext = ".ico";
                                        image::ImageFormat::Ico
                                    }
                                    ".hdr" => {
                                        ext = ".hdr";
                                        image::ImageFormat::Hdr
                                    }
                                    ".exr" => {
                                        ext = ".exr";
                                        image::ImageFormat::OpenExr
                                    }
                                    ".ff" => {
                                        ext = ".ff";
                                        image::ImageFormat::Farbfeld
                                    }
                                    ".avif" => {
                                        ext = ".avif";
                                        image::ImageFormat::Avif
                                    }
                                    ".qoi" => {
                                        ext = ".qoi";
                                        image::ImageFormat::Qoi
                                    }
                                    _ | ".png" => {
                                        ext = ".png";
                                        image::ImageFormat::Png
                                    }
                                },
                            )
                            .unwrap();

                        BASE64_STANDARD.encode(buffer)
                    }
                    _ | ".svg" => {
                        ext = ".svg";
                        let res = code
                            .render()
                            .quiet_zone(quiet)
                            .min_dimensions(size, size)
                            // .module_dimensions(module_size, module_size)
                            .dark_color(svg::Color(dark))
                            .light_color(svg::Color(light))
                            .build();
                        BASE64_STANDARD.encode(&res)
                    }
                };

                // println!("{}", base64_string);
                return [
                    r#"{"ver":"#,
                    &version_num.to_string(),
                    r#","ext":""#,
                    ext,
                    r#"","width":"#,
                    &version.width().to_string(),
                    r#","img":""#,
                    &base64_string,
                    r#""}"#,
                ]
                .join("");
            }
            Err(_) => {}
        }
    }
    return r#"{"err":"Data too long"}"#.to_owned();
}
