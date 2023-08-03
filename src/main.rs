use core::fmt;
use std::fmt::{Display, Formatter};

use conet::Tts;
use hound::WavSpec;
use lowpass_filter::lowpass_filter;
use spellabet::{PhoneticConverter, SpellingAlphabet};

enum Encoding {
  Words,
  Ascii,
  Phonetic,
}

struct Speak<'a> {
  text: &'a str,

  /// If None, a random voice will be selected
  voice: Option<VoiceModel>,

  /// If None, the no encoding will be used
  encoding: Option<Encoding>,
}

impl<'a> Speak<'a> {
  fn new(text: &'a str) -> Self {
    Self {
      encoding: None,
      text,
      voice: None,
    }
  }

  fn with_voice(mut self, voice: VoiceModel) -> Self {
    self.voice = Some(voice);
    self
  }

  fn with_encoding(mut self, encoding: Encoding) -> Self {
    self.encoding = Some(encoding);
    self
  }

  fn model(&self) -> Option<String> {
    let voice_model = self.voice;
    voice_model?;

    Some(voice_model.unwrap().to_string())
  }
}

#[derive(Clone, Copy, Debug)]
enum VoiceModel {
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

struct Pause(u32);

enum Clip<'a> {
  /// Text to speak
  Speak(Speak<'a>),

  /// Pause in milliseconds
  Pause(Pause),
}

impl<'a> Clip<'a> {
  async fn render(value: Clip<'a>, samples: &mut Vec<f32>, tts: &mut Tts) {
    match value {
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
                samples.extend([0.0f32; 4_000]);
              }

              // Long pause between words
              samples.extend([0.0f32; 10_000]);
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
                  samples.extend([0.0f32; 16_000]);
                  continue;
                }

                let more_samples = tts.generate(word, speak.model()).await;
                samples.extend(more_samples);

                // Short pause between words
                samples.extend([0.0f32; 4_000]);
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
    };
  }
}

impl<'a> From<Speak<'a>> for Clip<'a> {
  fn from(speak: Speak<'a>) -> Self {
    Clip::Speak(speak)
  }
}

impl<'a> From<Pause> for Clip<'a> {
  fn from(pause: Pause) -> Self {
    Clip::Pause(pause)
  }
}

#[tokio::main]
async fn main() {
  let mut tts = Tts::new();
  let clips: Vec<Clip> = vec![
    Speak::new("This is an automated message.")
      .with_voice(VoiceModel::A)
      .into(),
    Speak::new("Please listen carefully.")
      .with_encoding(Encoding::Words)
      .into(),
    Pause(1_000).into(),
    Speak::new("Hello, World!")
      .with_encoding(Encoding::Phonetic)
      .into(),
  ];

  let mut samples = vec![];

  for clip in clips {
    Clip::render(clip, &mut samples, &mut tts).await;
  }

  save_audio_file(&mut samples, "output.wav");
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
