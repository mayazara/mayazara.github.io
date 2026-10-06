# Music Wrapper

A lightweight personal desktop wrapper around the Apple Music web player
(music.apple.com), built with [Tauri 2](https://tauri.app). On Windows it uses the
system's WebView2, so the app is a few MB instead of shipping a whole browser.

## What it does so far

- Opens music.apple.com in its own window. Your sign-in persists between launches.
- Remembers window size and position.
- Only one copy runs at a time. Launching it again focuses the existing window.
- Apple domains stay in the app. Any other link opens in your default browser.
- Injects custom CSS and JS into the page (`src-tauri/inject/`):
  - `Ctrl+L` focuses search.
  - `Alt+Left` / `Alt+Right` go back and forward.

## Setup (Windows)

1. Install the [Tauri prerequisites](https://tauri.app/start/prerequisites/):
   - Microsoft C++ Build Tools ("Desktop development with C++")
   - WebView2 (already on Windows 10/11)
   - Rust via [rustup](https://rustup.rs)
2. Install [Node.js](https://nodejs.org) (LTS).
3. In this folder:

   ```sh
   npm install
   npm run dev     # run with DevTools available (right-click > Inspect)
   npm run build   # make an installer in src-tauri/target/release/bundle/nsis/
   ```

The first build compiles all the Rust dependencies and takes a few minutes. Later builds are fast.

## Customizing

| File | What to change |
|---|---|
| `src-tauri/inject/inject.css` | Styles applied on top of the web player |
| `src-tauri/inject/inject.js` | Scripts run in the page (shortcuts, tweaks) |
| `src-tauri/src/lib.rs` | Window setup, allowed domains, native features |

The inject files are compiled into the app, so restart `npm run dev` after editing them.

## Known limits

- Audio quality is whatever the web player gives (no lossless or Dolby Atmos).
- Selectors in `inject.css`/`inject.js` can break when Apple updates the site.
