# 📋 Clipbox

Clipbox is a small clipboard history app built with Tauri, React, TypeScript, and Rust.

It runs as a lightweight desktop utility: copy text normally, open Clipbox with a global shortcut, choose a previous clipboard item, and paste it back into the app you were using.

## 🍎 Platform support

Clipbox is currently macOS-focused.

Several parts of the app depend on macOS-specific behavior, including private Tauri macOS APIs, AppKit window focus handling, accessory app mode, a translucent floating window, and simulated paste behavior using the macOS command key.

Linux and Windows support is TBD. Some code paths exist for those platforms, but the app should be treated as macOS-only until cross-platform behavior is implemented and tested.

## ✨ Features

- Text clipboard history.
- Global shortcut to open the picker at the cursor position.
- Floating, always-on-top clipboard menu.
- Keyboard navigation for selecting, deleting, and pasting items.
- Tray icon so the app can stay resident in the background.
- In-memory history capped at 120 text items.

## ⌨️ Usage

- `Ctrl+Alt+V`: show Clipbox at the current cursor position.
- `ArrowUp` / `ArrowDown`: move through clipboard items.
- `ArrowRight`: focus the delete action for the selected item.
- `ArrowLeft`: return focus to the selected clipboard item.
- `Enter`: paste the selected item, or delete it when the delete action is active.
- `Escape`: hide Clipbox.

Clipbox currently stores text only. Image clipboard support is not implemented yet, and history is not persisted across app restarts.

## 🛠️ Development

Prerequisites:

- macOS.
- Rust and Cargo.
- Bun.
- Tauri development dependencies for macOS.

<details>
<summary>Setup</summary>

```sh
bun install
```

</details>

<details>
<summary>Run the app locally</summary>

Using Bun:

```sh
bun run tauri dev
```

Using Cargo:

```sh
cargo tauri dev
```

Run only the frontend dev server:

```sh
bun run dev
```

</details>

<details>
<summary>Generate release binaries</summary>

Using Bun:

```sh
bun run tauri build
```

Using Cargo:

```sh
cargo tauri build
```

The generated app bundle and installer artifacts are written under `src-tauri/target/release/bundle/`.

Build only the frontend assets:

```sh
bun run build
```

</details>

<details>
<summary>Format the project</summary>

Format Rust code:

```sh
cargo fmt --manifest-path src-tauri/Cargo.toml
```

Format frontend code:

```sh
bun run format
```

</details>

## 🔐 macOS permissions

Because Clipbox listens for global shortcuts, tracks the clipboard, restores focus to the previous app, and simulates paste, macOS may require permissions such as Accessibility or Input Monitoring depending on your system settings.

If the picker opens but cannot paste back into another app, check macOS System Settings privacy permissions for the built Clipbox app or the development terminal running it.
