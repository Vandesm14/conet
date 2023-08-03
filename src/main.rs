use conet::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let clips: Vec<Rc<dyn Render>> = vec![
    Rc::new(
      Speak::new("This is an automated message.").with_voice(VoiceModel::A),
    ),
    Rc::new(
      Speak::new("Please listen carefully.").with_encoding(Encoding::Words),
    ),
    Rc::new(Pause(1_000)),
    Rc::new(Speak::new("Hello, World!").with_encoding(Encoding::Phonetic)),
  ];

  let mut samples = render_all(clips.into_iter()).await;
  save_audio_file(&mut samples, "/tmp/conet/audio.wav");
}
