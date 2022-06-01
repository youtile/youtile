use ytextract::{Video, Stream};

#[derive(serde::Serialize)]
pub struct StreamResolution {
  width: u16,
  height: u16,
  mine: String,
}

#[derive(serde::Serialize)]
pub struct StreamCouple {
  video_stream: String,
  audio_stream: String,
  resolution: StreamResolution,
}

#[tauri::command]
pub async fn stream_video(code: String) -> Vec<StreamCouple> {
  // Get a Client for making request
  let client = ytextract::Client::new();

  // Get information about the Video identified by the id "code".
  let video = client.video(code.parse().unwrap()).await.unwrap();

  let streams = video.streams().await.unwrap();

  let mut bestvid = String::new();
  let mut bestaud = String::new();
  for stream in streams {
    match stream {
        Stream::Video(stream) => {
          println!("{}/{} : {}", stream.width(), stream.height(), stream.mime_type());
          if bestvid == String::new() {
            bestvid = stream.url().to_string();
          }
        },
        Stream::Audio(stream) => {
          if bestaud == String::new() {
            bestaud = stream.url().to_string();
          }
        }
    }
  }

  Vec::new() // TODO: Make the stream couples and return them.
  //format!("{} {}", bestvid, bestaud)
}