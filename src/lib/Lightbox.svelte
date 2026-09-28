<script lang="ts">
  import { fullUrl, thumbUrl, type Photo } from './photos'

  let { photo, onclose }: { photo: Photo; onclose: () => void } = $props()
  let loaded = $state(false)
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && onclose()} />

<button class="backdrop" aria-label="Close" onclick={onclose}>
  {#if !loaded}
    <img class="preview" src={thumbUrl(photo.id)} alt="" />
  {/if}
  <img
    class="full"
    class:loaded
    src={fullUrl(photo.id)}
    alt={photo.name}
    onload={() => (loaded = true)}
  />
</button>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    display: grid;
    place-items: center;
    padding: 0;
    border: 0;
    background: #000;
    cursor: zoom-out;
  }
  .backdrop img {
    grid-area: 1 / 1;
    max-width: 100vw;
    max-height: 100vh;
    object-fit: contain;
  }
  .full {
    opacity: 0;
    transition: opacity 150ms;
  }
  .full.loaded {
    opacity: 1;
  }
</style>
