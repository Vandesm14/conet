# Conetto

Build cryptographic audio messages using Text-To-Speech.

_Inspired by [Number Stations](https://en.wikipedia.org/wiki/Numbers_station) and [Person of Interest](https://en.wikipedia.org/wiki/Person_of_Interest_(TV*series))*

## Pre-requisites

You will need to install [gcloud](https://cloud.google.com/sdk/docs/install) and [Rust](https://www.rust-lang.org/tools/install) to build & use this project.

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

## Building

To build the project, run `cargo build --release`.

## Usage

### Library

Conetto uses a declarative approach to creating an audio file. Here is a simple "Hello World" example:

```rust
use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let clips: Vec<Rc<dyn Render>> = vec![
    Rc::new(Speak::new("Hello, World!")),
  ];

  let mut samples = render_all(clips.into_iter()).await;
  save_audio_file(&mut samples, "audio.wav");
}
```

Running this code will create an audio file called `audio.wav` in the current directory.

_For more examples, see the [examples](examples) directory._
