# Gal

Photo gallery POC: Tauri 2 + Svelte 5 (SvelteKit, static adapter) + Rust.

Lists the JPEGs in the `GAL` folder on your Desktop (hardcoded, top level only, `.jpg`/`.jpeg`), newest first.
Shows them in a justified, virtualized grid of disk-cached thumbnails, and opens the full resolution
in a lightbox on click (close with Escape or a click).

## Requirements

- Node.js and [pnpm](https://pnpm.io)
- Rust toolchain and the [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS

## Development

```bash
pnpm install
pnpm tauri dev
```

| Command                                           | What it does                 |
| ------------------------------------------------- | ---------------------------- |
| `pnpm test`                                       | Frontend unit tests (vitest) |
| `cargo test --manifest-path src-tauri/Cargo.toml` | Rust unit tests              |
| `pnpm check`                                      | svelte-check / type check    |
| `pnpm format` / `pnpm format:check`               | Prettier (rustfmt for Rust)  |
| `pnpm tauri build`                                | Production bundle            |

## How it works

- **Listing** (`src-tauri/src/photos.rs`): the `list_photos` command reads the folder and returns id, name,
  EXIF-oriented dimensions and modified time. Only the JPEG header is read, no pixel decode. A photo id is a
  hash of path + size + mtime, so an edited file gets a new id and misses every cache.
- **Thumbnails** (`src-tauri/src/thumbs.rs`): generated lazily on first request, 512 px tall, JPEG quality 80,
  EXIF orientation applied, stored as `<id>.jpg`.
- **Serving**: images are served through a custom `gal://` protocol (`/thumb/<id>` and `/full/<id>`), so the
  webview never gets raw filesystem access. Ids are resolved against the last listing.
- **Layout** (`src/lib/justified.ts`): a justified row layout via `justified-layout`, with only the tiles in
  the viewport rendered.

## Cache

Thumbnails and the dimensions cache (`dims.json`) live in the app cache directory:

- macOS: `~/Library/Caches/dev.vancura.gal/`
- Windows: `%LOCALAPPDATA%\dev.vancura.gal\`
- Linux: `~/.cache/dev.vancura.gal/`

Delete it to regenerate.

## POC limitations

- The library folder is not configurable and is not scanned recursively.
- The list is read once at startup; restart the app to pick up new photos.
- Only JPEG is supported.
- Thumbnails come from a full decode of the source image, which will be slow for very large backfills.
