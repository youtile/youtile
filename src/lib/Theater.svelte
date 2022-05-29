<script>
  import { appWindow } from '@tauri-apps/api/window';

  let pinned = false;
  function togglePin() {
    appWindow.setAlwaysOnTop(!pinned);
    pinned = !pinned;
  }

  function closeWindow() {
    appWindow.close();
  }
</script>

<div class="theater">
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
</div>

<style lang="scss">
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