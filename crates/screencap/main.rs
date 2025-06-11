use clap::Parser;

/// Screen capture utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show debug output
    #[arg(long)]
    debug: bool,
    /// X coordinate of region
    #[arg(long)]
    x: u32,
    /// Y coordinate of region
    #[arg(long)]
    y: u32,
    /// Width of region
    #[arg(long)]
    width: u32,
    /// Height of region
    #[arg(long)]
    height: u32,
    /// Output path for APNG
    #[arg(long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    word_player_shared::init_tracing(args.debug);
    // ...existing code for screen capture will go here...
    println!("screencap started with args: {:?}", args);
}
