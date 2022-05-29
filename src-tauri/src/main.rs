#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      set_shadow(&window, true).expect("Unsupported platform!");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![stream_video])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn stream_video(code: String) -> String {
  // Get a Client for making request
  let client = ytextract::Client::new();

  // Get information about the Video identified by the id "code".
  let video = client.video(code.parse().unwrap()).await.unwrap();

  // Print the title of the Video
  println!("Title: {}", video.title());

  let streams = video.streams().await.unwrap();

  let mut bestvid = String::new();
  let mut bestaud = String::new();
  for stream in streams {
    if stream.is_video() && stream.mime_type().contains("video/webm") {
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