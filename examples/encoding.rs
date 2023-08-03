use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Espeak);
  let clips: Vec<Rc<dyn Render>> = vec![
    Rc::new(Speak::new("Phonetic encoding")),
    Rc::new(Speak::new("ABCD").with_encoding(Encoding::Phonetic)),
    Rc::new(Pause(1_000)),
    Rc::new(Speak::new("ASCII encoding")),
    Rc::new(Speak::new("ASCII Chars").with_encoding(Encoding::Ascii)),
    Rc::new(Pause(1_000)),
    Rc::new(Speak::new("Words encoding")),
    Rc::new(Speak::new("Use different voices").with_encoding(Encoding::Words)),
  ];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
