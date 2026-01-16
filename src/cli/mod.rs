use clap::{Parser, Subcommand};

/// GIF Toolkit - A powerful cross-platform GIF optimization and manipulation toolkit
#[derive(Parser, Debug)]
#[command(name = "gif-toolkit")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Optimize and manipulate GIF images", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adjust GIF playback speed
    Speed {
        /// Input GIF file path
        #[arg(short, long)]
        input: String,

        /// Output GIF file path
        #[arg(short, long)]
        output: String,

        /// Speed factor (e.g., 2.0 for 2x faster, 0.5 for 2x slower)
        #[arg(short, long)]
        factor: f64,
    },

    /// Compress GIF file size
    Compress {
        /// Input GIF file path
        #[arg(short, long)]
        input: String,

        /// Output GIF file path
        #[arg(short, long)]
        output: String,

        /// Compression percentage (1-99)
        #[arg(short, long)]
        percent: u8,
    },

    /// Tune GIF parameters (resize, crop, etc.)
    Tune {
        /// Input GIF file path
        #[arg(short, long)]
        input: String,

        /// Output GIF file path
        #[arg(short, long)]
        output: String,

        /// New width in pixels
        #[arg(short, long)]
        width: Option<u32>,

        /// New height in pixels
        #[arg(short, long)]
        height: Option<u32>,
    },

    /// Display GIF information
    Info {
        /// Input GIF file path
        #[arg(short, long)]
        input: String,
    },
}
