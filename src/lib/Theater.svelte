<script lang="ts">
  import { appWindow, LogicalSize, PhysicalSize } from '@tauri-apps/api/window';
  import { emit } from "@tauri-apps/api/event";
  import { videoCode } from "./store/store";
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri';

  const videoId = $videoCode;
  let loading = true;

  onMount(() => {
    appWindow.setDecorations(false);

    appWindow.setMinSize(new LogicalSize(240, 135));
    appWindow.innerSize().then((size: PhysicalSize) => {
      appWindow.setSize(new PhysicalSize(Math.round(size.height * 1.777778), size.height));
    });

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
      audio.volume = 0.4;

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
  <div class="titlebar" data-tauri-drag-region />

  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="video" style="display: { loading ? 'none' : 'block' };"></video>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video id="audio" style="display: none;"></video>

  <div class="loader" style="opacity: { loading ? '1' : '0' };">
    <svg version="1.1" id="L9" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
      viewBox="0 0 100 100" enable-background="new 0 0 0 0" xml:space="preserve">
        <rect x="20" y="50" width="4" height="10" fill="#fff">
          <animateTransform attributeType="xml"
            attributeName="transform" type="translate"
            values="0 0; 0 20; 0 0"
            begin="0" dur="0.6s" repeatCount="indefinite" />
        </rect>
        <rect x="30" y="50" width="4" height="10" fill="#fff">
          <animateTransform attributeType="xml"
            attributeName="transform" type="translate"
            values="0 0; 0 20; 0 0"
            begin="0.2s" dur="0.6s" repeatCount="indefinite" />
        </rect>
        <rect x="40" y="50" width="4" height="10" fill="#fff">
          <animateTransform attributeType="xml"
            attributeName="transform" type="translate"
            values="0 0; 0 20; 0 0"
            begin="0.4s" dur="0.6s" repeatCount="indefinite" />
        </rect>
    </svg>
  </div>

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
</div>

<style lang="scss">
  .theater {
    position: relative;

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

    .loader {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 100px;
      height: 100px;

      svg {
        position: absolute;
        top: calc(50% - 20px);
        left: calc(50% + 20px);
        transform: translate(-50%, -50%);
        width: 100px;
        height: 100px;
      }
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
  }
</style>