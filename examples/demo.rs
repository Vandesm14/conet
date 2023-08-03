use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Espeak);
  let clips: Vec<Rc<dyn Render>> = vec![Rc::new(Speak::new("Hello, World!"))];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
