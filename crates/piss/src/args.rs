use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct GlobalArgs {
    /// Enable debug logging
    #[arg(long, global = true, default_value = "false")]
    pub debug: bool,
}

#[derive(Debug, Parser)]
#[command(name = "pee", bin_name = "pee", version= env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[command(flatten)]
    pub global: GlobalArgs,
    pub image_path: PathBuf,
}
