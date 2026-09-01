#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use image::{ImageBuffer, Pixel, Rgba};
use std::path::Path;

#[tauri::command]
fn adjust_brightness(input_path: String, output_path: String, factor: f32) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgba8();

    let adjusted = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b, a] = pixel.0;
        let r = ((r as f32 * factor).min(255.0).max(0.0)) as u8;
        let g = ((g as f32 * factor).min(255.0).max(0.0)) as u8;
        let b = ((b as f32 * factor).min(255.0).max(0.0)) as u8;
        Rgba([r, g, b, a])
    });

    adjusted
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Brightness adjusted: {}", output_path))
}

#[tauri::command]
fn adjust_contrast(input_path: String, output_path: String, factor: f32) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgba8();

    let adjusted = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b, a] = pixel.0;
        let r = (((r as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0)) as u8;
        let g = (((g as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0)) as u8;
        let b = (((b as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0)) as u8;
        Rgba([r, g, b, a])
    });

    adjusted
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Contrast adjusted: {}", output_path))
}

#[tauri::command]
fn adjust_saturation(input_path: String, output_path: String, factor: f32) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgb8();

    let adjusted = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b] = pixel.0;

        let max = r.max(g).max(b) as f32;
        let min = r.min(g).min(b) as f32;
        let mut l = (max + min) / 2.0;

        let mut h = 0.0;
        let mut s = 0.0;

        if max != min {
            let d = max - min;
            s = if l > 128.0 {
                d / (510.0 - max - min)
            } else {
                d / (max + min)
            };

            h = match max as u8 {
                _ if max as u8 == r => (g as f32 - b as f32) / d + if g as f32 < b as f32 { 6.0 } else { 0.0 },
                _ if max as u8 == g => (b as f32 - r as f32) / d + 2.0,
                _ => (r as f32 - g as f32) / d + 4.0,
            };
            h /= 6.0;
        }

        s = (s * factor).min(1.0).max(0.0);
        l /= 255.0;

        let hsl_to_rgb = |c: f32| {
            let c = if c < 0.0 { c + 1.0 } else if c > 1.0 { c - 1.0 } else { c };
            if c < 1.0 / 6.0 {
                l + (1.0 - l) * s * c * 6.0
            } else if c < 1.0 / 2.0 {
                l + (1.0 - l) * s
            } else if c < 2.0 / 3.0 {
                l + (1.0 - l) * s * (2.0 / 3.0 - c) * 6.0
            } else {
                l
            }
        };

        let r_new = ((hsl_to_rgb(h + 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;
        let g_new = ((hsl_to_rgb(h) * 255.0).min(255.0).max(0.0)) as u8;
        let b_new = ((hsl_to_rgb(h - 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;

        image::Rgb([r_new, g_new, b_new])
    });

    adjusted
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Saturation adjusted: {}", output_path))
}

#[tauri::command]
fn convert_grayscale(input_path: String, output_path: String) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?;

    let gray = image::DynamicImage::ImageRgba8(img.to_rgba8())
        .grayscale();

    gray.save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Image converted to grayscale: {}", output_path))
}

#[tauri::command]
fn apply_sepia(input_path: String, output_path: String) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgba8();

    let sepia = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b, a] = pixel.0;
        let r = r as f32;
        let g = g as f32;
        let b = b as f32;

        let sr = ((r * 0.393) + (g * 0.769) + (b * 0.189)).min(255.0) as u8;
        let sg = ((r * 0.349) + (g * 0.686) + (b * 0.168)).min(255.0) as u8;
        let sb = ((r * 0.272) + (g * 0.534) + (b * 0.131)).min(255.0) as u8;

        Rgba([sr, sg, sb, a])
    });

    sepia
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Sepia filter applied: {}", output_path))
}

#[tauri::command]
fn shift_hue(input_path: String, output_path: String, degrees: f32) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgb8();

    let shift = degrees / 360.0;

    let adjusted = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b] = pixel.0;

        let max = r.max(g).max(b) as f32;
        let min = r.min(g).min(b) as f32;
        let l = (max + min) / 2.0;

        let mut h = 0.0;
        let mut s = 0.0;

        if max != min {
            let d = max - min;
            s = if l > 128.0 {
                d / (510.0 - max - min)
            } else {
                d / (max + min)
            };

            h = match max as u8 {
                _ if max as u8 == r => (g as f32 - b as f32) / d + if g as f32 < b as f32 { 6.0 } else { 0.0 },
                _ if max as u8 == g => (b as f32 - r as f32) / d + 2.0,
                _ => (r as f32 - g as f32) / d + 4.0,
            };
            h /= 6.0;
        }

        h = (h + shift).fract();
        if h < 0.0 {
            h += 1.0;
        }

        let hsl_to_rgb = |c: f32| {
            let c = if c < 0.0 { c + 1.0 } else if c > 1.0 { c - 1.0 } else { c };
            if c < 1.0 / 6.0 {
                l / 255.0 + ((1.0 - l / 255.0) * s) * c * 6.0
            } else if c < 1.0 / 2.0 {
                l / 255.0 + ((1.0 - l / 255.0) * s)
            } else if c < 2.0 / 3.0 {
                l / 255.0 + ((1.0 - l / 255.0) * s) * (2.0 / 3.0 - c) * 6.0
            } else {
                l / 255.0
            }
        };

        let r_new = ((hsl_to_rgb(h + 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;
        let g_new = ((hsl_to_rgb(h) * 255.0).min(255.0).max(0.0)) as u8;
        let b_new = ((hsl_to_rgb(h - 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;

        image::Rgb([r_new, g_new, b_new])
    });

    adjusted
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Hue shifted by {} degrees: {}", degrees, output_path))
}

#[tauri::command]
fn colorize_grayscale(input_path: String, output_path: String, hue: f32) -> Result<String, String> {
    let img = image::open(&input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .to_rgb8();

    let colorized = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        let [r, g, b] = pixel.0;
        let gray = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0;

        let h = hue / 360.0;
        let s = 1.0;
        let l = gray;

        let hsl_to_rgb = |c: f32| {
            let c = if c < 0.0 { c + 1.0 } else if c > 1.0 { c - 1.0 } else { c };
            if c < 1.0 / 6.0 {
                l + (1.0 - l) * s * c * 6.0
            } else if c < 1.0 / 2.0 {
                l + (1.0 - l) * s
            } else if c < 2.0 / 3.0 {
                l + (1.0 - l) * s * (2.0 / 3.0 - c) * 6.0
            } else {
                l
            }
        };

        let r_new = ((hsl_to_rgb(h + 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;
        let g_new = ((hsl_to_rgb(h) * 255.0).min(255.0).max(0.0)) as u8;
        let b_new = ((hsl_to_rgb(h - 1.0 / 3.0) * 255.0).min(255.0).max(0.0)) as u8;

        image::Rgb([r_new, g_new, b_new])
    });

    colorized
        .save(&output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format!("Image colorized: {}", output_path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            adjust_brightness,
            adjust_contrast,
            adjust_saturation,
            convert_grayscale,
            apply_sepia,
            shift_hue,
            colorize_grayscale
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
