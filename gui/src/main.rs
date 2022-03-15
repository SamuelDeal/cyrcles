#[cfg(feature = "wingui")]
#[path = "gui/wingui/mod.rs"]
pub mod gui;

#[cfg(feature = "gtk")]
#[path = "gui/gtk/mod.rs"]
pub mod gui;

mod gpg;

fn main() {
    if gpg::is_installed() {
        println!("GPG is installed!");
    }
    gui::gui_main();
}
