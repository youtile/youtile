#[derive(serde::Serialize)]
pub struct VideoInfo {
  title: String,
  thumbnail: String,
  description: String,
  duration: u64,
  channel: ChannelInfo,
}

#[derive(serde::Serialize)]
pub struct ChannelInfo {
  name: String,
  thumbnail: String,
}

#[tauri::command]
pub async fn get_video_info(code: String) -> VideoInfo {

  // Get a Client for making request
  let client = ytextract::Client::new();

  // Get information about the Video identified by the id "code".
  let video = client.video(code.parse().unwrap()).await.unwrap();

  VideoInfo {
    title: video.title().to_string(),
    thumbnail: video.thumbnails()[video.thumbnails().len()-1].url.to_string(), 
    description: video.description().to_string(), 
    duration: video.duration().as_secs(),
    channel: ChannelInfo { name: video.channel().name().to_string(), thumbnail: video.clone().channel().thumbnails().next().unwrap().url.to_string() }
  }
}