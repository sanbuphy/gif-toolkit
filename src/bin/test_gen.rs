// Test GIF Generator
//
// This binary generates test GIF files for testing the GIF Toolkit

use anyhow::Result;
use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<()> {
    println!("GIF Test Generator");
    println!("==================");

    // Create fixtures directory
    let fixtures_dir = "tests/fixtures";
    std::fs::create_dir_all(fixtures_dir)?;

    // Generate test GIFs
    generate_simple_gif(fixtures_dir)?;
    generate_colorful_gif(fixtures_dir)?;
    generate_large_gif(fixtures_dir)?;
    generate_duplicates_gif(fixtures_dir)?;
    generate_high_fps_gif(fixtures_dir)?;

    println!("\nAll test GIFs generated successfully!");
    println!("Files created in: {}", fixtures_dir);

    Ok(())
}

/// Generate a simple 2-frame animation
fn generate_simple_gif(dir: &str) -> Result<()> {
    println!("Generating simple.gif...");

    let path = Path::new(dir).join("simple.gif");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let width = 100;
    let height = 100;

    // Create a simple palette: black (index 0), red (index 1), blue (index 2)
    let palette = vec![
        0, 0, 0,    // Index 0: Black (background)
        255, 0, 0,  // Index 1: Red
        0, 0, 255,  // Index 2: Blue
    ];

    let mut encoder = Encoder::new(writer, width, height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Frame 1: Red square
    let mut frame1_data = vec![0u8; width as usize * height as usize];
    for y in 20..80 {
        for x in 20..80 {
            frame1_data[y as usize * width as usize + x as usize] = 1; // Index of red in palette
        }
    }

    let mut frame1 = Frame::from_indexed_pixels(width, height, &frame1_data, None);
    frame1.delay = 20; // 0.2 seconds
    encoder.write_frame(&frame1)?;

    // Frame 2: Blue square
    let mut frame2_data = vec![0u8; width as usize * height as usize];
    for y in 20..80 {
        for x in 20..80 {
            frame2_data[y as usize * width as usize + x as usize] = 2; // Index of blue in palette
        }
    }

    let mut frame2 = Frame::from_indexed_pixels(width, height, &frame2_data, None);
    frame2.delay = 20; // 0.2 seconds
    encoder.write_frame(&frame2)?;

    println!("   ✓ Created simple.gif (2 frames, 100x100)");
    Ok(())
}

/// Generate a colorful 10-frame animation
fn generate_colorful_gif(dir: &str) -> Result<()> {
    println!("Generating colorful.gif...");

    let path = Path::new(dir).join("colorful.gif");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let width = 200;
    let height = 200;

    // Create a rainbow palette
    let mut palette = vec![0u8; 256 * 3];
    for i in 0..256 {
        let hue = (i as f32 / 256.0) * 360.0;
        let rgb = hsv_to_rgb(hue, 1.0, 1.0);
        palette[i * 3] = rgb.0;
        palette[i * 3 + 1] = rgb.1;
        palette[i * 3 + 2] = rgb.2;
    }

    let mut encoder = Encoder::new(writer, width, height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Generate 10 frames with different colors
    for frame_num in 0..10 {
        let color_index = (frame_num * 25) as usize;

        let mut frame_data = vec![0u8; width as usize * height as usize];

        // Create a circle that moves
        let center_x = 100 + ((frame_num as i32 * 15 - 75) as i32);
        let center_y = 100;

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();

                if dist < 40.0 {
                    frame_data[y as usize * width as usize + x as usize] = color_index as u8;
                }
            }
        }

        let mut frame = Frame::from_indexed_pixels(width, height, &frame_data, None);
        frame.delay = 10; // 0.1 seconds
        encoder.write_frame(&frame)?;
    }

    println!("   ✓ Created colorful.gif (10 frames, 200x200)");
    Ok(())
}

/// Generate a large 800x600 GIF
fn generate_large_gif(dir: &str) -> Result<()> {
    println!("Generating large.gif...");

    let path = Path::new(dir).join("large.gif");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let width = 800;
    let height = 600;

    let palette = [255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0];

    let mut encoder = Encoder::new(writer, width, height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Create 5 frames with different colored rectangles
    for frame_num in 0..5 {
        let mut frame_data = vec![0u8; width as usize * height as usize];
        let color_idx = (frame_num % 3 + 1) as u8;

        // Draw a rectangle
        let start_x = 200 + frame_num * 50;
        let start_y = 150 + frame_num * 30;
        let rect_width = 400;
        let rect_height = 300;

        for y in start_y..(start_y + rect_height).min(height) {
            for x in start_x..(start_x + rect_width).min(width) {
                frame_data[y as usize * width as usize + x as usize] = color_idx;
            }
        }

        let mut frame = Frame::from_indexed_pixels(width, height, &frame_data, None);
        frame.delay = 30; // 0.3 seconds
        encoder.write_frame(&frame)?;
    }

    println!("   ✓ Created large.gif (5 frames, 800x600)");
    Ok(())
}

/// Generate a GIF with duplicate frames
fn generate_duplicates_gif(dir: &str) -> Result<()> {
    println!("Generating duplicates.gif...");

    let path = Path::new(dir).join("duplicates.gif");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let width = 150;
    let height = 150;

    let palette = [255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128];

    let mut encoder = Encoder::new(writer, width, height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Create a frame
    let create_frame = |color: u8| -> Vec<u8> {
        let mut data = vec![0u8; width as usize * height as usize];
        for y in 0..height {
            for x in 0..width {
                if x > 50 && x < 100 && y > 50 && y < 100 {
                    data[y as usize * width as usize + x as usize] = color;
                }
            }
        }
        data
    };

    // Frame 1: Red
    let frame1_data = create_frame(1);
    let mut frame1 = Frame::from_indexed_pixels(width, height, &frame1_data, None);
    frame1.delay = 10;
    encoder.write_frame(&frame1)?;

    // Frame 2: Same as frame 1 (duplicate)
    let frame2_data = frame1_data.clone();
    let mut frame2 = Frame::from_indexed_pixels(width, height, &frame2_data, None);
    frame2.delay = 10;
    encoder.write_frame(&frame2)?;

    // Frame 3: Same as frame 1 (duplicate)
    let frame3_data = frame1_data.clone();
    let mut frame3 = Frame::from_indexed_pixels(width, height, &frame3_data, None);
    frame3.delay = 10;
    encoder.write_frame(&frame3)?;

    // Frame 4: Green
    let frame4_data = create_frame(2);
    let mut frame4 = Frame::from_indexed_pixels(width, height, &frame4_data, None);
    frame4.delay = 10;
    encoder.write_frame(&frame4)?;

    // Frame 5: Same as frame 4 (duplicate)
    let frame5_data = frame4_data.clone();
    let mut frame5 = Frame::from_indexed_pixels(width, height, &frame5_data, None);
    frame5.delay = 10;
    encoder.write_frame(&frame5)?;

    println!("   ✓ Created duplicates.gif (5 frames, with duplicates, 150x150)");
    Ok(())
}

/// Generate a high FPS GIF (30 FPS)
fn generate_high_fps_gif(dir: &str) -> Result<()> {
    println!("Generating high_fps.gif...");

    let path = Path::new(dir).join("high_fps.gif");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let width = 100;
    let height = 100;

    let palette = [255, 255, 255, 0, 0, 0];

    let mut encoder = Encoder::new(writer, width, height, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Create 30 frames at 30 FPS (delay = 3.33 centiseconds ≈ 3)
    for frame_num in 0..30 {
        let mut frame_data = vec![0u8; width as usize * height as usize];

        // Draw a bouncing ball
        let ball_y = (frame_num as f32 * 0.2).sin().abs() * 60.0 + 20.0;
        let ball_y = ball_y as u32;

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - 50;
                let dy = y as i32 - ball_y as i32;
                let dist = ((dx * dx + dy * dy) as f64).sqrt();

                if dist < 10.0 {
                    frame_data[y as usize * width as usize + x as usize] = 1;
                }
            }
        }

        let mut frame = Frame::from_indexed_pixels(width, height, &frame_data, None);
        frame.delay = 3; // ~30 FPS
        encoder.write_frame(&frame)?;
    }

    println!("   ✓ Created high_fps.gif (30 frames, ~30 FPS, 100x100)");
    Ok(())
}

/// Convert HSV color to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
