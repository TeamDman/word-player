mod args;
mod tracing_setup;

use crate::args::Args;
use crate::tracing_setup::setup_tracing;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use clap::Parser;
use color_eyre::eyre::Result;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::images::Image;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::info;

const MODEL: &str = "gemma3:12b";

// Helper function to load an image from a file path
fn load_image_from_path(image_path: &PathBuf) -> Result<Image, color_eyre::eyre::Error> {
    info!(?image_path, "Loading image from path");
    let image_bytes = fs::read(image_path)?;
    let base64_image = STANDARD.encode(&image_bytes);
    Ok(Image::from_base64(&base64_image))
}

// Helper function to get a String from an Image and &str prompt
async fn get_ollama_response(
    image: Image,
    prompt: &str,
) -> Result<String, color_eyre::eyre::Error> {
    let request = GenerationRequest::new(MODEL.to_string(), prompt.to_string()).add_image(image);
    let ollama = Ollama::default();
    let response = ollama.generate(request).await?;
    Ok(response.response)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    setup_tracing(&args.global, std::io::stderr)?;
    info!(
        ?args,
        "Starting Picture Interrogation and Stochastic Semantics"
    );

    let prompts = include_str!("prompts.md");
    let prompts = prompts
        .split('#')
        .filter_map(|content| {
            let Some((name, body)) = content.split_once('\n') else {
                return None;
            };
            Some((name.trim().to_string(), body.trim().to_string()))
        })
        .collect::<HashMap<_, _>>();

    let prompt = prompts["use"].as_str();
    info!(?prompt, "Using prompt");

    let image = load_image_from_path(&args.image_path)?;
    let ollama_response = get_ollama_response(image, prompt).await?;

    println!("{}", ollama_response);

    Ok(())
}
