<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit } from "@tauri-apps/api/event";
  import { secondsToDuration } from "./core/Duration";
  import type Video from "./core/IVideo";
  import { videoCode } from "./store/store";
  import { onMount } from "svelte";
  import { appWindow, LogicalSize } from "@tauri-apps/api/window";
  import Loader from "./comp/Loader.svelte";

  let selectedVideo: Video | undefined = undefined;
  let isSearching = false;
  let errorMessage = '';
  let code = $videoCode;

  onMount(() => {
    appWindow.setDecorations(true);
    appWindow.setMinSize(new LogicalSize(355, 270));
    appWindow.setMaxSize(new LogicalSize(355, 270));
    appWindow.setSize(new LogicalSize(355, 270));
    appWindow.setResizable(false);

    if (code != '') {
      search();
    }
  });

  /** A parsed version of the code which recognizes URLs */
  const parsedCode = () => {
    //   www.youtube.com/watch?v= | NYqH3HHRebs |
    // music.youtube.com/watch?v= | qqjVwAg5fK4 | &list=RDAMVM4jBDnYE1WjI
    if (code.includes('youtube.com')) {
      for (let i = 0; i < code.length; i++) {
        if (code[i] === '=') {
          return code.slice(i + 1, i + 12);
        }
      }
    }
    // youtu.be/ | NYqH3HHRebs |
    else if (code.includes('youtu.be')) {
      return code.slice(code.length - 11, code.length);
    } else return code;
  };

  /** Search for the video information connected to the current code */
  const search = () => {
    isSearching = true;
    selectedVideo = undefined;
    errorMessage = '';
    invoke('get_video_info', { code: parsedCode() }).then((video: Video) => {
      selectedVideo = video;
      isSearching = false;
    }).catch(e => {
      errorMessage = e;
      isSearching = false;
    });
  };

  /** Play the currently selected video */
  const play = () => {
    if (selectedVideo) {
      videoCode.set(parsedCode());
      emit('play_video');
    }
  };
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
      <div class="thumbnail-placeholder" />
    {/if}

    <div class="thumbnail-overlay">
      <Loader isLoading={isSearching} animSpeed={0.2} />
      <div class="error" style="opacity: { errorMessage == '' ? '0' : '1' };"> { errorMessage } </div>
    </div>

    <div class="description">
      { selectedVideo ? selectedVideo.description : '' }
    </div>
  </div>

  <div class="actions">
    <input class="code" type="text" name="video code" placeholder="Youtube link..." bind:value={code}>
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
          height: 42px;

          bottom: 4px;
          left: 0;

          padding-left: 8px;

          color: #ffffffee;
          z-index: 1;
          
          font-weight: 700;
          white-space: normal;
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

          top: 8px;
          right: 8px;

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

      .thumbnail-overlay {
        position: absolute;

        top: 17px;
        left: 17px;

        width: 320px;
        min-width: 320px;
        height: 180px;

        .error {
          position: absolute;

          top: 0;
          left: 0;

          width: 100%;
          height: 100%;

          pointer-events: none;

          text-align: center;
          display: flex;
          justify-content: center;
          align-items: center;

          color: #ef5859cc;
          font-family: FiraCode;
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
      border-top: 1.5px solid #404040;

      .code {
        flex-grow: 1;
        margin-right: 12px;
        height: 26px;
        font-family: FiraCode;

        background-color: #313131;
        border-radius: 6px;
        border: 1.5px solid #404040;
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
            transition: filter 0.1s;

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

        .search {
          position: relative;
          margin-right: 8px;
        }

        .play {
          position: relative;
        }
      }
    }
  }
</style>