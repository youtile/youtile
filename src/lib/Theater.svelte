<script lang="ts">
  import { appWindow, LogicalSize, PhysicalSize } from '@tauri-apps/api/window';
  import { emit } from "@tauri-apps/api/event";
  import { videoCode } from "./store/store";
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import Loader from './comp/Loader.svelte';
  import Controls from './comp/Controls.svelte';

  const videoId = $videoCode;
  let video: HTMLVideoElement | undefined;
  let audio: HTMLVideoElement | undefined;
  let loading = true;

  onMount(() => {
    appWindow.setDecorations(false);

    appWindow.setMinSize(new LogicalSize(240, 135));
    appWindow.setMaxSize(null);
    appWindow.innerSize().then((size: PhysicalSize) => {
      appWindow.setSize(new PhysicalSize(Math.round(size.height * 1.777778), size.height));
    });

    video = document.getElementById('video') as HTMLVideoElement;
    audio = document.getElementById('audio') as HTMLVideoElement;

    invoke('stream_video', { code: videoId }).then((url: string) => {
      // Split the url into video and audio:
      const videoUrl = url.split(' ')[0];
      const audioUrl = url.split(' ')[1];

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

      // TEMP //
      video.play();
      audio.play();

      // Keep the audio and video in sync:
      video.addEventListener('waiting', () => {
        audio.pause();
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

  let pinned = false;
  function togglePin() {
    appWindow.setAlwaysOnTop(!pinned);
    pinned = !pinned;
  }

  function closeWindow() {
    //appWindow.close();
    emit('stop_video');
  }
</script>

<div class="theater">
  <div class="drag-area" data-tauri-drag-region></div>

  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="video" style="opacity: { loading ? '0' : '1' };"></video>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="audio" style="display: none;"></video>

  <div class="loader">
    <Loader isLoading={loading} />
  </div>

  <Controls videoSource={video} audioSource={audio} />
</div>

<style lang="scss">
  .theater {
    position: relative;

    width: 100vw;
    height: 100vh;

    .drag-area {
      position: absolute;
      width: 100vw;
      height: 100vh;
      top: 0;
      left: 0;
      z-index: 1;

      background: transparent;
      user-select: none;
    }

    .loader {
      position: absolute;

      top: 0;
      left: 0;

      width: 100vw;
      height: 100vh;
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