// GIF Toolkit Library

// Public modules
pub mod cli;
pub mod core;
pub mod io;
pub mod operations;
pub mod utils;

// Re-exports
pub use cli::{Args, Commands};
pub use core::Gif;
