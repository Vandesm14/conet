use conetto::*;
use std::rc::Rc;

#[tokio::main]
async fn main() {
  let clips: Vec<Rc<dyn Render>> = vec![Rc::new(Speak::new("Hello, World!"))];

  let mut samples = render_all(clips.into_iter()).await;
  save_audio_file(&mut samples, "audio.wav");
}
