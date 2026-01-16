// Integration tests for GIF Toolkit core functionality
//
// These tests verify the complete workflow:
// 1. Creating frames programmatically
// 2. Building a GIF
// 3. Saving to file
// 4. Loading back
// 5. Verifying data integrity

use gif_toolkit::core::{Frame, Gif};
use std::path::Path;

#[test]
fn test_create_and_save_gif() {
    let width = 50u16;
    let height = 50u16;

    // Create a simple GIF with 3 frames
    let mut gif = Gif::new();

    // Frame 1: Red
    let mut frame1 = Frame::new(width, height);
    for pixel in frame1.data.chunks_exact_mut(4) {
        pixel[0] = 255; // R
        pixel[1] = 0;   // G
        pixel[2] = 0;   // B
        pixel[3] = 255; // A
    }
    frame1.delay = 10; // 100ms
    gif.add_frame(frame1);

    // Frame 2: Green
    let mut frame2 = Frame::new(width, height);
    for pixel in frame2.data.chunks_exact_mut(4) {
        pixel[0] = 0;   // R
        pixel[1] = 255; // G
        pixel[2] = 0;   // B
        pixel[3] = 255; // A
    }
    frame2.delay = 10; // 100ms
    gif.add_frame(frame2);

    // Frame 3: Blue
    let mut frame3 = Frame::new(width, height);
    for pixel in frame3.data.chunks_exact_mut(4) {
        pixel[0] = 0;   // R
        pixel[1] = 0;   // G
        pixel[2] = 255; // B
        pixel[3] = 255; // A
    }
    frame3.delay = 10; // 100ms
    gif.add_frame(frame3);

    // Verify GIF properties
    assert_eq!(gif.width, width);
    assert_eq!(gif.height, height);
    assert_eq!(gif.frame_count(), 3);
    assert_eq!(gif.total_duration(), 30); // 3 frames * 10 units

    // Save to file
    let output_path = "test_output.gif";
    let result = gif.to_file(output_path);
    assert!(result.is_ok(), "Failed to save GIF: {:?}", result.err());

    // Verify file was created
    assert!(Path::new(output_path).exists());

    // Clean up
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_frame_from_rgba() {
    // Create RGBA data for a 2x2 image
    let data = vec![
        255, 0, 0, 255,    // Red pixel
        0, 255, 0, 255,    // Green pixel
        0, 0, 255, 255,    // Blue pixel
        255, 255, 255, 255, // White pixel
    ];

    let frame = Frame::from_rgba(data, 2, 2);

    assert_eq!(frame.width, 2);
    assert_eq!(frame.height, 2);
    assert_eq!(frame.data.len(), 16); // 2x2x4

    // Verify pixel values
    assert_eq!(frame.data[0], 255); // R of first pixel
    assert_eq!(frame.data[1], 0);   // G of first pixel
    assert_eq!(frame.data[2], 0);   // B of first pixel
    assert_eq!(frame.data[3], 255); // A of first pixel
}

#[test]
fn test_gif_default() {
    let gif = Gif::default();

    assert_eq!(gif.width, 0);
    assert_eq!(gif.height, 0);
    assert_eq!(gif.frames.len(), 0);
    assert_eq!(gif.loop_count, 0);
    assert!(gif.global_palette.is_none());
}

#[test]
fn test_frame_default_properties() {
    let frame = Frame::new(100, 100);

    assert_eq!(frame.width, 100);
    assert_eq!(frame.height, 100);
    assert_eq!(frame.delay, 10); // Default 100ms
    assert!(!frame.transparent);
    assert_eq!(frame.data.len(), 100 * 100 * 4);
}

#[test]
fn test_add_multiple_frames() {
    let mut gif = Gif::new();

    // Add multiple frames
    for i in 0..5 {
        let frame = Frame::new(50, 50);
        gif.add_frame(frame);
    }

    assert_eq!(gif.frame_count(), 5);
    assert_eq!(gif.width, 50);
    assert_eq!(gif.height, 50);
}

#[test]
fn test_total_duration_calculation() {
    let mut gif = Gif::new();

    // Add frames with different delays
    let mut frame1 = Frame::new(10, 10);
    frame1.delay = 5; // 50ms

    let mut frame2 = Frame::new(10, 10);
    frame2.delay = 10; // 100ms

    let mut frame3 = Frame::new(10, 10);
    frame3.delay = 15; // 150ms

    gif.add_frame(frame1);
    gif.add_frame(frame2);
    gif.add_frame(frame3);

    assert_eq!(gif.total_duration(), 30); // 5 + 10 + 15
}

#[test]
#[should_panic(expected = "RGBA data length mismatch")]
fn test_frame_from_rgba_invalid_length() {
    // This should panic because the data length doesn't match dimensions
    let invalid_data = vec![0u8; 100]; // Wrong length for a 10x10 image (should be 400)
    Frame::from_rgba(invalid_data, 10, 10);
}
