// File I/O operations and platform abstraction

use anyhow::Result;
use std::path::Path;

/// Validate that a file exists and is readable
pub fn validate_input_file(path: &str) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        anyhow::bail!("Input file does not exist: {}", path.display());
    }

    if !path.is_file() {
        anyhow::bail!("Input path is not a file: {}", path.display());
    }

    // TODO: Add GIF format validation
    // Check file magic bytes or extension

    Ok(())
}

/// Validate that the output path is writable
pub fn validate_output_path(path: &str) -> Result<()> {
    let path = Path::new(path);

    // Check if parent directory exists or can be created
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            anyhow::bail!("Output directory does not exist: {}", parent.display());
        }
    }

    // TODO: Check write permissions
    // Check if we can write to the location

    Ok(())
}

/// Get file size in bytes
pub fn get_file_size(path: &str) -> Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

/// Calculate compression percentage
pub fn calculate_compression_ratio(original: u64, compressed: u64) -> f64 {
    if original == 0 {
        return 0.0;
    }

    let reduction = original - compressed;
    (reduction as f64 / original as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_compression_ratio() {
        assert_eq!(calculate_compression_ratio(1000, 500), 50.0);
        assert_eq!(calculate_compression_ratio(1000, 900), 10.0);
        assert_eq!(calculate_compression_ratio(1000, 100), 90.0);
    }
}
