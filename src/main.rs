use clap::Parser;
use gtk::gdk::Display;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Label, Orientation, ScrolledWindow};
use gtk::{CssProvider, glib, prelude::*};
use gtk4_layer_shell::LayerShell;
use log::{debug, error, info, warn};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the CSS file to apply
    #[arg(short, long, default_value = "style.css")]
    style: PathBuf,
    /// Path to the Pango markup file to display
    #[arg(short, long, default_value = "cheatsheet.pango")]
    file: PathBuf,
}

const APP_ID: &str = "org.gtk.HelloWorld";

const MIN_WIDTH: i32 = 400;
const MIN_HEIGHT: i32 = 300;

fn main() -> glib::ExitCode {
    // Initialize the logger
    env_logger::init();

    info!("=== Sway Cheatsheet Application Starting ===");
    info!(
        "Working directory: {:?}",
        std::env::current_dir().unwrap_or_default()
    );
    info!(
        "Command line args: {:?}",
        std::env::args().collect::<Vec<_>>()
    );

    let args = match Args::try_parse() {
        Ok(args) => {
            info!("Successfully parsed arguments: {:?}", args);
            args
        }
        Err(e) => {
            error!("Failed to parse command line arguments: {}", e);
            std::process::exit(1);
        }
    };

    // Build the GTK Application
    let app = Application::builder().application_id(APP_ID).build();
    info!("Created GTK application with ID: {}", APP_ID);
    info!("File to load: {}", args.file.display());

    // Pass the file path to the UI builder
    let file_path = args.file.clone();
    let style_path = args.style.clone();
    app.connect_startup(move |_| load_css(&style_path));
    app.connect_activate(move |app| build_ui(app, &file_path));

    info!("Running GTK application...");
    // Run with empty args to prevent GTK from parsing our custom arguments
    let exit_code = app.run_with_args(&[] as &[String]);
    info!("Application exited with code: {:?}", exit_code);
    exit_code
}

fn load_css(style_path: &PathBuf) {
    info!("=== Loading CSS ===");
    info!("CSS file path: {}", style_path.display());
    info!("CSS file exists: {}", style_path.exists());
    
    let provider = CssProvider::new();
    
    match fs::read_to_string(style_path) {
        Ok(css_content) => {
            info!("Successfully loaded CSS file ({} bytes)", css_content.len());
            debug!("CSS content preview: {}", &css_content[..css_content.len().min(200)]);
            
            provider.load_from_string(&css_content);
            info!("CSS loaded successfully");
            
            // Add the provider to the default screen
            gtk::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            info!("CSS provider added to display");
        }
        Err(e) => {
            error!("Failed to read CSS file '{}': {}", style_path.display(), e);
            warn!("Continuing without custom CSS");
        }
    }
}

fn build_ui(app: &Application, file_path: &PathBuf) {
    info!("=== Building UI ===");
    info!("File path to load: {}", file_path.display());
    info!("File path is absolute: {}", file_path.is_absolute());
    info!("File exists: {}", file_path.exists());

    // Load cheatsheet.pango content
    let cheatsheet = match fs::read_to_string(file_path) {
        Ok(content) => {
            info!("Successfully loaded file content ({} bytes)", content.len());
            debug!(
                "File content preview: {}",
                &content[..content.len().min(100)]
            );
            content
        }
        Err(e) => {
            error!("Failed to read file '{}': {}", file_path.display(), e);
            warn!("Using error message as content");
            format!("Error loading file: {}", e)
        }
    };

    info!("Creating GTK widgets...");
    // Create a Label and set Pango markup
    let label = Label::new(None);
    label.set_markup(&cheatsheet);
    label.set_selectable(true);
    label.set_xalign(0.0); // left align
    label.set_yalign(0.0); // top align
    info!("Created and configured label widget");

    // Put the Label in a ScrolledWindow
    let scrolled = ScrolledWindow::builder()
        .child(&label)
        .min_content_height(MIN_HEIGHT)
        .min_content_width(MIN_WIDTH)
        .build();
    info!("Created scrolled window");

    // Create a vertical box to hold the scrolled window and button
    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.append(&scrolled);
    info!("Created and populated vertical box");

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sway Cheatsheet")
        .default_height(MIN_HEIGHT)
        .default_width(MIN_WIDTH)
        .child(&vbox)
        .build();
    info!("Created application window");

    // Add key handler for ESC to close and F12 to toggle keyboard mode
    let app_clone2 = app.clone();
    let window_clone = window.clone();
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        debug!("Key pressed: {:?}", key);
        match key {
            gtk::gdk::Key::Escape => {
                info!("ESC key pressed - quitting application");
                app_clone2.quit();
                return gtk::glib::Propagation::Stop;
            }
            gtk::gdk::Key::F12 => {
                // Toggle keyboard mode for GTK Inspector access
                let current_mode = window_clone.keyboard_mode();
                let new_mode = match current_mode {
                    gtk4_layer_shell::KeyboardMode::Exclusive => {
                        info!("F12 pressed - switching to OnDemand mode (GTK Inspector accessible)");
                        gtk4_layer_shell::KeyboardMode::OnDemand
                    }
                    _ => {
                        info!("F12 pressed - switching to Exclusive mode (normal operation)");
                        gtk4_layer_shell::KeyboardMode::Exclusive
                    }
                };
                window_clone.set_keyboard_mode(new_mode);
                return gtk::glib::Propagation::Stop;
            }
            _ => {}
        }
        gtk::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);
    info!("Added key handlers (ESC to close, F12 to toggle keyboard mode)");

    info!("Initializing layer shell...");
    window.init_layer_shell();
    gtk::prelude::WidgetExt::realize(&window);
    window.set_layer(gtk4_layer_shell::Layer::Overlay);
    info!("Set window to overlay layer");

    // Sets the Window into focus mode, so no other input is possible until the window is closed.
    // Will be in the app until we finished rendering markup.
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive); // Enable keyboard input
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);
    info!("Configured layer shell: exclusive keyboard mode, right-anchored");

    // Present window
    info!("Presenting window...");
    window.present();
    info!("=== UI Setup Complete ===");
}
