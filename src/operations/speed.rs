use anyhow::{Context, Result};
use gif_toolkit::core::Gif;

/// Adjust GIF playback speed by the given factor
///
/// # Arguments
/// * `input` - Path to input GIF file
/// * `output` - Path to output GIF file
/// * `factor` - Speed multiplier (e.g., 2.0 = 2x faster, 0.5 = 2x slower)
///
/// # Example
/// ```no_run
/// use gif_toolkit::operations::speed;
///
/// speed::run("input.gif", "output.gif", 2.0).unwrap();
/// ```
pub fn run(input: &str, output: &str, factor: f64) -> Result<()> {
    // Validate factor
    if factor <= 0.0 {
        anyhow::bail!("Speed factor must be greater than 0");
    }

    // Load the GIF
    let mut gif = Gif::from_file(input)
        .context("Failed to load input GIF")?;

    println!("   Input file: {}", input);
    println!("   Speed factor: {:.2}x", factor);
    println!("   Original frames: {}", gif.frames.len());

    // Adjust frame delays
    for frame in &mut gif.frames {
        let new_delay = (frame.delay as f64 / factor).round() as u16;
        frame.delay = new_delay.max(1); // Minimum delay is 1 centisecond
    }

    // For extreme speedups (> 4.0), consider dropping frames
    if factor > 4.0 {
        let frames_to_keep = (gif.frames.len() as f64 / factor).ceil() as usize;
        let frames_to_keep = frames_to_keep.max(1);

        // Keep every Nth frame
        let step = (gif.frames.len() as f64 / frames_to_keep as f64).ceil() as usize;

        let mut filtered_frames = Vec::new();
        for (i, frame) in gif.frames.iter().enumerate() {
            if i % step == 0 {
                filtered_frames.push(frame.clone());
            }
        }

        gif.frames = filtered_frames;
        println!("   Frames after dropping: {}", gif.frames.len());
    }

    // Save the modified GIF
    gif.to_file(output)
        .context("Failed to save output GIF")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_factor() {
        // Valid factors
        assert!(run("test.gif", "out.gif", 1.0).is_ok());
        assert!(run("test.gif", "out.gif", 2.0).is_ok());
        assert!(run("test.gif", "out.gif", 0.5).is_ok());

        // Invalid factors (will fail when we actually implement)
        // assert!(run("test.gif", "out.gif", 0.0).is_err());
        // assert!(run("test.gif", "out.gif", -1.0).is_err());
    }
}
