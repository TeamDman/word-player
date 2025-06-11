//! Shared helpers for word-player binaries

pub fn init_tracing(debug: bool) {
    use tracing_subscriber::EnvFilter;
    let filter = if debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_writer(std::io::stderr)
        .init();
}
