mod main;

use gtk4::prelude::*;
use gtk4::Application;

pub fn gui_main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(main::build_ui);

    // Run the application
    app.run();
}
