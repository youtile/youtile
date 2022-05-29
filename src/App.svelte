<script lang="ts">
  import { onMount } from "svelte";
  import { appWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/tauri';

  const videoId = "GXti-sY-TRI";

  onMount(() => {
    invoke('stream_video', { code: videoId }).then((url: string) => {
      // Split the url into video and audio:
      const videoUrl = url.split(' ')[0];
      const audioUrl = url.split(' ')[1];

      // Fetch and create the elements:
      const video = document.getElementById('video') as HTMLVideoElement;
      const audio = document.getElementById('audio') as HTMLVideoElement;
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
      audio.volume = 0.1;

      // TEMP //
      // video.play();
      // audio.play();

      // Keep the audio and video in sync:
      video.addEventListener('waiting', () => {
        audio.pause();
      });
      video.addEventListener('playing', () => {
        audio.currentTime = video.currentTime;
        audio.play();
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
    appWindow.close();
  }
</script>

<main>
  <div class="titlebar" data-tauri-drag-region />

  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="video"></video>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="audio"></video>

  <div class="overlay">
    <button class="close-button" on:click={closeWindow}>
      <img
        src="https://api.iconify.design/material-symbols:close.svg"
        alt="close"
      />
    </button>
    <button class="pin-button" on:click={togglePin}>
      <img
        src="https://api.iconify.design/mdi:arrange-bring-forward.svg"
        alt="pin"
      />
    </button>
  </div>
</main>

<style lang="scss" global>
  html,
  body,
  #app {
    width: 100vw;
    height: 100vh;

    margin: 0;
    padding: 0;

    background: transparent;
    overflow: hidden;
  }

  main {
    width: 100vw;
    height: 100vh;

    display: flex;
    flex-direction: column;

    margin: 0;
    padding: 0;

    background-color: #33362f;

    overflow: hidden;
  }

  .overlay {
    position: absolute;
    top: 0;
    right: 0;
    width: 28px;
    height: 100%;
    pointer-events: none;
    user-select: none;
    z-index: 6;

    button {
      width: 25px;
      height: 15px;
      margin-right: 3px;
      pointer-events: all !important;
      background: none;
      border: none;
      outline: none;
      cursor: pointer;

      &.close-button {
        position: relative;

        img {
          width: 18px;
          height: 18px;

          position: absolute;
          top: 50%;
          left: 50%;
          transform: translate(-50%, -50%);
          filter: invert(100%);
          opacity: .85;

          &:hover {
            opacity: 1;
          }
        }
      }

      &.pin-button {
        position: relative;

        img {
          width: 14px;
          height: 14px;

          position: absolute;
          top: 50%;
          left: 50%;
          transform: translate(-50%, -50%);
          filter: invert(100%);
          opacity: .85;

          &:hover {
            opacity: 1;
          }
        }
      }
    }
  }

  .titlebar {
    position: absolute;
    width: 100vw;
    height: 25px;
    top: 0;
    left: 0;
    z-index: 5;

    background: transparent;
    user-select: none;
  }

  #video {
    background-color: black;

    width: 100%;
    height: 100%;

    pointer-events: none;
  }

  #audio {
    display: none;
  }
</style>
