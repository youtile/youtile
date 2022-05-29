#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod info;
use info::get_video_info;

mod stream;
use stream::stream_video;

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      set_shadow(&window, true).expect("Unsupported platform!");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![stream_video, get_video_info])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}