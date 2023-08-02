use base64::engine::general_purpose;
use base64::Engine;
use conet::generate_tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
  create_tts_file().await;

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

  let mut reader = hound::WavReader::open("audio/control.wav").unwrap();
  let mut writer =
    hound::WavWriter::create("audio/downsampled.wav", output_spec).unwrap();

  let mut samples = reader
    .samples::<i32>()
    .map(|s| s.unwrap())
    .map(|s| s as f32)
    .collect::<Vec<_>>();

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

async fn create_tts_file() {
  let wav_data = generate_tts(
    "This is an automated broadcast. Please listen carefully.".to_owned(),
    "en-US-Standard-F".to_owned(),
  )
  .await;

  let mut file = File::create("audio/control.wav").unwrap();
  file
    .write_all(
      general_purpose::STANDARD
        .decode(wav_data)
        .unwrap()
        .as_slice(),
    )
    .unwrap();
}
