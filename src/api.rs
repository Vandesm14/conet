#![warn(missing_docs)]

use std::process::{Command, Stdio};

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

/// Send an API request to Google Cloud Text-to-Speech and returns the Base64 WAVE data
pub async fn google_cloud_tts(text: &str, model: &str) -> String {
  let bearer_token = std::env::var("GCLOUD_BEARER")
    .expect("Environment variable: GCLOUD_BEARER is not set");
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

/// Runs the eSpeak command and returns the Base64 WAVE data
pub async fn espeak_tts(text: &str, _voice: &str) -> String {
  // FIXME: Espeak exports at a different sample rate than GCloud, so we need to slow it down and pitch down a bit
  // I'm not sure how to fix this besides the hack of speed/pitch adjustment.
  let command = Command::new("espeak")
    .args([text, "--stdout", "-s", "140", "-p", "30"])
    .stdout(Stdio::piped())
    .spawn();

  // You can handle the spawned process here if needed.
  if let Err(err) = &command {
    eprintln!("Error: {}", err);
  }

  let output = command.unwrap().wait_with_output().unwrap();
  let stdout = output.stdout;

  general_purpose::STANDARD.encode(stdout)
}
