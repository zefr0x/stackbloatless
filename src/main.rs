use adw::prelude::*;
use relm4::gtk;

mod api;
mod gui;
mod i18n;
mod utils;

const APP_ID: &str = "io.github.zefr0x.stackbloatless";

fn main() {
    let base_app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(relm4::gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // Create a communication channel to send messages to the app component.
    let (sender, receiver) = relm4::channel::<gui::main_window::AppInput>();

    base_app.connect_open(
        gtk::glib::clone!(@strong sender => move |_application, files, _hint| {
            let uris = files.iter().map(|file| file.uri().to_string()).collect::<Vec<String>>();

            for uri in uris {
                sender.send(gui::main_window::AppInput::RequestPagesByUri(uri)).unwrap();
            }
        }),
    );

    // FIX: Close remote instance after sending the `open` signal.

    base_app.connect_startup(|application| {
        application.activate();
    });

    relm4::RelmApp::from_app(base_app)
        .with_args(std::env::args().collect::<Vec<String>>())
        .run_async::<gui::main_window::AppModel>(gui::main_window::AppInit { receiver });
}
