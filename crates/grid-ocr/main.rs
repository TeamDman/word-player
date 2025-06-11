use clap::Parser;

/// Grid OCR utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show debug output
    #[arg(long)]
    debug: bool,
    /// Path to input image
    #[arg(long)]
    image: String,
}

fn main() {
    let args = Args::parse();
    word_player_shared::init_tracing(args.debug);
    // ...existing code for OCR and LLM interaction will go here...
    println!("grid-ocr started with args: {:?}", args);
}
