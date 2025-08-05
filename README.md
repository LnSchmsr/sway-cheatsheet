# sway-cheatsheet

A minimal floating cheatsheet panel for [Sway](https://swaywm.org/) window manager on Wayland.

This project is a learning tool to explore writing custom Wayland overlay applications that integrate well with tiling window managers. Inspired by the `<leader>` key concept from Neovim and LazyVim, this panel shows static keyboard shortcut help — but does not execute commands.

---

## 🚀 Goals

- [x] Display a static cheatsheet as an overlay panel in Sway
- [x] Launch using a custom keybinding (e.g. `Mod+Space`)
- [x] Float above all windows without stealing focus
- [x] Lightweight and fast startup
- [x] Close with a keypress (e.g. `ESC`)

---

## 🧰 Implementation Notes 

This app will likely be implemented using:

- `Rust` or `C`
- `gtk4-layer-shell` or a direct Wayland client library (e.g. `smithay-client-toolkit`)
- Integration via `swaymsg` + `exec` for launching
- Configurable text and style

---

## 🛣️ Future Ideas

- [ ] Support for themes (dark/light)
- [ ] Support for Markdown or keybinding tables
- [ ] Dynamic cheatsheets depending on context (Neovim, Tmux, etc.)
- [ ] Optional mouse interaction

---

## 📦 Status

🚧 In development — designed as a learning project for writing custom Wayland apps and enhancing terminal-based workflows.

---

## 📝 License

MIT (or your preferred license)
