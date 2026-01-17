use anyhow::{Context, Result};
use crate::core::{Frame, Gif};
use std::fs;

/// Compress GIF file size by the given percentage
///
/// # Arguments
/// * `input` - Path to input GIF file
/// * `output` - Path to output GIF file
/// * `target_percent` - Target compression percentage (1-99)
///
/// # Example
/// ```no_run
/// use gif_toolkit::operations::compress;
///
/// // Compress to 50% of original size
/// compress::run("input.gif", "output.gif", 50).unwrap();
/// ```
pub fn run(input: &str, output: &str, target_percent: u8) -> Result<()> {
    // Validate percentage
    if target_percent == 0 || target_percent > 99 {
        anyhow::bail!("Compression percentage must be between 1 and 99");
    }

    println!("   Input file: {}", input);
    println!("   Compression target: {}%", target_percent);

    // Load the input GIF
    let mut gif = Gif::from_file(input)
        .context("Failed to load input GIF")?;

    // Get original file size
    let original_size = fs::metadata(input)?.len();
    let target_size = (original_size * target_percent as u64) / 100;

    println!("   Original size: {} bytes", original_size);
    println!("   Target size: {} bytes", target_size);

    // Apply iterative compression strategy
    let compression_steps = [
        // Step 1: Frame deduplication (threshold 10)
        || deduplicate_frames(&mut gif, 10),
        // Step 2: Reduce colors to 128
        || reduce_colors(&mut gif, 128),
        // Step 3: Lossy compression (quality 80)
        || apply_lossy_compression(&mut gif, 80),
        // Step 4: Reduce colors to 64
        || reduce_colors(&mut gif, 64),
        // Step 5: Lossy compression (quality 60)
        || apply_lossy_compression(&mut gif, 60),
        // Step 6: Reduce colors to 32
        || reduce_colors(&mut gif, 32),
        // Step 7: Lossy compression (quality 40)
        || apply_lossy_compression(&mut gif, 40),
        // Step 8: Frame deduplication (threshold 5)
        || deduplicate_frames(&mut gif, 5),
        // Step 9: Reduce colors to 16
        || reduce_colors(&mut gif, 16),
        // Step 10: Reduce frame count to 70%
        || reduce_frame_count(&mut gif, 0.7),
    ];

    let mut temp_path = format!("{}.temp", output);

    for (step_num, step_fn) in compression_steps.iter().enumerate() {
        println!("   Applying compression step {}...", step_num + 1);

        // Apply the compression step
        step_fn()
            .with_context(|| format!("Failed to apply compression step {}", step_num + 1))?;

        // Save to temp file and check size
        gif.to_file(&temp_path)
            .context("Failed to write temporary GIF")?;

        let current_size = fs::metadata(&temp_path)?.len();
        println!("   Current size after step {}: {} bytes", step_num + 1, current_size);

        // Check if we've reached the target
        if current_size <= target_size {
            println!("   Target size reached!");
            break;
        }

        // If we're getting too small, stop
        if current_size < target_size / 2 {
            println!("   Size too small, stopping compression");
            break;
        }
    }

    // Rename temp file to output
    fs::rename(&temp_path, output)
        .context("Failed to rename temporary file")?;

    let final_size = fs::metadata(output)?.len();
    let compression_ratio = ((original_size - final_size) as f64 / original_size as f64) * 100.0;

    println!("   Final size: {} bytes", final_size);
    println!("   Compression achieved: {:.1}%", compression_ratio);

    Ok(())
}

/// Calculate the difference between two frames
///
/// Returns a value from 0-255 representing the average pixel difference
fn calculate_frame_difference(frame1: &Frame, frame2: &Frame) -> u8 {
    if frame1.width != frame2.width || frame1.height != frame2.height {
        return 255; // Maximum difference if dimensions don't match
    }

    if frame1.data.len() != frame2.data.len() {
        return 255;
    }

    let mut total_diff = 0u64;
    let pixel_count = (frame1.width as u64) * (frame1.height as u64);

    // Compare RGBA pixels
    for (p1, p2) in frame1.data.chunks(4).zip(frame2.data.chunks(4)) {
        // Calculate per-channel difference
        let r_diff = (p1[0] as i16 - p2[0] as i16).abs() as u64;
        let g_diff = (p1[1] as i16 - p2[1] as i16).abs() as u64;
        let b_diff = (p1[2] as i16 - p2[2] as i16).abs() as u64;
        let a_diff = (p1[3] as i16 - p2[3] as i16).abs() as u64;

        // Average difference across channels
        total_diff += (r_diff + g_diff + b_diff + a_diff) / 4;
    }

    if pixel_count == 0 {
        return 0;
    }

    (total_diff / pixel_count) as u8
}

/// Deduplicate frames that are similar to each other
///
/// Frames with difference less than threshold are merged
fn deduplicate_frames(gif: &mut Gif, threshold: u8) -> Result<()> {
    if gif.frames.len() <= 1 {
        return Ok(());
    }

    let mut unique_frames = Vec::new();
    let mut last_frame = &gif.frames[0];

    unique_frames.push(last_frame.clone());

    for current_frame in &gif.frames[1..] {
        let diff = calculate_frame_difference(last_frame, current_frame);

        if diff < threshold {
            // Merge: add the delay to the last unique frame
            if let Some(last) = unique_frames.last_mut() {
                last.delay += current_frame.delay;
            }
        } else {
            // Keep the frame
            unique_frames.push(current_frame.clone());
            last_frame = current_frame;
        }
    }

    let original_count = gif.frames.len();
    gif.frames = unique_frames;

    println!("      Deduplicated: {} -> {} frames", original_count, gif.frames.len());

    Ok(())
}

/// Reduce the color palette of the GIF
///
/// Uses median cut algorithm to find optimal color palette
fn reduce_colors(gif: &mut Gif, max_colors: usize) -> Result<()> {
    if max_colors >= 256 {
        return Ok(());
    }

    println!("      Reducing colors to {}", max_colors);

    // Collect all unique colors from all frames
    let mut all_colors = Vec::new();

    for frame in &gif.frames {
        for pixel in frame.data.chunks(4) {
            // Only add opaque or semi-transparent pixels
            if pixel[3] > 0 {
                all_colors.push([pixel[0], pixel[1], pixel[2]]);
            }
        }
    }

    if all_colors.is_empty() {
        return Ok(());
    }

    // Use color_quant to create optimized palette
    let quantizer = color_quant::NeuQuant::new(10, max_colors, &all_colors);
    let palette = quantizer.color_map_rgb();

    // Apply the palette to all frames
    for frame in &mut gif.frames {
        for pixel in frame.data.chunks_exact_mut(4) {
            if pixel[3] > 0 {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                // Get the closest color from the palette
                let [nr, ng, nb] = palette
                    .chunks(3)
                    .min_by_key(|color| {
                        let dr = (color[0] as i32 - r as i32).abs();
                        let dg = (color[1] as i32 - g as i32).abs();
                        let db = (color[2] as i32 - b as i32).abs();
                        dr + dg + db
                    })
                    .unwrap_or(&[r, g, b]);

                pixel[0] = nr;
                pixel[1] = ng;
                pixel[2] = nb;
            }
        }
    }

    Ok(())
}

/// Apply lossy compression by simplifying similar colors
///
/// quality: 0-100, where 100 is lossless
fn apply_lossy_compression(gif: &mut Gif, quality: u8) -> Result<()> {
    if quality >= 100 {
        return Ok(());
    }

    println!("      Applying lossy compression (quality: {})", quality);

    // Calculate the quantization factor
    // Lower quality = larger factor = more aggressive compression
    let factor = (100 - quality) as u8;
    if factor == 0 {
        return Ok(());
    }

    for frame in &mut gif.frames {
        for pixel in frame.data.chunks_exact_mut(4) {
            if pixel[3] > 0 {
                // Quantize each color channel
                pixel[0] = (pixel[0] / factor) * factor;
                pixel[1] = (pixel[1] / factor) * factor;
                pixel[2] = (pixel[2] / factor) * factor;
            }
        }
    }

    Ok(())
}

/// Reduce the number of frames in the GIF
///
/// Keeps the specified percentage of frames
fn reduce_frame_count(gif: &mut Gif, percentage: f64) -> Result<()> {
    if percentage >= 1.0 || percentage <= 0.0 {
        return Ok(());
    }

    let target_count = (gif.frames.len() as f64 * percentage).ceil() as usize;

    if target_count >= gif.frames.len() || target_count == 0 {
        return Ok(());
    }

    println!("      Reducing frames: {} -> {}", gif.frames.len(), target_count);

    // Calculate step size
    let step = gif.frames.len() as f64 / target_count as f64;

    let mut selected_frames = Vec::new();
    let mut current_pos = 0.0;

    while selected_frames.len() < target_count && current_pos < gif.frames.len() as f64 {
        let idx = current_pos.floor() as usize;
        if idx < gif.frames.len() {
            selected_frames.push(gif.frames[idx].clone());
        }
        current_pos += step;
    }

    // Adjust delays to maintain total duration
    let original_duration: u32 = gif.frames.iter().map(|f| f.delay as u32).sum();
    let new_duration: u32 = selected_frames.iter().map(|f| f.delay as u32).sum();

    if new_duration > 0 {
        let ratio = original_duration as f64 / new_duration as f64;
        for frame in &mut selected_frames {
            frame.delay = (frame.delay as f64 * ratio) as u16;
            // Ensure minimum delay
            if frame.delay == 0 {
                frame.delay = 1;
            }
        }
    }

    gif.frames = selected_frames;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_frame_difference() {
        use gif_toolkit::core::Frame;

        // Create two identical frames
        let data = vec![255u8; 10 * 10 * 4];
        let frame1 = Frame::from_rgba(data.clone(), 10, 10);
        let frame2 = Frame::from_rgba(data, 10, 10);

        let diff = calculate_frame_difference(&frame1, &frame2);
        assert_eq!(diff, 0);

        // Create completely different frames
        let data1 = vec![0u8; 10 * 10 * 4];
        let data2 = vec![255u8; 10 * 10 * 4];
        let frame3 = Frame::from_rgba(data1, 10, 10);
        let frame4 = Frame::from_rgba(data2, 10, 10);

        let diff2 = calculate_frame_difference(&frame3, &frame4);
        assert!(diff2 > 200);
    }
}
