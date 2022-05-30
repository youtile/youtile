#[tauri::command]
pub async fn stream_video(code: String) -> String {
  // Get a Client for making request
  let client = ytextract::Client::new();

  // Get information about the Video identified by the id "code".
  let video = client.video(code.parse().unwrap()).await.unwrap();

  let streams = video.streams().await.unwrap();

  let mut bestvid = String::new();
  let mut bestaud = String::new();
  for stream in streams {
    if stream.is_video() {
      println!("video stream: {}b {}", stream.bitrate(), stream.mime_type());
      if bestvid == String::new() {
        bestvid = stream.url().to_string();
      }
    }
    if stream.is_audio() && bestaud == String::new() {
      bestaud = stream.url().to_string();
    }
  }

  format!("{} {}", bestvid, bestaud)
}