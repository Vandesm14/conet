use base64::{engine::general_purpose, Engine};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::request_tts;

#[derive(Debug, Clone)]
pub struct TTS {
  rng: StdRng,
  pub memcache: std::collections::HashMap<String, String>,
}

impl Default for TTS {
  fn default() -> Self {
    Self {
      rng: StdRng::from_seed([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      memcache: std::collections::HashMap::new(),
    }
  }
}

impl TTS {
  /// Create a new TTS instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Get the Base64 WAVE data from the cache
  pub fn get_from_cache(
    &mut self,
    text: impl AsRef<[u8]>,
    model: impl AsRef<[u8]>,
  ) -> Option<String> {
    let text = general_purpose::STANDARD.encode(text);
    let model = general_purpose::STANDARD.encode(model);

    let key = format!("{}-{}", text, model);

    // If the data exists in memcache, return it
    if self.memcache.contains_key(&key) {
      return Some(self.memcache[&key].clone());
    }

    let path = format!("/tmp/conet/{}.wav", key);
    if std::path::Path::new(&path).exists() {
      let data = std::fs::read_to_string(path).unwrap();
      self.memcache.insert(key.clone(), data.clone());

      Some(data)
    } else {
      None
    }
  }

  /// Save the Base64 WAVE data to the cache
  pub fn send_to_cache(
    &mut self,
    text: impl AsRef<[u8]>,
    model: impl AsRef<[u8]>,
    contents: impl AsRef<[u8]>,
  ) {
    let text = general_purpose::STANDARD.encode(text);
    let model = general_purpose::STANDARD.encode(model);

    let key = format!("{}-{}", text, model);
    // If the key doesn't exist, insert it
    if !self.memcache.contains_key(&key) {
      self.memcache.insert(key.clone(), String::new());
    }

    let path = format!("/tmp/conet/{}.wav", key);
    std::fs::write(path, contents).unwrap();
  }

  /// Generate a Vec of f64 samples from a string
  pub async fn generate(
    &mut self,
    text: &str,
    model: Option<&str>,
  ) -> Vec<f32> {
    let text = text.to_lowercase();
    let text = text.as_str();

    let model_letters = "ABCDEFGHIJ".chars().collect::<Vec<_>>();
    let model = match model {
      Some(model) => model.to_owned(),
      None => {
        let model = model_letters[self.rng.gen_range(0..model_letters.len())];
        format!("en-US-Standard-{}", model)
      }
    };

    let base64_string = match TTS::get_from_cache(self, text, &model) {
      Some(val) => {
        println!("Cache hit: {}-{}", text, &model);
        val
      }
      None => {
        println!("Cache miss: {}-{}", text, model);
        let val = request_tts(text, &model).await;
        TTS::send_to_cache(self, text, model, &val);
        val
      }
    };

    vec_u8_to_vec_f32(general_purpose::STANDARD.decode(base64_string).unwrap())
  }
}

/// Convert a Vec of u8 samples to a Vec of f64 samples (little-endian)
fn vec_u8_to_vec_f32(vec_u8: Vec<u8>) -> Vec<f32> {
  vec_u8
    .chunks_exact(2)
    // Skip WAV header
    .skip(8 * 4)
    .map(|chunk| {
      let mut bytes = [0; 2];
      bytes.copy_from_slice(chunk);
      i16::from_le_bytes(bytes) as f32
    })
    .collect::<Vec<_>>()
}
