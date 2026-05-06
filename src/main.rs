use clap::Parser;
use dotenv::dotenv;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use transcribe_rs::audio;
use transcribe_rs::onnx::parakeet::{ParakeetModel, ParakeetParams, TimestampGranularity};
use transcribe_rs::onnx::Quantization;

/// Handy CLI: Transcribe WAV audio files using Handy's Parakeet model.
/// Input must be a 16kHz, mono, 16-bit PCM WAV file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input WAV file (16kHz, mono, 16-bit PCM)
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output text file
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,

    /// Path to the Parakeet model directory
    #[arg(short, long, env = "HANDY_MODEL_PATH")]
    model: PathBuf,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Load .env file (silently ignore if missing)
    let args = Args::parse();

    // Load the Parakeet model
    let mut model = ParakeetModel::load(&args.model, &Quantization::Int8)?;

    // Read WAV file
    let samples = audio::read_wav_samples(&args.input)?;

    // Transcribe
    let result = model.transcribe_with(
        &samples,
        &ParakeetParams {
            timestamp_granularity: Some(TimestampGranularity::Segment),
            ..Default::default()
        },
    )?;

    // Save to output file
    let mut file = File::create(&args.output)?;
    file.write_all(result.text.as_bytes())?;

    println!(
        "Transcription saved to {}",
        args.output.display()
    );

    Ok(())
}