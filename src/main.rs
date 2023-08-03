use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Espeak);
  let tts = tts.without_cache();

  let clips: Vec<Rc<dyn Render>> = vec![
    Rc::new(
      Speak::new("This is an automated message.").with_voice(VoiceModel::A),
    ),
    Rc::new(Speak::new("Please listen carefully.").with_voice(VoiceModel::C)),
    Rc::new(Pause(1_000)),
    Rc::new(Speak::new("Hello, World!").with_encoding(Encoding::Phonetic)),
  ];

  let mut samples = render_all(clips.into_iter(), tts).await;
  save_audio_file(&mut samples, "/tmp/conetto/audio.wav");
}
