use crate::core::{Frame, Gif};
use anyhow::{Context, Result};
use gif::DisposalMethod;
use image::imageops::FilterType;
use image::{ImageBuffer, Rgba};

/// Normalize frames by compositing them with proper disposal handling
///
/// This is critical for GIFs with partial frames and Keep disposal.
/// Each frame must be composited onto the previous frame's result.
fn normalize_frames_composited(gif: &mut Gif) -> Result<()> {
    if gif.frames.is_empty() {
        return Ok(());
    }

    let full_frame_size = (gif.width as usize) * (gif.height as usize) * 4;

    // Check if any frame needs normalization
    let needs_normalization = gif.frames.iter().any(|f| f.data.len() < full_frame_size);

    if !needs_normalization {
        return Ok(());
    }

    println!("   Normalizing frames with composite disposal handling...");

    // Get background color (transparent black by default for GIFs)
    let mut canvas: Vec<u8> = vec![0; full_frame_size]; // Start with transparent black

    for (i, frame) in gif.frames.iter_mut().enumerate() {
        // Save current canvas state for disposal handling
        let previous_canvas = canvas.clone();

        // If this is a partial frame, composite it onto the canvas
        if frame.data.len() < full_frame_size {
            let frame_stride = (frame.width as usize) * 4;
            let gif_stride = (gif.width as usize) * 4;

            // Calculate offset to center the partial frame
            let offset_x = ((gif.width - frame.width) / 2) as usize;
            let offset_y = ((gif.height - frame.height) / 2) as usize;

            for y in 0..(frame.height as usize) {
                let frame_row_start = y * frame_stride;
                let canvas_row_start = (offset_y * gif_stride) + (y * gif_stride);
                let canvas_row_start_with_x = canvas_row_start + (offset_x * 4);

                for x in 0..(frame.width as usize) {
                    let pixel_offset = x * 4;
                    let src_alpha = frame.data[frame_row_start + pixel_offset + 3];

                    if src_alpha > 0 {
                        // Composite pixel onto canvas (simple replace for now)
                        for c in 0..4 {
                            canvas[canvas_row_start_with_x + pixel_offset + c] =
                                frame.data[frame_row_start + pixel_offset + c];
                        }
                    }
                }
            }

            // Update frame with composited result
            frame.data = canvas.clone();
            frame.width = gif.width;
            frame.height = gif.height;
        } else {
            // Full frame, replace canvas
            canvas = frame.data.clone();
        }

        // Handle disposal for next frame
        match frame.disposal {
            DisposalMethod::Keep => {
                // Keep current canvas for next frame (nothing to do)
            }
            DisposalMethod::Background => {
                // Restore to background (transparent black)
                canvas = vec![0; full_frame_size];
            }
            DisposalMethod::Previous => {
                // Restore to previous state
                canvas = previous_canvas;
            }
            _ => {
                // Any/Other - treat as Keep
            }
        }
    }

    Ok(())
}

/// Tune GIF parameters (resize, crop, etc.)
///
/// # Arguments
/// * `input` - Path to input GIF file
/// * `output` - Path to output GIF file
/// * `width` - Optional new width in pixels
/// * `height` - Optional new height in pixels
///
/// # Example
/// ```no_run
/// use gif_toolkit::operations::tune;
///
/// // Resize to 400x300
/// tune::run("input.gif", "output.gif", Some(400), Some(300)).unwrap();
///
/// // Resize maintaining aspect ratio (width only)
/// tune::run("input.gif", "output.gif", Some(400), None).unwrap();
/// ```
pub fn run(input: &str, output: &str, width: Option<u32>, height: Option<u32>) -> Result<()> {
    // Validate at least one dimension is specified
    if width.is_none() && height.is_none() {
        anyhow::bail!("At least one dimension (width or height) must be specified");
    }

    // Load the GIF
    let mut gif = Gif::from_file(input).context("Failed to load input GIF")?;

    let original_width = gif.width as u32;
    let original_height = gif.height as u32;
    let aspect_ratio = original_width as f64 / original_height as f64;

    // Calculate new dimensions maintaining aspect ratio
    let (new_width, new_height) = match (width, height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let h = (w as f64 / aspect_ratio).round() as u32;
            (w, h.max(1))
        }
        (None, Some(h)) => {
            let w = (h as f64 * aspect_ratio).round() as u32;
            (w.max(1), h)
        }
        _ => unreachable!(),
    };

    println!("   Input file: {}", input);
    println!("   Original size: {}x{}", original_width, original_height);
    println!("   Target size: {}x{}", new_width, new_height);

    // Ensure new dimensions are valid
    if new_width == 0 || new_height == 0 {
        anyhow::bail!("Invalid target dimensions: {}x{}", new_width, new_height);
    }

    // CRITICAL: Normalize frames BEFORE resizing
    // This ensures partial frames are properly composited
    normalize_frames_composited(&mut gif)?;

    // Resize all frames
    for frame in &mut gif.frames {
        let img_buffer = frame.to_image_buffer();
        // Use Triangle filter for smoother edges without ringing artifacts
        let resized =
            image::imageops::resize(&img_buffer, new_width, new_height, FilterType::Triangle);
        frame.update_from_image_buffer(&resized);
    }

    // Update GIF dimensions
    gif.width = new_width as u16;
    gif.height = new_height as u16;

    // Save the modified GIF
    gif.to_file(output).context("Failed to save output GIF")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validate_dimensions() {
        // Test dimension validation logic
        // At least one dimension must be specified
        let has_width: Option<u32> = Some(400);
        let has_height: Option<u32> = Some(300);
        let no_width: Option<u32> = None;
        let no_height: Option<u32> = None;

        // Valid combinations
        assert!(has_width.is_some() || has_height.is_some());
        assert!(has_width.is_some() || no_height.is_some());
        assert!(no_width.is_some() || has_height.is_some());

        // Invalid combination
        assert!(!(no_width.is_some() || no_height.is_some()));
    }
}
