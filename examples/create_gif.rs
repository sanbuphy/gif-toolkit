// Example: Creating a simple animated GIF programmatically
//
// This example demonstrates how to create a GIF from scratch:
// 1. Create frames with different colors
// 2. Set proper delays
// 3. Add them to a GIF structure
// 4. Save to file

use gif_toolkit::core::{Frame, Gif};

fn main() -> anyhow::Result<()> {
    let width = 100u16;
    let height = 100u16;

    println!("Creating a simple animated GIF");

    // Create a new GIF
    let mut gif = Gif::new();

    // Create 10 frames with changing colors (red to blue gradient)
    for i in 0..10 {
        let mut frame = Frame::new(width, height);

        // Calculate color for this frame (gradient from red to blue)
        let r = (255 * (10 - i) / 10) as u8;
        let g = 0;
        let b = (255 * i / 10) as u8;

        // Fill the frame with the calculated color
        for pixel in frame.data.chunks_exact_mut(4) {
            pixel[0] = r; // R
            pixel[1] = g; // G
            pixel[2] = b; // B
            pixel[3] = 255; // A (fully opaque)
        }

        // Set delay to 100ms (10 units of 10ms)
        frame.delay = 10;

        // Add the frame to the GIF
        gif.add_frame(frame);
    }

    // Set loop count to infinite (0)
    gif.loop_count = 0;

    println!("Created GIF with {} frames", gif.frame_count());
    println!("Dimensions: {}x{}", gif.width, gif.height);
    println!("Total duration: {} ms", gif.total_duration() * 10);

    // Save the GIF
    let output_path = "examples/simple_gradient.gif";
    println!("\nSaving to: {}", output_path);
    gif.to_file(output_path)?;

    println!("Done! You can now view the GIF at: {}", output_path);

    Ok(())
}
