// Core GIF processing functionality

use anyhow::{Context, Result};
use gif::{Encoder, Frame as GifFrame, Repeat};
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// Represents a single frame in a GIF image
#[derive(Debug, Clone)]
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

impl Frame {
    /// Create a new empty frame
    pub fn new(width: u16, height: u16) -> Self {
        let pixel_count = (width as usize) * (height as usize);
        Self {
            data: vec![0; pixel_count * 4],
            width,
            height,
            delay: 10, // Default 100ms delay
            transparent: false,
        }
    }

    /// Create a frame from RGBA pixel data
    pub fn from_rgba(data: Vec<u8>, width: u16, height: u16) -> Self {
        let expected_len = (width as usize) * (height as usize) * 4;
        assert_eq!(data.len(), expected_len, "RGBA data length mismatch");

        Self {
            data,
            width,
            height,
            delay: 10,
            transparent: false,
        }
    }

    /// Convert frame data to ImageBuffer for manipulation
    pub fn to_image_buffer(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image::ImageBuffer::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.clone()
        ).expect("Failed to create ImageBuffer from frame data")
    }

    /// Update frame data from ImageBuffer
    pub fn update_from_image_buffer(&mut self, buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
        let (width, height) = buffer.dimensions();
        self.width = width as u16;
        self.height = height as u16;
        self.data = buffer.as_raw().clone();
    }
}

/// Represents a GIF image with all its frames and metadata
#[derive(Debug, Clone)]
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

impl Gif {
    /// Create a new empty GIF
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            width: 0,
            height: 0,
            global_palette: None,
            loop_count: 0, // Infinite loop by default
        }
    }

    /// Load a GIF from a file
    pub fn from_file(path: &str) -> Result<Self> {
        // Open the file
        let file = File::open(path)
            .with_context(|| format!("Failed to open GIF file: {}", path))?;
        let mut reader = BufReader::new(file);

        // Configure decoder to output RGBA format
        let mut decoder_options = gif::DecodeOptions::new();
        decoder_options.set_color_output(gif::ColorOutput::RGBA);

        // Create decoder and read info
        let mut decoder = decoder_options.read_info(&mut reader)
            .with_context(|| format!("Failed to read GIF header from: {}", path))?;

        // Get dimensions
        let width = decoder.width();
        let height = decoder.height();

        // Read global palette if present
        let global_palette = decoder.global_palette().map(|palette| {
            palette.chunks_exact(3)
                .map(|chunk| {
                    let mut rgb = [0u8; 3];
                    rgb.copy_from_slice(chunk);
                    rgb
                })
                .collect()
        });

        // Collect all frames
        let mut frames = Vec::new();

        while let Some(frame_info) = decoder.read_next_frame()
            .with_context(|| format!("Failed to read frame from: {}", path))?
        {
            // Get RGBA data from the frame buffer
            let data = frame_info.buffer.to_vec();

            // Ensure data is in RGBA format
            assert_eq!(data.len() % 4, 0, "Frame data should be RGBA");

            let frame = Frame {
                data,
                width,
                height,
                delay: frame_info.delay.max(1), // Ensure minimum delay of 1 (10ms)
                transparent: frame_info.transparent.is_some(),
            };

            frames.push(frame);
        }

        Ok(Self {
            frames,
            width,
            height,
            global_palette,
            loop_count: 0, // Default to infinite loop
        })
    }

    /// Save the GIF to a file
    pub fn to_file(&self, path: &str) -> Result<()> {
        // Create output file
        let file = File::create(path)
            .with_context(|| format!("Failed to create GIF file: {}", path))?;
        let writer = BufWriter::new(file);

        // Prepare global palette (empty if none)
        let global_palette: Vec<u8> = if let Some(palette) = &self.global_palette {
            palette.iter()
                .flat_map(|rgb| rgb.iter().copied())
                .collect()
        } else {
            Vec::new()
        };

        // Create encoder
        let mut encoder = Encoder::new(writer, self.width, self.height, &global_palette)
            .with_context(|| format!("Failed to create GIF encoder for: {}", path))?;

        // Set loop count (0 = infinite)
        if self.loop_count == 0 {
            encoder.set_repeat(Repeat::Infinite)
                .context("Failed to set loop count")?;
        } else {
            encoder.set_repeat(Repeat::Finite(self.loop_count))
                .context("Failed to set loop count")?;
        }

        // Write each frame
        for frame in &self.frames {
            // Create GIF frame from RGBA data
            let mut gif_frame = GifFrame::from_rgba(frame.width, frame.height, &mut frame.data.clone());

            // Set delay and transparency
            gif_frame.delay = frame.delay.max(1); // Ensure minimum delay

            if frame.transparent {
                gif_frame.transparent = Some(0);
            }

            encoder.write_frame(&gif_frame)
                .with_context(|| format!("Failed to write frame to: {}", path))?;
        }

        Ok(())
    }

    /// Add a new frame to the GIF
    pub fn add_frame(&mut self, frame: Frame) {
        // Update dimensions if this is the first frame
        if self.frames.is_empty() {
            self.width = frame.width;
            self.height = frame.height;
        }
        self.frames.push(frame);
    }

    /// Get the number of frames
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Get total duration (in 10ms units)
    pub fn total_duration(&self) -> u32 {
        self.frames.iter()
            .map(|f| f.delay as u32)
            .sum()
    }
}

impl Default for Gif {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_new() {
        let frame = Frame::new(10, 10);
        assert_eq!(frame.width, 10);
        assert_eq!(frame.height, 10);
        assert_eq!(frame.data.len(), 10 * 10 * 4);
        assert_eq!(frame.delay, 10);
        assert!(!frame.transparent);
    }

    #[test]
    fn test_frame_from_rgba() {
        let data = vec![255u8; 100 * 100 * 4];
        let frame = Frame::from_rgba(data, 100, 100);
        assert_eq!(frame.width, 100);
        assert_eq!(frame.height, 100);
        assert_eq!(frame.data.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_gif_new() {
        let gif = Gif::new();
        assert_eq!(gif.width, 0);
        assert_eq!(gif.height, 0);
        assert_eq!(gif.frames.len(), 0);
        assert_eq!(gif.loop_count, 0);
    }

    #[test]
    fn test_gif_add_frame() {
        let mut gif = Gif::new();
        let frame = Frame::new(50, 50);
        gif.add_frame(frame);

        assert_eq!(gif.width, 50);
        assert_eq!(gif.height, 50);
        assert_eq!(gif.frame_count(), 1);
    }

    #[test]
    fn test_gif_total_duration() {
        let mut gif = Gif::new();
        let mut frame1 = Frame::new(10, 10);
        frame1.delay = 20;
        let mut frame2 = Frame::new(10, 10);
        frame2.delay = 30;

        gif.add_frame(frame1);
        gif.add_frame(frame2);

        assert_eq!(gif.total_duration(), 50);
    }
}
