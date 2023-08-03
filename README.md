# Conetto

Build cryptographic audio messages using Text-To-Speech.

_Inspired by [Number Stations](https://en.wikipedia.org/wiki/Numbers_station) and [Person of Interest](https://en.wikipedia.org/wiki/Person_of_Interest_(TV*series))*

## Pre-requisites

You will need to install [gcloud](https://cloud.google.com/sdk/docs/install) for using Google Text-To-Speech as a TTS service. Alternatively, you can use [eSpeak](#espeak) as the TTS engine.

### Google Cloud

Once you have installed the `gcloud` CLI, you will need to authenticate with Google Cloud. You can do this by running `gcloud auth login` and following the instructions.

You will also need to create a Google Cloud project and enable the [Text-To-Speech API](https://console.cloud.google.com/apis/library/texttospeech.googleapis.com).

Next, you will need to create a service account and download the credentials file. You can do this by following the instructions [here](https://cloud.google.com/text-to-speech/docs/quickstart-protocol).

Finally, you need to impersonate the service account. You can do this by running `gcloud auth activate-service-account --key-file=<path-to-credentials-file>`.

#### Bearer Token

Before you can use the project, you will need to set the `GCLOUD_BEARER` environment variable to the bearer token of the service account.

We provide a few ways to do this automatically:

- BASH/ZSH: `source shell.sh`
- Nushell: `source shell.nu`
- Nix: `nix-shell`

Alternatively, you can get the bearer token by running `gcloud auth application-default print-access-token` and setting the `GCLOUD_BEARER` environment variable to the output.

_Note: The bearer token will need to be set whenever you run the project or the binary. Also, Google may revoke the token which Conetto will let you know if this is the case and you need to rerun the steps above._

### eSpeak

If you do not wish to use Google Cloud, you can use [eSpeak](http://espeak.sourceforge.net/) as the TTS engine. You can install `espeak` from their [downloads page](https://espeak.sourceforge.net/download.html) or use `Nix` with our `shell.nix` file.

## Building

To build the project, run `cargo build --release`.

## Usage

### Library

Conetto uses a declarative approach to creating an audio file. Here is a simple "Hello World" example:

```rust
use conetto::*;

#[tokio::main]
async fn main() {
  // You can use TTSService::Google for Google Cloud TTS
  let mut tts = Tts::new(TTSService::Espeak);
  let clips: Vec<Clip> = vec![Speak::new("Hello, World!").into()];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
```

Running this code will create an audio file at `/tmp/conetto.wav`.

_For more examples, see the [examples](./examples) directory._
