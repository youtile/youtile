<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Home from "./lib/Home.svelte";
  import Theater from "./lib/Theater.svelte";

  let isPlaying = false;

  listen('play_video', () => {
    isPlaying = true;
  });

  listen('stop_video', () => {
    isPlaying = false;
  });
</script>

<main>
  {#if isPlaying}
    <Theater />
  {:else}
    <Home />
  {/if}
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

    background-color: #1c1c1c;
    overflow: hidden;
  }
</style>
