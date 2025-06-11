use clap::Parser;

/// Word suggestion utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show debug output
    #[arg(long)]
    debug: bool,
    /// List of letters (e.g. "ENOT..." or "E N O T ...")
    #[arg(long)]
    letters: String,
    /// Optional path to word list (default: cain.txt)
    #[arg(long)]
    wordlist: Option<String>,
}

fn main() {
    let args = Args::parse();
    word_player_shared::init_tracing(args.debug);
    // ...existing code for word suggestion will go here...
    println!("word-suggest started with args: {:?}", args);
}
