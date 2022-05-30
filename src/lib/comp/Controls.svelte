<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  export let videoSource: HTMLVideoElement;
  export let audioSource: HTMLVideoElement;

  const playing = () => (videoSource && videoSource.currentTime > 0 && !videoSource.paused && !videoSource.ended && videoSource.readyState > 2);

  let isPlaying = false;
  let isPinned = false;
  let isMuted = false;
  let volume = 5; // Between 0 and 1000
  let time = 0;

  onMount(() => {
    setTimeout(() => {
      isPlaying = playing();
      updateSpeakerIcon();

      document.getElementById('audio-input').style.backgroundSize = (5 - 0) * 100 / (1000 - 0) + '% 100%';

      /* Setup the video events */
      videoSource.addEventListener('playing', () => isPlaying = playing());
      videoSource.addEventListener('play', () => isPlaying = playing());
      videoSource.addEventListener('pause', () => isPlaying = playing());
      videoSource.addEventListener('waiting', () => isPlaying = playing());
      videoSource.addEventListener('timeupdate', () => { 
        time = videoSource.currentTime; 
        document.getElementById('timeline-input').style.backgroundSize = (time - 0) * 100 / (videoSource.duration - 0) + '% 100%';
      });
    }, 100);

    document.getElementById('audio-input').addEventListener('input', handleInputChange);
    document.getElementById('timeline-input').addEventListener('input', (e) => { 
      handleInputChange(e); videoSource.currentTime = time; audioSource.currentTime = time; 
    });
  });

  let audioOpen = false;

  /** Return different icon based on the selected volume */
  let speakerIcon = 'fluent:speaker-mute-24-filled';
  const updateSpeakerIcon = () => {
    if (isMuted) speakerIcon = 'fluent:speaker-mute-24-filled';
    else if (volume == 0) speakerIcon = 'fluent:speaker-0-24-filled';
    else if (volume < 500) speakerIcon = 'fluent:speaker-1-24-filled';
    else speakerIcon = 'fluent:speaker-2-24-filled';
  };

  /** Toggle the playing state of the video */
  const togglePlay = () => { playing() ? videoSource.pause() : videoSource.play(); };

  /** Toggle the muted state of the audio */
  const toggleMute = () => { isMuted = !isMuted; updateSpeakerIcon(); };

  /** Toggle the pinned state of the window */
  const togglePin = () => { isPinned = !isPinned; appWindow.setAlwaysOnTop(isPinned); };

  /** Make sure the audio slider doesn't go away too quickly */
  let audioTimer: NodeJS.Timeout;
  const audioEnter = () => { audioOpen = true; clearTimeout(audioTimer); };
  const audioLeave = () => { audioOpen = true; clearTimeout(audioTimer); audioTimer = setTimeout(() => {
    audioOpen = false;
  }, 500); };

  /** Animate the audio slider */
  function handleInputChange(e) {
    let target = e.target;
    if (e.target.type !== 'range') {
      target = document.getElementById('range');
    } 
    const min = target.min;
    const max = target.max;
    const val = target.value;

    updateSpeakerIcon();
    
    target.style.backgroundSize = (val - min) * 100 / (max - min) + '% 100%';
  }

</script>

<div class="controls">
  <button class="audio" on:mouseenter={audioEnter} on:mouseleave={audioLeave}>
    <img
      src="https://api.iconify.design/{ speakerIcon }.svg"
      alt="pin" on:click={toggleMute}
    />
    <input type="range" id="audio-input" min="0" max="1000" bind:value={volume} style="opacity: { audioOpen ? '1' : '0' };">
  </button>

  <button class="play" on:click={togglePlay}>
    <img
      src="https://api.iconify.design/{ isPlaying ? 'fluent:pause-12-filled' : 'carbon:play-filled-alt' }.svg"
      alt="pin"
    />
  </button>

  <button class="settings align-end">
    <img
      src="https://api.iconify.design/ci:settings-filled.svg"
      alt="pin"
    />
  </button>

  <button class="pin" on:click={togglePin}>
    <img
      src="https://api.iconify.design/{ isPinned ? 'mdi:arrange-send-backward' : 'mdi:arrange-bring-forward' }.svg"
      alt="pin"
    />
  </button>

  <div class="timeline">
    <input type="range" id="timeline-input" min="0" max="1000" bind:value={time}>
  </div>
</div>

<style lang="scss">
  .controls {
    position: absolute;

    width: calc(100% - 24px);
    height: calc(54px - 24px);
    max-height: 54px;

    bottom: 0;
    left: 0;

    padding: 12px;

    display: flex;
    user-select: none;

    button {
      position: relative;
      z-index: 2;

      width: 30px;
      height: 30px;

      background-color: transparent;
      border-radius: 6px;
      border: none;
      outline: none;

      cursor: pointer;
      user-select: none;
      transition: background-color 0.1s;

      &:hover {
        background-color: #ffffff1a;
      }

      img {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);

        width: 14px;
        height: 14px;
        transition: filter 0.1s, opacity 0.1s;

        filter: invert(100%);
        opacity: .5;
      }

      &:hover img {
        opacity: 1;
      }

      &:disabled {
        background-color: transparent !important;
        cursor: default;

        img {
          opacity: .2;
        }
      }
    }

    .play {
      margin-left: 8px;
    }

    .audio {
      position: relative;

      > input {
        position: absolute;

        width: 70px;
        height: 5px;

        bottom: 65px;
        right: -21.5px;

        background: #ffffff55;
        outline: none;

        transform: rotate(270deg);

        appearance: none;
        border-radius: 2.5px;
        opacity: 0;
        transition: opacity 0.3s;

        background-image: linear-gradient(#ffffffbb, #ffffffbb);
        background-size: 70% 100%;
        background-repeat: no-repeat;

        &::-webkit-slider-thumb {
          opacity: 0;
        }

        &:focus {
          outline: none;
        }

        &::-ms-track {
          width: 100%;
          cursor: pointer;

          background: transparent; 
          border-color: transparent;
          color: transparent;
        }
      }

      &:hover {
        > input {
          opacity: 1;
        }
      }
    }

    .settings {
      margin-right: 8px;
    }

    .align-end {
      margin-left: auto;
      justify-self: flex-end;
    }

    .timeline {
      position: absolute;
      width: 100vw;
      height: 52px;

      bottom: 0;
      left: 0;

      > input {
        position: absolute;

        width: calc(100vw - 180px);
        height: 3px;

        bottom: 24px;
        left: 88px;

        background: #ffffff55;
        outline: none;

        appearance: none;
        border-radius: 2.5px;
        z-index: 2;

        background-image: linear-gradient(#ffffffbb, #ffffffbb);
        background-size: 70% 100%;
        background-repeat: no-repeat;

        &::-webkit-slider-thumb {
          opacity: 0;
        }

        &:focus {
          outline: none;
        }

        &::-ms-track {
          width: 100%;
          cursor: pointer;

          background: transparent; 
          border-color: transparent;
          color: transparent;
        }
      }
    }
  }
</style>
