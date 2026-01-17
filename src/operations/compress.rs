use crate::core::{Frame, Gif};
use anyhow::{Context, Result};
use std::fs;

/// Normalize all frames to full GIF dimensions
///
/// Fill transparent areas with background color to prevent flickering with Background disposal
fn normalize_frames(gif: &mut Gif) -> Result<()> {
    if gif.frames.is_empty() {
        return Ok(());
    }

    // Calculate expected full frame size
    let full_frame_size = (gif.width as usize) * (gif.height as usize) * 4;

    // Check if any frame needs normalization
    let needs_normalization = gif.frames.iter().any(|f| f.data.len() < full_frame_size);

    if !needs_normalization {
        return Ok(());
    }

    println!("      Normalizing frames to full dimensions...");

    // Get background color from global palette
    let bg_color: u8 = if let Some(palette) = &gif.global_palette {
        if !palette.is_empty() {
            palette[0][0]  // palette[0] is [u8; 3], we need the first byte (R)
        } else {
            255  // Default to white
        }
    } else {
        255  // Default to white
    };

    // Apply normalization to all frames
    for frame in &mut gif.frames {
        // Check if this is a partial frame
        if frame.data.len() < full_frame_size {
            // Create a full-size canvas filled with background color
            let mut canvas: Vec<u8> = vec![bg_color; full_frame_size];

            // Calculate offset to center the partial frame
            let offset_x = ((gif.width - frame.width) / 2) as usize;
            let offset_y = ((gif.height - frame.height) / 2) as usize;

            let frame_stride = (frame.width as usize) * 4;
            let canvas_stride = (gif.width as usize) * 4;

            // Copy the partial frame to the center of the canvas
            for y in 0..(frame.height as usize) {
                let frame_row_start = y * frame_stride;
                let canvas_row_start = (offset_y * canvas_stride) + (offset_x * 4);

                // Copy pixel data
                let row_bytes = frame.width as usize * 4;
                if frame_row_start + row_bytes <= frame.data.len()
                    && canvas_row_start + row_bytes <= canvas.len() {
                    // Iterate over pixels, not bytes
                    for x in 0..frame.width as usize {
                        let pixel_offset = x * 4;
                        let src_alpha = frame.data[frame_row_start + pixel_offset + 3];
                        if src_alpha > 0 {
                            // Copy all 4 channels
                            for c in 0..4 {
                                canvas[canvas_row_start + pixel_offset + c] = frame.data[frame_row_start + pixel_offset + c];
                            }
                        }
                        // Keep background color if transparent
                    }
                }
            }

            // Replace frame data with the filled canvas
            frame.data = canvas;
            frame.width = gif.width;
            frame.height = gif.height;
        }
    }

    Ok(())
}

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
    let mut gif = Gif::from_file(input).context("Failed to load input GIF")?;

    // Get original file size
    let original_size = fs::metadata(input)?.len();
    let target_size = (original_size * target_percent as u64) / 100;

    println!("   Original size: {} bytes", original_size);
    println!("   Target size: {} bytes", target_size);

    // Determine compression strategy based on target
    // IMPORTANT: Use 256 colors for ALL targets to prevent color shift (色差)
    // Only use lossy compression and other methods to reduce size
    let (_skip_dedup, initial_colors, lossy_quality, apply_steps, skip_normalize) = if target_percent >= 90 {
        // Maximum quality - skip normalization entirely
        (true, 256, 100, false, true)
    } else if target_percent >= 80 {
        // Very high quality - skip normalization
        (true, 256, 98, false, true)
    } else if target_percent >= 70 {
        // High quality - skip normalization
        (true, 256, 96, true, true)
    } else if target_percent >= 60 {
        // Good quality - skip normalization
        (true, 256, 94, true, true)
    } else if target_percent >= 50 {
        // Medium-high quality - skip normalization
        (true, 256, 92, true, true)
    } else if target_percent >= 40 {
        // Medium quality - NO color quantization to avoid color shift
        (true, 256, 90, true, true)
    } else if target_percent >= 30 {
        // Medium-low quality - NO color quantization
        (true, 256, 88, true, true)
    } else if target_percent >= 20 {
        // Low quality - NO color quantization
        (true, 256, 85, true, true)
    } else {
        // Very low quality - NO color quantization, only lossy compression
        (true, 256, 82, true, true)
    };

    // Normalize frames to full dimensions BEFORE compression
    // For high quality targets, skip normalization to preserve original quality
    if skip_normalize {
        println!("   Skipping frame normalization to preserve quality");
    } else {
        normalize_frames(&mut gif)?;
    }

    // Apply iterative compression strategy
    let temp_path = format!("{}.temp", output);

    let mut final_step_reached = false;

    for step_num in 0..10 {
        println!("   Applying compression step {}...", step_num + 1);

        // Apply the appropriate compression step based on target
        match step_num {
            0 => {
                println!("      Frame deduplication disabled to preserve animation");
            }
            1 => {
                if initial_colors < 256 {
                    reduce_colors(&mut gif, initial_colors)?;
                } else {
                    println!("      Skipping color reduction (already optimal)");
                }
            }
            2 => {
                if lossy_quality < 100 {
                    apply_lossy_compression(&mut gif, lossy_quality)?;
                } else {
                    println!("      Skipping lossy compression (lossless mode)");
                }
            }
            3 => {
                // Additional color reduction based on target
                // IMPORTANT: Keep 256 colors for all targets to avoid color shift
                let next_colors = if target_percent >= 70 {
                    256 // Keep max colors for 70%+
                } else if target_percent >= 60 {
                    256 // Keep max colors for 60%+ to avoid color shift
                } else if target_percent >= 50 {
                    256 // Keep max colors for 50%+ to avoid color shift
                } else if target_percent >= 40 {
                    256 // Keep max colors for 40%+ to avoid color shift
                } else if target_percent >= 30 {
                    256 // Keep max colors for 30%+ to avoid color shift
                } else if target_percent >= 20 {
                    256 // Keep max colors for 20%+ to avoid color shift
                } else {
                    256 // Keep max colors for 10% to avoid color shift
                };

                if next_colors < initial_colors {
                    reduce_colors(&mut gif, next_colors)?;
                } else {
                    println!("      Skipping color reduction (preserving original colors)");
                }
            }
            4 => {
                // Additional lossy compression based on target
                if target_percent < 90 && lossy_quality > 80 {
                    let additional_quality = if target_percent >= 80 {
                        lossy_quality // Keep same for 80%+
                    } else if target_percent >= 70 {
                        lossy_quality.saturating_sub(2)
                    } else if target_percent >= 60 {
                        lossy_quality.saturating_sub(4)
                    } else {
                        lossy_quality.saturating_sub(6)
                    };

                    if additional_quality < lossy_quality {
                        apply_lossy_compression(&mut gif, additional_quality)?;
                    }
                }
            }
            _ => break,
        }

        // Save to temp file and check size
        gif.to_file(&temp_path)
            .context("Failed to write temporary GIF")?;

        let current_size = fs::metadata(&temp_path)?.len();
        let current_percent = (current_size as f64 / original_size as f64) * 100.0;

        println!(
            "   Current size after step {}: {} bytes ({:.1}%)",
            step_num + 1,
            current_size,
            current_percent
        );

        // Check if we've reached or exceeded the target
        if current_size <= target_size {
            // For very low targets, stop early
            if target_percent < 15 {
                println!("   Target size reached!");
                final_step_reached = true;
                break;
            }
        }

        // For higher quality targets (90%+), stop if we're close to original size
        if target_percent >= 90 {
            // Stop if within 10% of target or after step 3
            if current_percent <= target_percent as f64 + 10.0 || step_num >= 3 {
                println!("   Close to target, stopping for quality");
                final_step_reached = true;
                break;
            }
        } else if target_percent >= 70 {
            // Continue compressing to apply quality settings
            if current_percent <= target_percent as f64 + 15.0 && step_num >= 3 {
                println!("   Close to target, stopping for quality");
                final_step_reached = true;
                break;
            }
        } else if target_percent >= 40 {
            if current_percent >= target_percent as f64 - 5.0 && current_percent <= target_percent as f64 + 10.0 {
                println!("   Close to target, stopping for quality");
                final_step_reached = true;
                break;
            }
        } else {
            // For low quality targets, stop when close
            if current_percent <= target_percent as f64 + 5.0 {
                println!("   Close to target, stopping for quality");
                final_step_reached = true;
                break;
            }
        }

        // If we're getting too small (less than 50% of target), stop
        if current_size < target_size / 2 && target_percent > 20 {
            println!("   Size too small, stopping compression");
            break;
        }
    }

    // If no steps achieved the target, try one more aggressive step
    // But skip this for high quality targets (70%+) to preserve quality
    // IMPORTANT: Use stronger lossy compression instead of color reduction to avoid color shift
    if !final_step_reached && fs::metadata(&temp_path)?.len() > target_size && target_percent < 70 {
        println!("   Applying final aggressive compression...");
        // Use stronger lossy compression instead of reducing colors
        apply_lossy_compression(&mut gif, 70)?;
        gif.to_file(&temp_path)?;
    }

    // Rename temp file to output
    fs::rename(&temp_path, output).context("Failed to rename temporary file")?;

    let final_size = fs::metadata(output)?.len();
    let compression_ratio = if final_size < original_size {
        ((original_size - final_size) as f64 / original_size as f64) * 100.0
    } else {
        -((final_size - original_size) as f64 / original_size as f64) * 100.0
    };

    println!("   Final size: {} bytes", final_size);
    if compression_ratio >= 0.0 {
        println!("   Compression achieved: {:.1}%", compression_ratio);
    } else {
        println!("   Size increased: {:.1}%", -compression_ratio);
    }

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
        let r_diff = (p1[0] as i16 - p2[0] as i16).unsigned_abs() as u64;
        let g_diff = (p1[1] as i16 - p2[1] as i16).unsigned_abs() as u64;
        let b_diff = (p1[2] as i16 - p2[2] as i16).unsigned_abs() as u64;
        let a_diff = (p1[3] as i16 - p2[3] as i16).unsigned_abs() as u64;

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

    println!(
        "      Deduplicated: {} -> {} frames",
        original_count,
        gif.frames.len()
    );

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

    // Flatten color data for color_quant
    let flat_colors: Vec<u8> = all_colors.iter().flatten().copied().collect();

    // Use color_quant to create optimized palette
    let quantizer = color_quant::NeuQuant::new(10, max_colors, &flat_colors);
    let palette = quantizer.color_map_rgb();

    // Apply the palette to all frames
    for frame in &mut gif.frames {
        for pixel in frame.data.chunks_exact_mut(4) {
            if pixel[3] > 0 {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                // Get the closest color from the palette
                let fallback = [r, g, b];
                let closest = palette
                    .chunks(3)
                    .min_by_key(|color| {
                        let dr = (color.first().copied().unwrap_or(0) as i32 - r as i32).abs();
                        let dg = (color.get(1).copied().unwrap_or(0) as i32 - g as i32).abs();
                        let db = (color.get(2).copied().unwrap_or(0) as i32 - b as i32).abs();
                        dr + dg + db
                    })
                    .unwrap_or(&fallback);

                pixel[0] = closest.first().copied().unwrap_or(r);
                pixel[1] = closest.get(1).copied().unwrap_or(g);
                pixel[2] = closest.get(2).copied().unwrap_or(b);
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
    let factor = 100 - quality;
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

    println!(
        "      Reducing frames: {} -> {}",
        gif.frames.len(),
        target_count
    );

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
        use crate::core::Frame;

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
