<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { secondsToDuration } from "./core/Duration";
  import type Video from "./core/IVideo";

  let selectedVideo: Video | undefined = undefined;
  let code = 'wXF3HIhnUOg';

  function search() {
    invoke('get_video_info', { code: code }).then((video: Video) => {
      selectedVideo = video;
    });
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
      <button class="search" on:click={search}>
        <img
          src="https://api.iconify.design/fa:search.svg"
          alt="pin"
        />
      </button>
      <button class="play">
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

      background-color: #292b2f;

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

          background-color: #222427;

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

      border-top: 1.5px solid #424448;

      .code {
        width: 128px;
        height: 26px;
        font-family: FiraCode;

        background-color: #292b2f;
        border-radius: 6px;
        border: 1.5px solid #424448;
        outline: none;

        color: #ffffffbb;
        padding-left: 8px;

        &:focus {
          border: 1.5px solid #5ea5f666;
        }
      }

      .buttons {
        button {
          width: 30px;
          height: 30px;

          background-color: #292b2f;
          border-radius: 6px;
          border: 1.5px solid #424448;
          outline: none;

          cursor: pointer;
          user-select: none;
        }

        .search {
          background-color: rgb(46, 61, 68);
          border-color:rgb(69, 85, 96);

          margin-right: 8px;

          &:hover {
            border-color:rgb(87, 107, 120);
          }

          img {
            width: 100%;
            height: 100%;

            filter: invert(80%);
          }
        }

        .play {
          background-color: rgb(46, 68, 56);
          border-color:rgb(69, 96, 82);

          &:hover {
            border-color:rgb(80, 123, 103);
          }

          img {
            width: 100%;
            height: 100%;

            filter: invert(80%);
          }
        }
      }
    }
  }
</style>