use base64::engine::general_purpose;
use base64::Engine;
use conet::generate_tts;
use hound::WavSpec;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
  let spec = WavSpec {
    channels: 1,
    sample_rate: 24_000,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let wav_data = generate_tts("Leonskidev".to_owned()).await;

  let mut file = File::create("audio/control.wav").unwrap();
  file
    .write_all(
      general_purpose::STANDARD
        .decode(wav_data)
        .unwrap()
        .as_slice(),
    )
    .unwrap();

  let sample_rate = 8000;

  let output_spec = WavSpec {
    channels: 1,
    sample_rate: 24_000,
    bits_per_sample: 8,
    sample_format: hound::SampleFormat::Int,
  };
  let downsample_ratio = spec.sample_rate / sample_rate;

  let mut reader = hound::WavReader::open("audio/control.wav").unwrap();
  let mut writer =
    hound::WavWriter::create("audio/downsampled.wav", output_spec).unwrap();

  let samples = reader.samples::<i32>().map(|s| s.unwrap());
  for sample in samples {
    writer
      .write_sample(
        sample >> (spec.bits_per_sample - output_spec.bits_per_sample),
      )
      .unwrap();
  }
}
