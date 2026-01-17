// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Import gif-toolkit library
use gif_toolkit::core::Gif;

#[derive(Debug, Serialize, Deserialize)]
pub struct GifInfo {
    file_path: String,
    file_size: u64,
    file_size_mb: f64,
    width: u16,
    height: u16,
    frame_count: usize,
    duration_sec: f64,
    avg_delay_ms: u32,
    loop_count: u16,
    has_palette: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    success: bool,
    message: String,
    output_size: Option<u64>,
    compression_ratio: Option<f64>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_gif_info(file_path: String) -> Result<GifInfo, String> {
    let path = PathBuf::from(&file_path);

    // Check if file exists
    if !path.exists() {
        return Err(format!("File does not exist: {}", file_path));
    }

    // Get file size
    let metadata = fs::metadata(&path)
        .map_err(|e| format!("Failed to read file metadata: {}", e))?;
    let file_size = metadata.len();
    let file_size_mb = file_size as f64 / (1024.0 * 1024.0);

    // Load GIF
    let gif = Gif::from_file(&file_path)
        .map_err(|e| format!("Failed to load GIF: {}", e))?;

    // Calculate duration
    let total_duration_cs = gif.total_duration();
    let duration_sec = total_duration_cs as f64 / 100.0;

    // Calculate average frame delay
    let avg_delay_ms = if !gif.frames.is_empty() {
        (total_duration_cs / gif.frame_count() as u32) * 10
    } else {
        0
    };

    Ok(GifInfo {
        file_path,
        file_size,
        file_size_mb,
        width: gif.width,
        height: gif.height,
        frame_count: gif.frame_count(),
        duration_sec,
        avg_delay_ms,
        loop_count: gif.loop_count,
        has_palette: gif.global_palette.is_some(),
    })
}

#[tauri::command]
fn process_speed(
    input_path: String,
    output_path: String,
    factor: f64,
) -> Result<ProcessResult, String> {
    // Get original file size
    let original_size = fs::metadata(&input_path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Import and use gif_toolkit operations
    use gif_toolkit::operations::speed;

    speed::run(&input_path, &output_path, factor)
        .map_err(|e| format!("Speed adjustment failed: {}", e))?;

    // Get output file size
    let output_size = fs::metadata(&output_path)
        .map(|m| m.len())
        .ok();

    let compression_ratio = if let Some(os) = output_size {
        if original_size > 0 {
            Some(((original_size - os) as f64 / original_size as f64) * 100.0)
        } else {
            None
        }
    } else {
        None
    };

    Ok(ProcessResult {
        success: true,
        message: format!("Speed adjusted by {:.1}x", factor),
        output_size,
        compression_ratio,
    })
}

#[tauri::command]
fn process_compress(
    input_path: String,
    output_path: String,
    percent: u8,
) -> Result<ProcessResult, String> {
    // Get original file size
    let original_size = fs::metadata(&input_path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Import and use gif_toolkit operations
    use gif_toolkit::operations::compress;

    compress::run(&input_path, &output_path, percent)
        .map_err(|e| format!("Compression failed: {}", e))?;

    // Get output file size
    let output_size = fs::metadata(&output_path)
        .map(|m| m.len())
        .ok();

    let compression_ratio = if let Some(os) = output_size {
        if original_size > 0 {
            Some(((original_size - os) as f64 / original_size as f64) * 100.0)
        } else {
            None
        }
    } else {
        None
    };

    Ok(ProcessResult {
        success: true,
        message: format!("Compressed to {}% of original size", percent),
        output_size,
        compression_ratio,
    })
}

#[tauri::command]
fn process_tune(
    input_path: String,
    output_path: String,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<ProcessResult, String> {
    // Get original file size
    let original_size = fs::metadata(&input_path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Import and use gif_toolkit operations
    use gif_toolkit::operations::tune;

    tune::run(&input_path, &output_path, width, height)
        .map_err(|e| format!("Tune operation failed: {}", e))?;

    // Get output file size
    let output_size = fs::metadata(&output_path)
        .map(|m| m.len())
        .ok();

    let compression_ratio = if let Some(os) = output_size {
        if original_size > 0 {
            Some(((original_size - os) as f64 / original_size as f64) * 100.0)
        } else {
            None
        }
    } else {
        None
    };

    let dims_message = match (width, height) {
        (Some(w), Some(h)) => format!("resized to {}x{}", w, h),
        (Some(w), None) => format!("width set to {}", w),
        (None, Some(h)) => format!("height set to {}", h),
        (None, None) => "no change".to_string(),
    };

    Ok(ProcessResult {
        success: true,
        message: format!("Image tuned: {}", dims_message),
        output_size,
        compression_ratio,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_gif_info,
            process_speed,
            process_compress,
            process_tune
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
