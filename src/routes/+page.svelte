<script lang="ts">
  import { onMount } from 'svelte'
  import { listPhotos, thumbUrl, type Photo } from '$lib/photos'
  import { layout, visibleIndices } from '$lib/justified'
  import Lightbox from '$lib/Lightbox.svelte'

  let photos = $state<Photo[]>([])
  let error = $state<string | null>(null)
  let width = $state(0)
  let viewportHeight = $state(0)
  let scrollTop = $state(0)
  let open = $state<Photo | null>(null)

  const lay = $derived(layout(photos.map((p) => p.width / p.height), width))
  const indices = $derived(visibleIndices(lay.boxes, scrollTop, viewportHeight))

  onMount(async () => {
    try {
      photos = await listPhotos()
    } catch (e) {
      error = String(e)
    }
  })
</script>

<main
  bind:clientWidth={width}
  bind:clientHeight={viewportHeight}
  onscroll={(e) => (scrollTop = e.currentTarget.scrollTop)}
>
  {#if error}
    <p class="msg">{error}</p>
  {:else if photos.length === 0}
    <p class="msg">No JPEGs in ~/Desktop/GAL</p>
  {/if}
  <div class="grid" style:height="{lay.height}px">
    {#each indices as i (photos[i].id)}
      {@const b = lay.boxes[i]}
      <button
        class="tile"
        style:top="{b.top}px"
        style:left="{b.left}px"
        style:width="{b.width}px"
        style:height="{b.height}px"
        onclick={() => (open = photos[i])}
      >
        <img src={thumbUrl(photos[i].id)} alt={photos[i].name} loading="lazy" decoding="async" />
      </button>
    {/each}
  </div>
</main>

{#if open}
  <Lightbox photo={open} onclose={() => (open = null)} />
{/if}

<style>
  :global(body) {
    margin: 0;
    background: #111;
    color: #ccc;
    font: 14px system-ui, sans-serif;
  }
  main {
    height: 100vh;
    overflow-y: auto;
  }
  .msg {
    padding: 2rem;
    text-align: center;
  }
  .grid {
    position: relative;
  }
  .tile {
    position: absolute;
    padding: 0;
    border: 0;
    background: #222;
    cursor: zoom-in;
    overflow: hidden;
  }
  .tile img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>
