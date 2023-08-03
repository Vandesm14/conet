#![feature(iter_intersperse)]

use base64::{engine::general_purpose, Engine};
use conet::generate_tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use spellabet::{PhoneticConverter, SpellingAlphabet};

enum Word {
  Word(String),
  Capital(String),
}

#[tokio::main]
async fn main() {
  let secret_phrase = "Hello, World!";

  let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);

  // Create initial preamble
  let mut samples = generate_tts(
    "This is an automated broadcast. Please listen carefully.",
    "en-US-Standard-A",
  )
  .await;

  // Convert the secret phrase into base64
  let string = general_purpose::STANDARD.encode(secret_phrase);

  // Convert the base64 string into a sequence of words ("capital <word>" or "<word>")
  let words = string.chars().map(|c| match c.is_ascii_uppercase() {
    true => {
      Word::Capital(converter.convert(c.to_lowercase().to_string().as_str()))
    }
    false => Word::Word(converter.convert(c.to_string().as_str())),
  });

  // Run throuch eagh word and prepend "capital" if it's a capital letter
  for word in words {
    match word {
      Word::Word(word) => {
        let more_samples = generate_tts(&word, "en-US-Standard-F").await;
        samples.extend(more_samples);
        samples.extend([0.0f32; 8_000]);
      }
      Word::Capital(word) => {
        let more_samples = generate_tts("capital", "en-US-Standard-F").await;
        samples.extend(more_samples);

        let more_samples = generate_tts(&word, "en-US-Standard-F").await;
        samples.extend(more_samples);
        samples.extend([0.0f32; 8_000]);
      }
    }
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
