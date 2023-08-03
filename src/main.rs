#![feature(iter_intersperse)]

use conet::generate_tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use spellabet::{PhoneticConverter, SpellingAlphabet};

#[tokio::main]
async fn main() {
  let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);

  let mut samples = generate_tts(
    "This is an automated broadcast. Please listen carefully.",
    "en-US-Standard-A",
  )
  .await;

  let string = converter.convert("https://google.com");
  let words = string.split(' ');

  for word in words {
    let more_samples = generate_tts(word, "en-US-Standard-F").await;
    samples.extend(more_samples);
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
