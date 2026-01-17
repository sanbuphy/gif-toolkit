use anyhow::Result;
use clap::Parser;
use gif_toolkit::cli::{Args, Commands};
use gif_toolkit::operations::{compress, info, speed, tune};

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse command-line arguments
    let args = Args::parse();

    // Execute the appropriate command
    match args.command {
        Commands::Speed {
            input,
            output,
            factor,
        } => {
            println!("Adjusting GIF speed...");
            speed::run(&input, &output, factor)?;
            println!("Speed adjustment complete!");
            println!("Output: {}", output);
        }
        Commands::Compress {
            input,
            output,
            percent,
        } => {
            println!("Compressing GIF...");
            compress::run(&input, &output, percent)?;
            println!("Compression complete!");
            println!("Output: {}", output);
        }
        Commands::Tune {
            input,
            output,
            width,
            height,
        } => {
            println!("Tuning GIF parameters...");
            tune::run(&input, &output, width, height)?;
            println!("Parameter tuning complete!");
            println!("Output: {}", output);
        }
        Commands::Info { input } => {
            info::run(&input)?;
        }
    }

    Ok(())
}
