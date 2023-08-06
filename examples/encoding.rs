use conetto::*;

#[tokio::main]
async fn main() {
  let mut tts = Tts::new(TTSService::Espeak, true, true);
  let clips: Vec<Clip> = vec![
    Speak::new("Phonetic encoding").into(),
    Speak::new("ABCD").with_encoding(Encoding::Phonetic).into(),
    //
    Speak::new("ASCII encoding").into(),
    Speak::new("foo").with_encoding(Encoding::Ascii).into(),
    //
    Speak::new("Words encoding").into(),
    Speak::new("one word at a time")
      .with_encoding(Encoding::Words)
      .into(),
  ];

  let mut samples = render_all(clips.into_iter(), &mut tts).await;
  save_audio_file(&mut samples, DEFAULT_RENDER_PATH);
}
