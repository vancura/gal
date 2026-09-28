# Gal

Photo gallery POC: Tauri 2 + Svelte 5 + Rust.

Lists JPEGs in `~/Desktop/GAL` (hardcoded, top level only), shows them in a
justified, virtualized grid of disk-cached thumbnails, and opens the full
resolution on click.

```bash
pnpm install
pnpm tauri dev
pnpm test          # vitest
cargo test --manifest-path src-tauri/Cargo.toml
```

Thumbnails and the dimensions cache live in the app cache directory
(`~/Library/Caches/cz.vancura.gal/`). Delete it to regenerate.
