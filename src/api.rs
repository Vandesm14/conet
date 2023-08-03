use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostBody {
  input: Input,
  voice: Voice,
  audio_config: AudioConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
  text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Voice {
  language_code: String,
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioConfig {
  audio_encoding: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
  audio_content: String,
}

async fn request_tts(text: &str, model: &str) -> String {
  let bearer_token = std::env::var("GCLOUD_BEARER").unwrap();
  let project = "ornate-axiom-327716";
  let post_body = PostBody {
    input: Input {
      text: text.to_owned(),
    },
    voice: Voice {
      language_code: "en-us".to_owned(),
      name: model.to_owned(),
    },
    audio_config: AudioConfig {
      audio_encoding: "LINEAR16".to_owned(),
    },
  };

  let res = Client::new()
    .post("https://texttospeech.googleapis.com/v1beta1/text:synthesize")
    .bearer_auth(bearer_token)
    .header("x-goog-user-project", project)
    .body(serde_json::to_string(&post_body).unwrap())
    .send()
    .await
    .unwrap();

  let text = res.text().await.unwrap();
  let json: Response = serde_json::from_str(&text).unwrap();

  json.audio_content
}

fn get_from_cache(
  text: impl AsRef<[u8]>,
  model: impl AsRef<[u8]>,
) -> Option<String> {
  let text = general_purpose::STANDARD.encode(text);
  let model = general_purpose::STANDARD.encode(model);

  let path = format!("/tmp/conet/{}-{}.wav", text, model);
  if std::path::Path::new(&path).exists() {
    Some(std::fs::read_to_string(path).unwrap())
  } else {
    None
  }
}

fn send_to_cache(
  text: impl AsRef<[u8]>,
  model: impl AsRef<[u8]>,
  contents: impl AsRef<[u8]>,
) {
  let text = general_purpose::STANDARD.encode(text);
  let model = general_purpose::STANDARD.encode(model);

  let path = format!("/tmp/conet/{}-{}.wav", text, model);
  std::fs::write(path, contents).unwrap();
}

pub async fn generate_tts(text: &str, model: &str) -> Vec<f32> {
  let text = text.to_lowercase();
  let text = text.as_str();

  let base64_string = match get_from_cache(text, model) {
    Some(val) => {
      println!("Cache hit: {}-{}", text, model);
      val
    }
    None => {
      println!("Cache miss: {}-{}", text, model);
      let val = request_tts(text, model).await;
      send_to_cache(text, model, &val);
      val
    }
  };

  vec_u8_to_vec_f32(general_purpose::STANDARD.decode(base64_string).unwrap())
}

fn vec_u8_to_vec_f32(vec_u8: Vec<u8>) -> Vec<f32> {
  vec_u8
    .chunks_exact(2)
    .map(|chunk| {
      let mut bytes = [0; 2];
      bytes.copy_from_slice(chunk);
      i16::from_le_bytes(bytes) as f32
    })
    .collect::<Vec<_>>()
}
