mod utils;

#[cfg(feature = "image")]
use image::{Luma, Rgb};
use qrcode::render::svg;
use qrcode::*;
#[cfg(feature = "image")]
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn svg(s: &str, level: u32, size: u32, light: &str, dark: &str) -> String {
    let lv = match level {
        3 => EcLevel::H,
        2 => EcLevel::Q,
        1 => EcLevel::M,
        _ => EcLevel::L,
    };
    let code = QrCode::with_version(s.as_bytes(), Version::Normal(10), lv).unwrap();
    let image = code
        .render()
        .module_dimensions(size, size)
        .quiet_zone(false)
        .dark_color(svg::Color(dark))
        .light_color(svg::Color(light))
        .build();
    // println!("{}", image);
    image
}

#[cfg(feature = "image")]
#[wasm_bindgen]
pub fn png(s: &str, level: u32, size: u32, light: &str, dark: &str) -> Vec<u8> {
    let lv = match level {
        3 => EcLevel::H,
        2 => EcLevel::Q,
        1 => EcLevel::M,
        _ => EcLevel::L,
    };
    let code = QrCode::with_version(s.as_bytes(), Version::Normal(10), lv).unwrap();
    let image = code
        .render::<Luma<u8>>()
        .module_dimensions(size, size)
        .quiet_zone(false)
        // .dark_color(Rgb(dark))
        // .light_color(Rgb(dark))
        .build();

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    image
        .write_to(&mut writer, image::ImageFormat::Png)
        .unwrap();
    buffer
}

#[cfg(feature = "image")]
#[wasm_bindgen]
pub fn webp(s: &str, level: u32, size: u32, light: &str, dark: &str) -> Vec<u8> {
    let lv = match level {
        3 => EcLevel::H,
        2 => EcLevel::Q,
        1 => EcLevel::M,
        _ => EcLevel::L,
    };
    let code = QrCode::with_version(s.as_bytes(), Version::Normal(10), lv).unwrap();
    let image = code
        .render::<Luma<u8>>()
        .module_dimensions(size, size)
        .quiet_zone(false)
        // .dark_color(Rgb(dark))
        // .light_color(Rgb(dark))
        .build();

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    image
        .write_to(&mut writer, image::ImageFormat::WebP)
        .unwrap();
    buffer
}
