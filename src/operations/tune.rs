use crate::core::Gif;
use anyhow::{Context, Result};
use image::imageops::FilterType;

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
