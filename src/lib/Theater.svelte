<script lang="ts">
  import { appWindow, LogicalSize, PhysicalSize } from '@tauri-apps/api/window';
  import { videoCode } from "./store/store";
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import Loader from './comp/Loader.svelte';
  import Controls from './comp/Controls.svelte';
  import type Streams from './core/IStreams';
  import type { VideoStream } from './core/IStreams';

  const videoId = $videoCode;
  let video: HTMLVideoElement | undefined;
  let audio: HTMLVideoElement | undefined;
  let videoStreams: [VideoStream, VideoStream] | undefined;
  let loading = true;

  onMount(() => {
    appWindow.setDecorations(false);

    appWindow.setResizable(true);
    appWindow.setMinSize(new LogicalSize(240, 135));
    appWindow.setMaxSize(null);
    appWindow.innerSize().then((size: PhysicalSize) => {
      appWindow.setSize(new PhysicalSize(Math.round(size.height * 1.777778), size.height));
    });

    video = document.getElementById('video') as HTMLVideoElement;
    audio = document.getElementById('audio') as HTMLVideoElement;

    invoke('stream_video', { code: videoId }).then((streams: Streams) => {
      // Split the url into video and audio:
      const videoUrl = streams.video.find(v => v.resolution.height <= 1080 && v.resolution.width <= 1920).url;
      const audioUrl = streams.audio[0].url;

      videoStreams = [
        // UHD / 4K
        streams.video[0],
        // HD and below
        streams.video.find(v => v.resolution.height <= 1080 && v.resolution.width <= 1920)
      ];

      // Fetch and create the elements:
      const videoSource = document.createElement("source");
      const audioSource = document.createElement("source");
      
      // Setup the video and audio stream:
      videoSource.setAttribute('src', videoUrl);
      videoSource.setAttribute('type', 'video/mp4');
      audioSource.setAttribute('src', audioUrl);
      audioSource.setAttribute('type', 'audio/mp4');

      // Add the sources into the video elements:
      video.appendChild(videoSource);
      audio.appendChild(audioSource);
      audio.volume = 0.05;

      // Start the video once its ready:
      video.addEventListener('canplay', () => {
        video.play();
        audio.currentTime = video.currentTime;
        audio.play();
      });

      // Keep the audio and video in sync:
      video.addEventListener('waiting', () => {
        audio.pause();
        video.pause();

        // Stop the video to buffer for 2 seconds.
        setTimeout(() => {
          video.play();
          audio.currentTime = video.currentTime;
          audio.play();
        }, 2000);
      });
      video.addEventListener('playing', () => {
        audio.currentTime = video.currentTime;
        audio.play();
        loading = false;
      });
      video.addEventListener('pause', () => {
        audio.pause();
      });
      video.addEventListener('play', () => {
        audio.currentTime = video.currentTime;
        audio.play();
      });
    });
  });
</script>


<div class="theater">
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="video" style="opacity: { loading ? '0' : '1' };"></video>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="audio" style="display: none;"></video>

  <div class="loader">
    <Loader isLoading={loading} />
  </div>

  <div class="control-bar" data-tauri-drag-region>
    <Controls videoSource={video} audioSource={audio} videoStreams={videoStreams} />
  </div>
</div>


<style lang="scss">
  .theater {
    position: relative;

    width: 100vw;
    height: 100vh;

    .loader {
      position: absolute;

      top: 0;
      left: 0;

      width: 100vw;
      height: 100vh;
    }

    .control-bar {
      position: absolute;

      width: 100vw;
      height: 100vh;

      top: 0;
      left: 0;

      transition: opacity 0.2s, transform 0.2s;

      opacity: 0;
      transform: translateY(24px);
      z-index: 1;

      &:hover {
        opacity: 1;
        transform: translateY(0px);
      }
    }

    #video {
      background-color: black;

      width: 100%;
      height: 100%;

      pointer-events: none;
      transition: opacity 0.5s;
    }

    #audio {
      display: none;
    }
  }
</style>