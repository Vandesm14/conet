#![feature(iter_intersperse)]

use conet::TTS;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;

#[tokio::main]
async fn main() {
  let mut tts = TTS::new();
  let secret_phrase = "Hello, World!";

  // Create initial preamble
  let mut samples = tts
    .generate(
      "This is an automated broadcast. Please listen carefully.",
      Some("en-US-Standard-F"),
    )
    .await;

  // Long pause between preamble and secret phrase
  samples.extend([0.0f32; 24_000]);

  ascii_encoding(secret_phrase, &mut samples, &mut tts).await;

  // Save audio file
  save_audio_file(&mut samples);
}

async fn ascii_encoding(string: &str, samples: &mut Vec<f32>, tts: &mut TTS) {
  // Convert secret phrase into ascii codes (String of numbers)
  let words = string
    .as_bytes()
    .iter()
    // Convert each byte into a string, padded with 0s
    .map(|b| format!("{:0>3}", b))
    .reduce(|a, b| a + &b)
    .unwrap();

  // Split the ascii string into chars
  let words = words.chars().collect::<Vec<_>>();

  // Split into chunks of 5
  let words = words.chunks(5);

  // Run throuch each chunk and TTS samples
  for word in words {
    for char in word {
      let more_samples = tts.generate(&char.to_string(), None).await;
      samples.extend(more_samples);

      // Short pause between letters
      samples.extend([0.0f32; 4_000]);
    }

    // Long pause between words
    samples.extend([0.0f32; 10_000]);
  }
}

fn save_audio_file(samples: &mut [f32]) {
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

  let mut writer =
    hound::WavWriter::create("/tmp/conet/audio.wav", output_spec).unwrap();

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
