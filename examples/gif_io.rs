// Example: Loading and Saving GIF files
//
// This example demonstrates how to use the GIF Toolkit to:
// 1. Load a GIF file from disk
// 2. Access its metadata and frames
// 3. Modify the GIF
// 4. Save it back to disk

use gif_toolkit::core::{Frame, Gif};
use std::env;

fn main() -> anyhow::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.gif> [output.gif]", args[0]);
        eprintln!("Example: {} input.gif output.gif", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        format!("output_{}", input_path)
    };

    println!("Loading GIF from: {}", input_path);

    // Load GIF from file
    let gif = Gif::from_file(input_path)?;

    println!("GIF Information:");
    println!("  Dimensions: {}x{}", gif.width, gif.height);
    println!("  Frames: {}", gif.frame_count());
    println!("  Total duration: {} ms", gif.total_duration() * 10);
    println!("  Loop count: {}",
        if gif.loop_count == 0 { "Infinite".to_string() }
        else { gif.loop_count.to_string() }
    );
    println!("  Global palette: {}",
        if gif.global_palette.is_some() {
            format!("{} colors", gif.global_palette.as_ref().unwrap().len())
        } else {
            "None".to_string()
        }
    );

    // Display frame information
    println!("\nFrame Details:");
    for (i, frame) in gif.frames.iter().enumerate() {
        println!("  Frame {}: {}x{}, delay: {}ms, transparent: {}",
            i + 1,
            frame.width,
            frame.height,
            frame.delay * 10,
            frame.transparent
        );
    }

    // Save the GIF (demonstrates to_file functionality)
    println!("\nSaving GIF to: {}", output_path);
    gif.to_file(&output_path)?;

    println!("Done!");

    Ok(())
}
