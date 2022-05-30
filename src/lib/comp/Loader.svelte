<script lang="ts">

  export let isLoading: Boolean;
  export let animSpeed: Number = 0.5;

</script>


<div class="loader" style="opacity: { isLoading ? '0.8' : '0' }; transition: opacity { animSpeed }s;">
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


<style lang="scss">
  .loader {
    position: relative;

    width: 100%;
    height: 100%;

    margin: 0;
    padding: 0;

    pointer-events: none;

    svg {
      position: absolute;

      top: calc(50% - 20px);
      left: calc(50% + 20px);

      transform: translate(-50%, -50%);

      width: 100px;
      height: 100px;
    }
  }
</style>
