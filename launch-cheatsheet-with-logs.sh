#!/bin/bash
# Sway Cheatsheet Launcher with Logging
# Usage: bindsym $mod+f1 exec ~/code/sway-cheatsheet/launch-cheatsheet-with-logs.sh

# Set log level (debug, info, warn, error)
export RUST_LOG=info

# Set log file location
LOG_FILE="/tmp/sway-cheatsheet.log"

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Log the launch attempt
echo "$(date): Sway cheatsheet launch attempt" >> "$LOG_FILE"
echo "$(date): Working directory: $(pwd)" >> "$LOG_FILE"
echo "$(date): Script directory: $SCRIPT_DIR" >> "$LOG_FILE"
echo "$(date): WAYLAND_DISPLAY: $WAYLAND_DISPLAY" >> "$LOG_FILE"
echo "$(date): DISPLAY: $DISPLAY" >> "$LOG_FILE"

# Run the application with full paths and redirect logs
"$SCRIPT_DIR/target/debug/sway-cheatsheet" \
    -f "$SCRIPT_DIR/cheatsheet.pango" \
    -s "$SCRIPT_DIR/style.css" \
    2>&1 | tee -a "$LOG_FILE"

# Log the exit status
echo "$(date): Application exited with status: $?" >> "$LOG_FILE"
echo "$(date): ---" >> "$LOG_FILE"
