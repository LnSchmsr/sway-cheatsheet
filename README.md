# sway-cheatsheet

A minimal floating cheatsheet panel for [Sway](https://swaywm.org/) window manager on Wayland, which I made to learn `GTK4` and the `sway` commands.

It displays keboard shortcuts and comma`d referenes in a floating panel. Buitl with Rust and GTK4.

---

## Requirements

### System Dependencies
- **Wayland compositor** (Sway, Hyprland, etc.)
- **GTK4** with layer shell support
- **Rust toolchain** (for building from source)

### Arch Linux / Manjaro
```bash
sudo pacman -S gtk4 gtk4-layer-shell rust
```

### Ubuntu / Debian
```bash
sudo apt install libgtk-4-dev libgtk4-layer-shell-dev build-essential rustc cargo
```

---

## Installation & Usage

### Building from Source
```bash
git clone https://github.com/LnSchmsr/sway-cheatsheet.git
cd sway-cheatsheet
cargo build --release
```

### Running the Application
```bash
# Basic usage (loads default files)
./target/release/sway-cheatsheet

# Custom cheatsheet and styling
./target/release/sway-cheatsheet -f my-shortcuts.pango -s custom-theme.css

# With logging enabled
RUST_LOG=info ./target/release/sway-cheatsheet
```

### Sway Integration
Add to your Sway config (`~/.config/sway/config`):
```bash
# Launch cheatsheet with Mod+F1
bindsym $mod+F1 exec /path/to/sway-cheatsheet/target/release/sway-cheatsheet

# Or use the launcher script for better logging
bindsym $mod+F1 exec /path/to/sway-cheatsheet/launch-cheatsheet-with-logs.sh
```

### Keyboard Controls
- **ESC** - Close the cheatsheet
- **F12** - Toggle keyboard mode (for GTK Inspector access during development)

---

## Configuration

### Cheatsheet Content (`cheatsheet.pango`)
The cheatsheet uses Pango markup for rich text formatting:
```xml
<span size="large" weight="bold">Sway Shortcuts</span>

<tt>$mod+Enter</tt>    Open terminal
<tt>$mod+d</tt>        Application launcher  
<tt>$mod+Shift+q</tt>  Quit application
```

### Custom Styling (`style.css`)
GTK4 CSS for theming the application:
```css
window {
    background: transparent;
}

box {
    background: linear-gradient(145deg, #2c1810 0%, #3d2817 50%, #2c1810 100%);
    border: 3px solid #8b7355;
    border-radius: 12px;
    padding: 20px;
}
```

---

## Command Line Options

```bash
Usage: sway-cheatsheet [OPTIONS]

Options:
  -f, --file <FILE>   Path to the Pango markup file to display [default: cheatsheet.pango]
  -s, --style <FILE>  Path to the CSS file to apply [default: style.css]
  -h, --help          Print help
  -V, --version       Print version
```

---

## Future Ideas

- [ ] Dynamic cheatsheets depending on context (Neovim, Tmux, etc.)
- [ ] Multi-panel support for different categories

---

## Development

### Debugging
```bash
# Enable detailed logging
RUST_LOG=debug ./target/debug/sway-cheatsheet

# View logs when launched from Sway
tail -f /tmp/sway-cheatsheet.log
```

### GTK Inspector
1. Press **F12** to switch to OnDemand keyboard mode
2. Press **Ctrl+Shift+I** to open GTK Inspector
3. Edit CSS live in the inspector
4. Press **F12** again to return to exclusive mode

---

