use base64::engine::general_purpose;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
extern crate dotenv;
use dotenv::dotenv;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PostBody {
  input: Input,
  voice: Voice,
  audioConfig: AudioConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
  text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Voice {
  languageCode: String,
  name: String,
  ssmlGender: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct AudioConfig {
  audioEncoding: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
  audioContent: String,
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let bearer_token = std::env::var("GCLOUD_BEARER").unwrap();
  let project = std::env::var("GCLOUD_PROJECT").unwrap();
  let post_body = PostBody {
    input: Input {
      text: "Hello, world!".to_string(),
    },
    voice: Voice {
      languageCode: "en-gb".to_owned(),
      name: "en-GB-Standard-A".to_owned(),
      ssmlGender: "MALE".to_owned(),
    },
    audioConfig: AudioConfig {
      audioEncoding: "MP3".to_owned(),
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

  let json = res.json::<Response>().await.unwrap();
  let audio_context = json.audioContent;

  let mut file = File::create("audio.mp3").unwrap();
  file
    .write_all(
      general_purpose::STANDARD
        .decode(audio_context)
        .unwrap()
        .as_slice(),
    )
    .unwrap();
}
