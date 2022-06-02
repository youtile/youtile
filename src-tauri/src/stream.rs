use ytextract::Stream;

#[derive(serde::Serialize)]
pub struct StreamResolution {
  width: u16,
  height: u16,
  fps: u8,
  mine: String,
}

#[derive(serde::Serialize)]
pub struct StreamQuality {
  audio_sample_rate: u64,
  audio_channels: u64,
}

#[derive(serde::Serialize)]
pub struct VideoStreamInfo {
  url: String,
  resolution: StreamResolution,
}

#[derive(serde::Serialize)]
pub struct AudioStreamInfo {
  url: String,
  quality: StreamQuality,
}

#[derive(serde::Serialize)]
pub struct StreamsInfo {
  video: Vec<VideoStreamInfo>,
  audio: Vec<AudioStreamInfo>,
}

#[tauri::command]
pub async fn stream_video(code: String) -> StreamsInfo {
  // Get a Client for making request.
  let client = ytextract::Client::new();

  // Get information about the Video identified by the id "code".
  let video = client.video(code.parse().unwrap()).await.unwrap();

  let streams = video.streams().await.unwrap();

  // Build the video and audio channels:
  let mut video_channels: Vec<VideoStreamInfo> = Vec::new();
  let mut audio_channels: Vec<AudioStreamInfo> = Vec::new();
  for stream in streams {
    match stream {
        Stream::Video(stream) => {
          video_channels.push(VideoStreamInfo {
            resolution: StreamResolution {
              width: stream.width() as u16,
              height: stream.height() as u16,
              fps: stream.fps() as u8,
              mine: stream.mime_type().to_string(),
            },
            url: stream.url().to_string(),
          });
        },
        Stream::Audio(stream) => {
          audio_channels.push(AudioStreamInfo {
            quality: StreamQuality {
              audio_sample_rate: stream.sample_rate() as u64,
              audio_channels: stream.channels() as u64,
            },
            url: stream.url().to_string(),
          });
        }
    }
  }

  StreamsInfo {
    video: video_channels,
    audio: audio_channels,
  }
}