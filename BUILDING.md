# Building GIF Toolkit

This document provides instructions for building GIF Toolkit from source.

## Prerequisites

### Common Requirements
- Git
- C compiler and build tools
- Rust toolchain (latest stable)

### Platform-Specific Requirements

#### Windows
- [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [Rustup](https://rustup.rs/)

#### macOS
- Xcode Command Line Tools: `xcode-select --install`
- [Rustup](https://rustup.rs/)

#### Linux
- GCC/Clang
- pkg-config
- Development libraries:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install build-essential pkg-config libssl-dev

  # Fedora
  sudo dnf install gcc pkg-config openssl-devel

  # Arch Linux
  sudo pacman -S base-devel pkg-config openssl
  ```
- [Rustup](https://rustup.rs/)

## Installation Steps

### 1. Install Rust

If you don't have Rust installed, use rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install the latest stable Rust toolchain.

### 2. Clone the Repository

```bash
git clone https://github.com/yourusername/gif-toolkit.git
cd gif-toolkit
```

### 3. Build the Project

```bash
cargo build --release
```

The compiled binary will be located at:
- **Windows**: `target/release/gif-toolkit.exe`
- **macOS/Linux**: `target/release/gif-toolkit`

### 4. Verify Installation

```bash
./target/release/gif-toolkit --version
```

## Development Build

For development with debugging symbols:

```bash
cargo build
```

For faster incremental builds:

```bash
cargo check
```

## Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

## Cross-Compilation

### Building for Different Targets

Install the target toolchain:

```bash
rustup target add x86_64-pc-windows-gnu        # Windows from Linux/macOS
rustup target add x86_64-unknown-linux-musl    # Linux from macOS
rustup target add aarch64-apple-darwin         # Apple Silicon (ARM64)
```

Build for the target:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Using Docker for Linux Builds

```bash
docker build -t gif-toolkit-builder -f docker/Dockerfile .
docker run --rm -v $(pwd):/app gif-toolkit-builder
```

## Build Options

### Release Profile

For maximum performance:

```bash
cargo build --release
```

### Custom Optimization Level

Edit `Cargo.toml` or use environment variables:

```bash
CARGO_PROFILE_RELEASE_OPT_LEVEL=3 cargo build --release
```

### Strip Debug Information

Reduce binary size:

```bash
# Linux/macOS
strip target/release/gif-toolkit

# Windows (using strip from GNU binutils)
strip.exe target/release/gif-toolkit.exe
```

## Packaging

### Creating Release Archives

```bash
# Linux/macOS
tar -czf gif-toolkit-$(git describe --tags)-linux-x64.tar.gz -C target/release gif-toolkit

# Windows
zip -r gif-toolkit-$(git describe --tags)-windows-x64.zip target/release/gif-toolkit.exe
```

### Creating Installers

#### Windows (MSI)
Use tools like WiX or NSIS to create installers.

#### macOS (DMG)
```bash
hdiutil create -volname "GIF Toolkit" -srcfolder target/release/gif-toolkit -ov -format UDZO gif-toolkit.dmg
```

#### Linux (DEB/RPM)
Use scripts in `packaging/` directory or tools like `cargo-deb`.

## CI/CD

The project uses GitHub Actions for automated building. See `.github/workflows/build.yml` for the build configuration.

## Troubleshooting

### Build Fails with "Linking with cc failed"

**Solution**: Install C compiler and build tools for your platform.

### Out of Memory During Build

**Solution**: Limit the number of parallel jobs:
```bash
CARGO_BUILD_JOBS=2 cargo build --release
```

### OpenSSL Linking Errors

**Solution**: Install OpenSSL development libraries (see platform-specific requirements).

### Permission Denied When Running Binary

**Solution**: Make the binary executable:
```bash
chmod +x target/release/gif-toolkit
```

## Performance Optimization

### Profile-Guided Optimization (PGO)

For even better performance:

```bash
# Build with instrumentation
cargo build --release

# Run representative workloads
./target/release/gif-toolkit bench

# Rebuild with profile data
cargo build --release -- -C profile-use=default.profdata
```

### LTO (Link-Time Optimization)

Enable in `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
```

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

## Getting Help

If you encounter issues building GIF Toolkit:
1. Check existing [GitHub Issues](https://github.com/yourusername/gif-toolkit/issues)
2. Create a new issue with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce
