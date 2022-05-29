<script lang="ts">
  import Home from "./lib/Home.svelte";

  // import { onMount } from "svelte";
  // import { appWindow } from '@tauri-apps/api/window';
  // import { invoke } from '@tauri-apps/api/tauri';

  // const videoId = "GXti-sY-TRI";

  // onMount(() => {
  //   invoke('stream_video', { code: videoId }).then((url: string) => {
  //     // Split the url into video and audio:
  //     const videoUrl = url.split(' ')[0];
  //     const audioUrl = url.split(' ')[1];

  //     // Fetch and create the elements:
  //     const video = document.getElementById('video') as HTMLVideoElement;
  //     const audio = document.getElementById('audio') as HTMLVideoElement;
  //     const videoSource = document.createElement("source");
  //     const audioSource = document.createElement("source");
      
  //     // Setup the video and audio stream:
  //     videoSource.setAttribute('src', videoUrl);
  //     videoSource.setAttribute('type', 'video/mp4');
  //     audioSource.setAttribute('src', audioUrl);
  //     audioSource.setAttribute('type', 'audio/mp4');

  //     // Add the sources into the video elements:
  //     video.appendChild(videoSource);
  //     audio.appendChild(audioSource);
  //     audio.volume = 0.1;

  //     // TEMP //
  //     // video.play();
  //     // audio.play();

  //     // Keep the audio and video in sync:
  //     video.addEventListener('waiting', () => {
  //       audio.pause();
  //     });
  //     video.addEventListener('playing', () => {
  //       audio.currentTime = video.currentTime;
  //       audio.play();
  //     });
  //     video.addEventListener('pause', () => {
  //       audio.pause();
  //     });
  //     video.addEventListener('play', () => {
  //       audio.currentTime = video.currentTime;
  //       audio.play();
  //     });
  //   });
  // });

  // let pinned = false;
  // function togglePin() {
  //   appWindow.setAlwaysOnTop(!pinned);
  //   pinned = !pinned;
  // }

  // function closeWindow() {
  //   appWindow.close();
  // }
</script>

<main>
  <Home />
</main>

<style lang="scss" global>
  @import './assets/fonts/fonts.css';

  :root {
    font-family: RadioCanada !important;
  }

  html,
  body,
  #app,
  main {
    width: 100vw;
    height: 100vh;

    margin: 0;
    padding: 0;

    background-color: #2f3136;
    overflow: hidden;
  }
</style>
