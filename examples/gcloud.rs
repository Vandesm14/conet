use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Google);
  let clips: Vec<Rc<dyn Render>> = vec![
    Rc::new(Speak::new("Hello!").with_voice(VoiceModel::A)),
    Rc::new(Speak::new("this is using").with_voice(VoiceModel::B)),
    Rc::new(Speak::new("Google Cloud").with_voice(VoiceModel::C)),
    Rc::new(Speak::new("Text-To-Speech service.").with_voice(VoiceModel::D)),
  ];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
