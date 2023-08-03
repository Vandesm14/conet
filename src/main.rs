#![feature(iter_intersperse)]

use conet::generate_tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;

#[tokio::main]
async fn main() {
  let secret_phrase = "Hello, World!";

  // Create initial preamble
  let mut samples = generate_tts(
    "This is an automated broadcast. Please listen carefully.",
    Some("en-US-Standard-F"),
  )
  .await;

  // Convert secret phrase into ascii codes (String of numbers)
  let words = secret_phrase
    .as_bytes()
    .iter()
    // Convert each byte into a string, padded with 0s
    .map(|b| format!("{:0>3}", b))
    .reduce(|a, b| a + &b)
    .unwrap();
  let words = words.split("").collect::<Vec<_>>();

  // Run throuch eagh word and prepend "capital" if it's a capital letter
  for word in words.chunks(5) {
    for char in word {
      let more_samples = generate_tts(char, None).await;
      samples.extend(more_samples);

      // Short pause between letters
      samples.extend([0.0f32; 4_000]);
    }

    // Long pause between words
    samples.extend([0.0f32; 10_000]);
  }

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

  lowpass_filter(&mut samples, 24_000.0, 8_000.0);

  let downsampling_factor =
    (spec.sample_rate / output_spec.sample_rate) as usize;

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
