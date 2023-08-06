use conetto::*;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Google, true, true);
  let clips: Vec<Clip> = vec![
    Speak::new("Hello!").with_voice(VoiceModel::A).into(),
    Speak::new("this is using").with_voice(VoiceModel::B).into(),
    Speak::new("Google Cloud").with_voice(VoiceModel::C).into(),
    Speak::new("Text-To-Speech service.")
      .with_voice(VoiceModel::D)
      .into(),
  ];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
