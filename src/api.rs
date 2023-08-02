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

pub async fn generate_tts(text: String, model: String) -> Vec<f32> {
  let bearer_token = std::env::var("GCLOUD_BEARER").unwrap();
  let project = "ornate-axiom-327716";
  let post_body = PostBody {
    input: Input { text },
    voice: Voice {
      language_code: "en-us".to_owned(),
      name: model,
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

  let vec_u8 = general_purpose::STANDARD
    .decode(json.audio_content)
    .unwrap();
  let vec_f32 = vec_u8
    .chunks_exact(2)
    .map(|chunk| {
      let mut bytes = [0; 2];
      bytes.copy_from_slice(chunk);
      i16::from_le_bytes(bytes) as f32
    })
    .collect::<Vec<_>>();

  vec_f32
}
