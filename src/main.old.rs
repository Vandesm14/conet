#![feature(iter_intersperse)]

use core::panic;
use std::time::Instant;

use clap::Parser;
use conet::Tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use spellabet::{PhoneticConverter, SpellingAlphabet};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Secret message to encode
  #[arg(required = true)]
  message: String,

  /// Encoding method to use for secret message
  #[arg(long, short)]
  encoding: Option<String>,

  /// Disables the cache
  #[arg(long, short)]
  disable_cache: bool,

  /// Output file
  #[arg(long, short)]
  output: Option<String>,

  /// Disable random per-chunk voice selection
  #[arg(long)]
  no_random: bool,

  /// Beginning preamble message
  #[arg(long)]
  preamble: Option<String>,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let encoding = &args.encoding;
  let disable_cache = &args.disable_cache;
  let message = &args.message;
  let output = &args.output;
  let no_random = &args.no_random;
  let preamble = &args.preamble;

  let start_time = Instant::now();
  let mut tts = Tts::new();

  if *disable_cache {
    tts.without_cache();
  }

  if *no_random {
    tts.without_randomness();
  }

  let preamble = match preamble {
    Some(preamble) => preamble,
    None => "This is an automated broadcast. Please listen carefully.",
  };

  // Create initial preamble
  let mut samples = vec![];

  no_encoding(preamble, &mut samples, &mut tts).await;

  // Long pause between preamble and secret phrase
  if !preamble.is_empty() {
    samples.extend([0.0f32; 24_000]);
  }

  match encoding {
    Some(encoding) => match encoding.as_str() {
      "ascii" => ascii_encoding(message, &mut samples, &mut tts).await,
      "phonetic" => phonetic_encoding(message, &mut samples, &mut tts).await,
      "words" => words_encoding(message, &mut samples, &mut tts).await,
      _ => panic!("Invalid encoding method: {}", encoding),
    },
    None => no_encoding(message, &mut samples, &mut tts).await,
  }

  let end_time = Instant::now();

  println!(
    "Generated {} samples in {}ms",
    samples.len(),
    end_time.duration_since(start_time).as_millis().to_string()
  );

  let default_path = "/tmp/conet/audio.wav";
  let output = match output {
    Some(output) => output,
    None => default_path,
  };

  // Save audio file
  save_audio_file(&mut samples, output);
}

fn save_audio_file(samples: &mut [f32], path: &str) {
  let spec = WavSpec {
    channels: 1,
    sample_rate: 24_000,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };

  let output_spec = WavSpec {
    channels: 1,
    sample_rate: 8_000,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };

  let mut writer = hound::WavWriter::create(path, output_spec).unwrap();

  lowpass_filter(samples, 24_000.0, 8_000.0);

  let downsampling_factor =
    (spec.sample_rate / output_spec.sample_rate) as usize;

  // Downsample to 8kHz
  for sample in samples
    .iter()
    .skip(downsampling_factor - 1)
    .step_by(downsampling_factor)
    .copied()
  {
    writer
      // .write_sample((sample as i32).clamp(0, 2i32.pow(10)) << 3)
      .write_sample(sample as i32)
      .unwrap();
  }
}
