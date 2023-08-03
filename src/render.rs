#![warn(missing_docs)]

use crate::Tts;
use async_recursion::async_recursion;
use core::fmt;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use spellabet::{PhoneticConverter, SpellingAlphabet};
use std::{
  fmt::{Display, Formatter},
  time::Instant,
};

#[derive(Clone, Copy, Debug)]
/// The voice model to use when rendering text to speech (Based on the `en-US-Standard-*` voices of [Google Cloud Text-to-Speech](https://cloud.google.com/text-to-speech/docs/voices))
#[allow(missing_docs)]
pub enum VoiceModel {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
}

impl Display for VoiceModel {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      VoiceModel::A => write!(f, "A"),
      VoiceModel::B => write!(f, "B"),
      VoiceModel::C => write!(f, "C"),
      VoiceModel::D => write!(f, "D"),
      VoiceModel::E => write!(f, "E"),
      VoiceModel::F => write!(f, "F"),
      VoiceModel::G => write!(f, "G"),
      VoiceModel::H => write!(f, "H"),
      VoiceModel::I => write!(f, "I"),
      VoiceModel::J => write!(f, "J"),
    }
  }
}

/// The encoding method to use when rendering text to speech
pub enum Encoding {
  /// Splits the text into separate words
  Words,

  /// Turns the text into ASCII numbers and splits them into chunks of 5 numbers
  Ascii,

  /// Turns the text into NATO phonetic alphabet words
  Phonetic,
}

/// Creates a speakable (TTS) clip
pub struct Speak<'a> {
  /// The text to speak
  pub text: &'a str,

  /// If None, a random voice will be selected
  pub voice: Option<VoiceModel>,

  /// If None, the no encoding will be used
  pub encoding: Option<Encoding>,
}

impl<'a> Speak<'a> {
  /// Creates a new speakable clip
  pub fn new(text: &'a str) -> Self {
    Self {
      encoding: None,
      text,
      voice: None,
    }
  }

  /// Sets the voice to use
  pub fn with_voice(mut self, voice: VoiceModel) -> Self {
    self.voice = Some(voice);
    self
  }

  /// Sets the encoding method to use
  pub fn with_encoding(mut self, encoding: Encoding) -> Self {
    self.encoding = Some(encoding);
    self
  }

  fn model(&self) -> Option<String> {
    let voice_model = self.voice;
    voice_model?;

    Some(voice_model.unwrap().to_string())
  }
}

/// Creates a pause in n milliseconds
pub struct Pause(pub u32);

/// An unrendered audio clip
pub enum Clip<'a> {
  /// A clip that speaks text
  Speak(Speak<'a>),

  /// A clip that pauses for a specified amount of time
  Pause(Pause),
}

impl<'a> From<Speak<'a>> for Clip<'a> {
  fn from(speak: Speak<'a>) -> Self {
    Clip::Speak(speak)
  }
}

impl From<Pause> for Clip<'_> {
  fn from(pause: Pause) -> Self {
    Clip::Pause(pause)
  }
}

impl<'a> Clip<'a> {
  #[async_recursion]
  async fn render(&self, samples: &mut Vec<f32>, tts: &mut Tts) {
    match self {
      Clip::Speak(speak) => {
        match speak.encoding {
          Some(Encoding::Words) => {
            // Split the secret phrase into words
            let words = speak.text.split_whitespace();

            // Run through each word and TTS samples
            for word in words {
              let more_samples = tts.generate(word, speak.model()).await;
              samples.extend(more_samples);
            }
          }
          Some(Encoding::Ascii) => {
            // Convert secret phrase into ascii codes (String of numbers)
            let words = speak
              .text
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
                let more_samples =
                  tts.generate(&char.to_string(), speak.model()).await;
                samples.extend(more_samples);

                // Short pause between letters
                Clip::from(Pause(160)).render(samples, tts).await;
              }

              // Long pause between words
              Clip::from(Pause(400)).render(samples, tts).await;
            }
          }
          Some(Encoding::Phonetic) => {
            {
              let converter = PhoneticConverter::new(&SpellingAlphabet::Nato);

              // Convert secret phrase into phonetic alphabet
              let string = converter.convert(speak.text);

              // Split into words
              let words = string.split_whitespace();

              // Run throuch each word and TTS samples
              for word in words {
                if word.to_lowercase().as_str() == "space" {
                  // Long pause between words
                  Clip::from(Pause(600)).render(samples, tts).await;
                  continue;
                }

                let more_samples = tts.generate(word, speak.model()).await;
                samples.extend(more_samples);

                // Short pause between words
                Clip::from(Pause(160)).render(samples, tts).await;
              }
            }
          }
          None => {
            let more_samples = tts.generate(speak.text, speak.model()).await;
            samples.extend(more_samples);
          }
        }
      }
      Clip::Pause(pause) => {
        samples.extend(vec![0.0f32; 24 * (pause.0 as usize)])
      }
    }
  }
}

/// Renders all clips and returns the WAV samples
pub async fn render_all<'a>(
  clips: impl Iterator<Item = Clip<'a>>,
  tts: &mut Tts,
) -> Vec<f32> {
  let start_time = Instant::now();
  let mut samples = vec![];

  let mut clip_count = 0;
  for clip in clips {
    clip.render(&mut samples, tts).await;
    clip_count += 1;
  }

  println!(
    "Rendered {} samples ({} clips) in {}ms",
    samples.len(),
    clip_count,
    start_time.elapsed().as_millis()
  );

  samples
}

/// The default file path for the rendered audio file
pub const DEFAULT_RENDER_PATH: &str = "/tmp/conetto/audio.wav";

/// Saves the samples to a WAV file
pub fn save_audio_file(samples: &mut [f32], path: &str) {
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
