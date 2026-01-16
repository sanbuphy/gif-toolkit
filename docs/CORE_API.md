# GIF Toolkit - Core API Documentation

## Overview

The core module (`src/core/mod.rs`) provides the fundamental data structures and file I/O operations for working with GIF files in Rust.

## Core Structures

### Frame

Represents a single frame in a GIF animation.

```rust
pub struct Frame {
    /// RGBA pixel data (4 bytes per pixel: R, G, B, A)
    pub data: Vec<u8>,
    /// Frame width
    pub width: u16,
    /// Frame height
    pub height: u16,
    /// Delay time in 10ms units (e.g., 10 = 100ms)
    pub delay: u16,
    /// Whether this frame has transparency
    pub transparent: bool,
}
```

#### Methods

- **`new(width: u16, height: u16) -> Self`**: Creates a new empty frame with black pixels
- **`from_rgba(data: Vec<u8>, width: u16, height: u16) -> Self`**: Creates a frame from RGBA pixel data
- **`to_image_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>>`**: Converts frame to image buffer for manipulation
- **`update_from_image_buffer(&mut self, buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>)`**: Updates frame from image buffer

#### Example

```rust
use gif_toolkit::core::Frame;

// Create a new 100x100 frame
let mut frame = Frame::new(100, 100);

// Modify the first pixel to red
frame.data[0] = 255; // R
frame.data[1] = 0;   // G
frame.data[2] = 0;   // B
frame.data[3] = 255; // A

// Set delay to 200ms (20 units of 10ms)
frame.delay = 20;
```

### Gif

Represents a complete GIF image with all frames and metadata.

```rust
pub struct Gif {
    /// All frames in the GIF
    pub frames: Vec<Frame>,
    /// GIF width
    pub width: u16,
    /// GIF height
    pub height: u16,
    /// Global color palette (optional, each entry is RGB)
    pub global_palette: Option<Vec<[u8; 3]>>,
    /// Loop count (0 = infinite loop)
    pub loop_count: u16,
}
```

#### Methods

- **`new() -> Self`**: Creates a new empty GIF
- **`from_file(path: &str) -> Result<Self>`**: Loads a GIF from file
- **`to_file(&self, path: &str) -> Result<()>`**: Saves the GIF to file
- **`add_frame(&mut self, frame: Frame)`**: Adds a frame to the GIF
- **`frame_count(&self) -> usize`**: Returns the number of frames
- **`total_duration(&self) -> u32`**: Returns total duration in 10ms units

## Usage Examples

### Loading a GIF File

```rust
use gif_toolkit::core::Gif;

fn main() -> anyhow::Result<()> {
    // Load a GIF from file
    let gif = Gif::from_file("input.gif")?;

    println!("Dimensions: {}x{}", gif.width, gif.height);
    println!("Frames: {}", gif.frame_count());
    println!("Duration: {} ms", gif.total_duration() * 10);

    Ok(())
}
```

### Creating a GIF Programmatically

```rust
use gif_toolkit::core::{Frame, Gif};

fn main() -> anyhow::Result<()> {
    let mut gif = Gif::new();

    // Create 3 frames with different colors
    for color in &[[255, 0, 0], [0, 255, 0], [0, 0, 255]] {
        let mut frame = Frame::new(100, 100);

        // Fill frame with solid color
        for pixel in frame.data.chunks_exact_mut(4) {
            pixel[0] = color[0];
            pixel[1] = color[1];
            pixel[2] = color[2];
            pixel[3] = 255; // Opaque
        }

        frame.delay = 10; // 100ms
        gif.add_frame(frame);
    }

    // Save the GIF
    gif.to_file("output.gif")?;

    Ok(())
}
```

### Modifying an Existing GIF

```rust
use gif_toolkit::core::Gif;

fn main() -> anyhow::Result<()> {
    // Load GIF
    let mut gif = Gif::from_file("input.gif")?;

    // Modify each frame
    for frame in &mut gif.frames {
        // Invert colors
        for pixel in frame.data.chunks_exact_mut(4) {
            pixel[0] = 255 - pixel[0]; // R
            pixel[1] = 255 - pixel[1]; // G
            pixel[2] = 255 - pixel[2]; // B
        }

        // Double the delay
        frame.delay *= 2;
    }

    // Save modified GIF
    gif.to_file("output.gif")?;

    Ok(())
}
```

### Accessing Frame Data

```rust
use gif_toolkit::core::Gif;

fn main() -> anyhow::Result<()> {
    let gif = Gif::from_file("animation.gif")?;

    for (i, frame) in gif.frames.iter().enumerate() {
        println!("Frame {}: {}x{}, delay: {}ms",
            i + 1,
            frame.width,
            frame.height,
            frame.delay * 10
        );

        // Access pixel data (RGBA format)
        for (y, row) in (0..frame.height).step_by(4) {
            for x in 0..frame.width {
                let idx = ((y * frame.width + x) as usize) * 4;
                let r = frame.data[idx];
                let g = frame.data[idx + 1];
                let b = frame.data[idx + 2];
                let a = frame.data[idx + 3];

                // Process pixel...
            }
        }
    }

    Ok(())
}
```

## Important Notes

### Delay Time Units

GIF delay times are stored in units of 10ms (centiseconds):
- `delay = 10` means 100ms (0.1 seconds)
- `delay = 50` means 500ms (0.5 seconds)
- Minimum delay is 1 (10ms)

### RGBA Format

Frame pixel data is stored in RGBA format:
- 4 bytes per pixel (Red, Green, Blue, Alpha)
- Total size: `width * height * 4` bytes
- Values range from 0-255 for each channel

### Loop Count

The `loop_count` field controls animation looping:
- `0` = Infinite loop (default)
- `1` = Play once
- `N` = Play N times

### Color Palettes

- `global_palette` stores the GIF's global color table if present
- Each palette entry is an RGB triplet `[u8; 3]`
- When saving, the encoder will use this palette if provided
- If no palette is provided, the encoder creates one automatically

## Error Handling

All file I/O operations return `anyhow::Result<T>` for easy error propagation:

```rust
use anyhow::Result;

fn process_gif() -> Result<()> {
    let gif = Gif::from_file("input.gif")?;
    gif.to_file("output.gif")?;
    Ok(())
}
```

## Testing

Run the included tests:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test integration_test
```

## Performance Considerations

- Frame data is stored as `Vec<u8>` for efficient serialization
- Large GIF files may consume significant memory (all frames loaded at once)
- For processing large GIFs, consider processing frames iteratively
- The `to_image_buffer()` method is useful for image processing operations

## Future Enhancements

Potential improvements to the core module:

1. **Lazy loading**: Load frames on-demand instead of all at once
2. **Frame iteration**: Implement `Iterator` for frame access
3. **Color quantization**: Better palette generation for saving
4. **Optimization**: Reduce memory usage for large GIFs
5. **Caching**: Cache encoded data to avoid re-encoding
