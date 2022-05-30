<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit } from "@tauri-apps/api/event";
  import { secondsToDuration } from "./core/Duration";
  import type Video from "./core/IVideo";
  import { videoCode } from "./store/store";
  import { onMount } from "svelte";
  import { appWindow, LogicalSize } from "@tauri-apps/api/window";

  let selectedVideo: Video | undefined = undefined;
  let code = 'Q5ztUFmIR0Q';

  onMount(() => {
    appWindow.setDecorations(true);
    appWindow.setMinSize(new LogicalSize(323 + 32, 270));
    appWindow.setSize(new LogicalSize(323 + 32, 270));
  });

  function search() {
    invoke('get_video_info', { code: code }).then((video: Video) => {
      selectedVideo = video;
    });
  }

  function play() {
    if (selectedVideo) {
      videoCode.set(code);

      emit('play_video');
    }
  }
</script>


<div class="home">

  <div class="info" data-tauri-drag-region>
    {#if selectedVideo}
      <div class="thumbnail" style="--src: { `url(${selectedVideo.thumbnail})` };">
        <div class="title"> { selectedVideo.title } </div>
        <div class="channel" style="--src: { `url(${selectedVideo.channel.thumbnail})` };"></div>
        <div class="duration"> { secondsToDuration(selectedVideo.duration) } </div>
      </div>
    {:else}
      <div class="thumbnail-placeholder">
      </div>
    {/if}

    <div class="description">
      { selectedVideo ? selectedVideo.description : '' }
    </div>
  </div>

  <div class="actions">
    <input class="code" type="text" name="video code" placeholder="Video code..." bind:value={code}>
    <div class="buttons">
      <button class="search" on:click={search} disabled={!(code !== '')}>
        <img
          src="https://api.iconify.design/fa:search.svg"
          alt="pin"
        />
      </button>
      <button class="play" on:click={play} disabled={!selectedVideo}>
        <img
          src="https://api.iconify.design/carbon:play-filled-alt.svg"
          alt="pin"
        />
      </button>
    </div>
  </div>

</div>


<style lang="scss">
  .home {
    width: 100%;
    height: 100%;

    margin: 0;
    padding: 0;

    display: flex;
    flex-direction: column;

    .info {
      width: 100%;
      flex-grow: 1;

      padding: 16px 0px 0px 16px;

      display: flex;

      background-color: #242424;

      .thumbnail {
        position: relative;

        width: 320px;
        min-width: 320px;
        height: 180px;

        background-size: cover;
        background-position: center;
        background-image: var(--src);

        border-radius: 6px;
        border: 1.5px solid #424448;

        &-placeholder {
          position: relative;

          width: 320px;
          min-width: 320px;
          height: 180px;

          background-color: #1c1c1c;

          border-radius: 6px;
          border: 1.5px solid #424448;
        }

        .title {
          position: absolute;

          width: 252px;
          height: 32px;

          bottom: -5px;
          left: 0;

          padding-left: 8px;

          color: #ffffffee;
          z-index: 1;
          
          font-weight: 700;
          white-space: nowrap;
          text-overflow: ellipsis;
          overflow: hidden;
        }

        .channel {
          position: absolute;

          width: 42px;
          height: 42px;

          bottom: 8px;
          right: 8px;

          background-size: cover;
          background-position: center;
          background-image: var(--src);
          z-index: 1;

          border-radius: 50%;
        }

        .duration {
          position: absolute;
          width: fit-content;
          height: 12px;

          top: 4px;
          right: 7px;

          padding: 1px 4px 4px 4px;
          border-radius: 4px;

          font-size: .8rem;
          user-select: none;

          background-color: #00000088;
          color: #ffffffcc;
        }

        &::after {
          position: absolute;
          content: "";

          bottom: -.5px;
          left: 0px;

          width: 320.5px;
          height: 100px;

          background: linear-gradient(0deg, #000000cc, transparent 100%);
          border-bottom-left-radius: 5px;
          border-bottom-right-radius: 5px;
        }
      }

      .description {
        flex-grow: 1;
        max-height: calc(100vh - 88px);

        padding-left: 16px;
        padding-right: 16px;

        color: #ffffff66;
        font-weight: 100;
        font-size: .9rem;

        overflow: hidden;
        text-overflow: ellipsis;
        white-space: pre-wrap;
        pointer-events: none;
        user-select: none;
      }
    }

    .actions {
      width: calc(100% - 24px);
      height: calc(54px - 24px);
      max-height: 54px;

      padding: 12px;

      display: flex;
      justify-content: space-between;

      background-color: #1c1c1c;
      border-top: 1.5px solid #424448;

      .code {
        width: 128px;
        height: 26px;
        font-family: FiraCode;

        background-color: #2f2f2f;
        border-radius: 6px;
        border: 1.5px solid #4e5055;
        outline: none;

        color: #ffffffbb;
        padding-left: 8px;

        &:focus {
          border: 1.5px solid #5ea5f666;
        }
      }

      .buttons {
        user-select: none;

        button {
          width: 30px;
          height: 30px;

          background-color: transparent;
          border-radius: 6px;
          border: none;
          outline: none;

          cursor: pointer;
          user-select: none;

          &:hover {
            background-color: #ffffff1a;
          }
        }

        .search {
          position: relative;
          margin-right: 8px;

          img {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);

            width: 14px;
            height: 14px;

            filter: invert(60%);
          }

          &:hover img {
            filter: invert(80%);
          }

          &:disabled {
            background-color: transparent !important;
            cursor: default;

            img {
              filter: invert(30%);
            }
          }
        }

        .play {
          position: relative;

          img {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);

            width: 14px;
            height: 14px;

            filter: invert(60%);
          }

          &:hover img {
            filter: invert(80%);
          }

          &:disabled {
            background-color: transparent !important;
            cursor: default;

            img {
              filter: invert(30%);
            }
          }
        }
      }
    }
  }
</style>