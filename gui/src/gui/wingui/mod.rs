#![windows_subsystem = "windows"]

mod main;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwg::NativeUi;

pub fn gui_main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = main::BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
