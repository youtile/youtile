<script lang="ts">
  import { emit } from "@tauri-apps/api/event";
  import { appWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import type { VideoStream } from "../core/IStreams";
  import Loader from "./Loader.svelte";

  export let videoSource: HTMLVideoElement;
  export let audioSource: HTMLVideoElement;
  export let videoStreams: [VideoStream, VideoStream];

  const playing = () => (videoSource && !videoSource.paused && !videoSource.ended && videoSource.readyState > 2);

  let isPlaying = false;
  let isPinned = false;
  let isMuted = false;
  let isBuffering = true;
  let volume = 5; // Between 0 and 1000
  let time = 0;
  let duration = 0;
  let quality: 'hd' | '4k' = 'hd';

  $: quality, onQualityChanged();

  onMount(() => {
    setTimeout(() => {
      isPlaying = playing();
      appWindow.setAlwaysOnTop(false); isPinned = false;
      updateSpeakerIcon();

      /* Initialize the sliders */
      document.getElementById('audio-input').style.backgroundSize = (5 - 0) * 100 / (1000 - 0) + '% 100%';
      document.getElementById('timeline-input').style.backgroundSize = '0% 100%';

      /* Setup the video events */
      videoSource.addEventListener('playing', () => { isPlaying = playing(); isBuffering = false; });
      videoSource.addEventListener('play', () => isPlaying = playing());
      videoSource.addEventListener('pause', () => isPlaying = playing());
      videoSource.addEventListener('waiting', () => { isPlaying = playing(); isBuffering = true; });
      videoSource.addEventListener('timeupdate', () => { 
        // Update the time and duration of the video:
        if (timelineBusy == false) {
          time = videoSource.currentTime; 
          duration = videoSource.duration;
          document.getElementById('timeline-input').style.backgroundSize = `calc(${(time - 0) * 100 / (videoSource.duration - 0)}% + 6px) 100%`;
        }
      });
    }, 100);

    /* Setup the slider input events */
    document.getElementById('audio-input').addEventListener('input', (e) => { 
      handleInputChange(e); audioSource.volume = (e.target as any).value / 1000.0; updateSpeakerIcon();
    });
    document.getElementById('timeline-input').addEventListener('input', (e) => { 
      handleInputChange(e); timelineInput();
    });
    document.getElementById('timeline-input').addEventListener('change', (e) => { 
      if (videoSource.currentTime != (e.target as any).value)
        videoSource.currentTime = (e.target as any).value; audioSource.currentTime = (e.target as any).value; 
    });
  });

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
  const toggleMute = () => { isMuted = !isMuted; updateSpeakerIcon(); audioSource.muted = isMuted; };

  /** Toggle the pinned state of the window */
  const togglePin = () => { isPinned = !isPinned; appWindow.setAlwaysOnTop(isPinned); };

  /** Return back to the home screen */
  const returnFunc = () => { 
    var newSource = videoSource.cloneNode(true);
    videoSource.parentNode.replaceChild(newSource, videoSource);
    newSource = audioSource.cloneNode(true);
    audioSource.parentNode.replaceChild(newSource, audioSource);
    
    emit('stop_video'); 
  };

  /** Make sure the audio slider doesn't go away too quickly */
  let audioOpen = false;
  let audioTimer: NodeJS.Timeout;
  const audioEnter = () => { audioOpen = true; clearTimeout(audioTimer); };
  const audioLeave = () => { audioOpen = true; clearTimeout(audioTimer); audioTimer = setTimeout(() => {
    audioOpen = false;
  }, 500); };

  /** Make sure the timeline isn't updated while the user is moving it */
  let timelineBusy = false;
  let timelineTimer: NodeJS.Timeout;
  const timelineInput = () => { timelineBusy = true; clearTimeout(timelineTimer); timelineTimer = setTimeout(() => {
    timelineBusy = false;
    videoSource.currentTime = time; 
    audioSource.currentTime = time;
  }, 300); };

  /** Animate the audio slider */
  function handleInputChange(e) {
    let target = e.target;
    if (e.target.type !== 'range') {
      target = document.getElementById('range');
    } 
    const min = target.min;
    const max = target.max;
    const val = target.value;
    target.style.backgroundSize = `calc(${(val - min) * 100 / (max - min) + 1}%) 100%`;
  }

  /** Called whenever the quality selected is changed */
  const onQualityChanged = () => {
    if (videoSource && videoStreams) {
      const time = videoSource.currentTime;
      videoSource.setAttribute('src', videoStreams[quality == 'hd' ? 1 : 0].url);
      videoSource.currentTime = time;
    }
  };

</script>


<div class="controls">
  <button class="audio" on:mouseenter={audioEnter} on:mouseleave={audioLeave}>
    <img
      src="https://api.iconify.design/{ speakerIcon }.svg"
      alt="audio" on:click={toggleMute}
    />
    <input type="range" id="audio-input" min="0" max="1000" bind:value={volume} style="opacity: { audioOpen ? '1' : '0' };">
  </button>

  <button class="play" style="{ isBuffering ? 'pointer-events: none' : '' };" on:click={togglePlay}>
    <img
      src="https://api.iconify.design/{ isPlaying ? 'fluent:pause-12-filled' : 'carbon:play-filled-alt' }.svg"
      style="{ isBuffering ? 'opacity: 0' : '' };"
      alt="play"
    />
    <div class="loader">
      <Loader isLoading={isBuffering} animSpeed={0.1} />
    </div>
  </button>

  <button class="pin align-end" on:click={togglePin}>
    <img
      src="https://api.iconify.design/{ isPinned ? 'mdi:arrange-send-backward' : 'mdi:arrange-bring-forward' }.svg"
      alt="pin"
    />
  </button>

  <button class="return" on:click={returnFunc}>
    <img
      src="https://api.iconify.design/icon-park-solid:back.svg"
      alt="return"
    />
  </button>

  <div class="timeline">
    <input type="range" id="timeline-input" min="0" max={duration} bind:value={time}>
  </div>

  <div class="quality">
    <button class="quality align-end" on:click={() => quality = quality == 'hd' ? '4k' : 'hd'}>
      <img
        src="https://api.iconify.design/{ quality == 'hd' ? 'mdi:high-definition-box' : 'mdi:video-4k-box' }.svg"
        alt="quality"
      />
    </button>
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
      position: relative;
      margin-left: 8px;

      .loader {
        position: absolute;
        width: 16px;
        height: 16px;
        opacity: .7;

        top: 50%;
        left: 50%;
        transform: translate(-60%, -30%) scale(75%, 75%);
      }
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

    .return {
      margin-left: 8px;
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

    &::before {
      position: absolute;
      content: "";

      bottom: 0px;
      left: 0px;

      width: 100vw;
      height: 80px;

      pointer-events: none;
      background: linear-gradient(0deg, #00000088, transparent 100%);
    }

    .quality {
      position: fixed;
      top: 8px;
      left: 8px;

      .quality img {
        width: 24px;
        height: 24px;

        opacity: .8;

        &:hover {
          opacity: 1;
        }
      }
    }
  }
</style>
