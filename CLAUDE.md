# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Building
```bash
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Fix clippy warnings automatically
cargo clippy --fix
```

### Running the CLI
```bash
# Build and run
cargo run --release -- <command> [args]

# Example: Get GIF info
cargo run --release -- info input.gif

# Example: Speed adjustment
cargo run --release -- speed input.gif output.gif --factor 2.0

# Example: Compression
cargo run --release -- compress input.gif output.gif --percent 50

# Example: Resize/tune
cargo run --release -- tune input.gif output.gif --width 400 --height 300
```

### Test Generation
```bash
# Run the test generator binary
cargo run --bin test_gen
```

## Architecture Overview

GIF Toolkit is a Rust-based CLI tool for GIF manipulation. The architecture follows a modular design:

### Core Data Structures (src/core/mod.rs)
- **Frame**: Represents a single GIF frame with RGBA pixel data, dimensions, delay, and disposal method
- **Gif**: Container for GIF images with metadata (width, height, frames, global_palette, loop_count)

Key implementation details:
- Uses `gif` crate for encoding/decoding with RGBA color output
- Frames support partial dimensions (smaller than overall GIF dimensions)
- Disposal methods are preserved to prevent flicker in animated GIFs
- All pixel data is stored as RGBA (4 bytes per pixel)

### Module Structure
- `src/core/mod.rs` - Core GIF data structures and I/O operations
- `src/operations/` - Command implementations (speed, compress, tune, info)
- `src/cli/mod.rs` - CLI argument parsing using clap derive macros
- `src/io/mod.rs` - File I/O utilities
- `src/utils/mod.rs` - Shared helper functions
- `src/main.rs` - CLI entry point and command dispatch

### Operations
Each operation module implements specific GIF manipulations:
- **speed**: Adjusts frame delays for faster/slower playback; drops frames for extreme speedups (>4x)
- **compress**: Iterative compression using color quantization, lossy compression, and frame deduplication
- **tune**: Resizes GIF dimensions while maintaining aspect ratio if only one dimension specified
- **info**: Displays comprehensive GIF metadata including file size, dimensions, frame count, duration

### Key Dependencies
- `gif` - GIF encoding/decoding
- `image` - Image processing and manipulation
- `clap` - CLI argument parsing (derive API)
- `anyhow`/`thiserror` - Error handling
- `rayon` - Parallel processing
- `color_quant` - NeuQuant color quantization for compression
- `indicatif` - Progress bars for long operations

### Frame Handling
When processing GIFs with partial frames (frames smaller than GIF dimensions):
- Frame dimensions are preserved in `frame.width` and `frame.height`
- GIF dimensions are stored separately in `gif.width` and `gif.height`
- When encoding, smaller frames are centered using `gif_frame.top` and `gif_frame.left`

### Transparency and Disposal
- Transparency is tracked per-frame with the `transparent` bool flag
- Disposal methods (from gif crate) control how frames are rendered
- Default disposal is `DisposalMethod::Keep` to prevent artifacts

## Testing Strategy

Tests are co-located with implementation code in `#[cfg(test)]` modules. Key test patterns:
- Unit tests for individual functions (Frame::new, Gif::new, etc.)
- Tests verify data structure integrity (dimensions, delays, frame counts)
- Tests use in-memory data rather than external files where possible

Run `cargo test` to execute all tests including those in `src/core/mod.rs`.

## Performance Considerations

- Time complexity: O(n) for most operations where n = number of frames
- Memory usage: Typically 3-5x of input file size
- Compression uses iterative algorithm with early termination when target size reached
- Parallel processing via rayon for frame operations

## Release Profile

The Cargo.toml uses optimized release settings:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

This produces small, highly optimized binaries at the cost of longer compile times.
