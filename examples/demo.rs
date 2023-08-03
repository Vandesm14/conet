use conetto::*;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Espeak);
  let clips: Vec<Clip> = vec![Speak::new("Hello, World!").into()];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
