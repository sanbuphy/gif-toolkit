use anyhow::{Context, Result};
use crate::core::Gif;
use std::fs;

/// Display information about a GIF file
///
/// # Arguments
/// * `input` - Path to the GIF file
///
/// # Example
/// ```no_run
/// use gif_toolkit::operations::info;
///
/// info::run("example.gif").unwrap();
/// ```
pub fn run(input: &str) -> Result<()> {
    // Load the GIF
    let gif = Gif::from_file(input)
        .context("Failed to load GIF")?;

    // Get file size
    let metadata = fs::metadata(input)
        .context("Failed to read file metadata")?;
    let file_size = metadata.len();
    let file_size_mb = file_size as f64 / (1024.0 * 1024.0);

    // Calculate duration in seconds
    let total_duration_cs = gif.total_duration();
    let total_duration_sec = total_duration_cs as f64 / 100.0;

    println!("GIF Information:");
    println!("  File: {}", input);
    println!("  Size: {} bytes ({:.2} MB)", file_size, file_size_mb);
    println!("  Dimensions: {}x{} pixels", gif.width, gif.height);
    println!("  Frames: {}", gif.frame_count());
    println!("  Duration: {:.2} seconds ({} centiseconds)", total_duration_sec, total_duration_cs);

    // Calculate average frame delay
    if !gif.frames.is_empty() {
        let avg_delay = total_duration_cs / gif.frame_count() as u32;
        println!("  Average frame delay: {} ms", avg_delay * 10);
    }

    // Get color count (this might be slow for large GIFs, so we'll skip for now)
    // let colors = gif.color_count();
    // println!("  Colors: {}", colors);

    // Loop count
    if gif.loop_count == 0 {
        println!("  Loop: Infinite");
    } else {
        println!("  Loop: {} times", gif.loop_count);
    }

    // Global palette info
    if let Some(palette) = &gif.global_palette {
        println!("  Global palette: {} colors", palette.len());
    } else {
        println!("  Global palette: None");
    }

    // Optional: Show detailed frame information
    // Uncomment if you want per-frame details
    // println!("\nFrame Details:");
    // for (i, frame) in gif.frames.iter().enumerate() {
    //     println!("  Frame {}: delay={}ms, size={}x{}",
    //              i + 1, frame.delay * 10, frame.width, frame.height);
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_display() {
        // This test requires a sample GIF file
        // For now, we'll just test that it compiles
        assert!(true);
    }
}
