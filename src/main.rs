use base64::engine::general_purpose;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

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
  ssml_gender: String,
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

#[tokio::main]
async fn main() {
  let bearer_token = std::env::var("GCLOUD_BEARER").unwrap();
  let project = "ornate-axiom-327716";
  let post_body = PostBody {
    input: Input {
      text: "Alpha, bravo, charlie, delta.".to_string(),
    },
    voice: Voice {
      language_code: "en-us".to_owned(),
      name: "en-US-Standard-B".to_owned(),
      ssml_gender: "MALE".to_owned(),
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
  let audio_context = json.audio_content;

  let mut file = File::create("audio.wav").unwrap();
  file
    .write_all(
      general_purpose::STANDARD
        .decode(audio_context)
        .unwrap()
        .as_slice(),
    )
    .unwrap();
}
