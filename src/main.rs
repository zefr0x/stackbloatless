use adw::prelude::*;
use relm4::gtk;

mod api;
mod gui;
mod i18n;
mod utils;

const APP_ID: &str = "io.github.zefr0x.stackbloatless";

// Create a communication channel to communicate with the main app component.
static BASE_BROKER: relm4::MessageBroker<gui::main_window::AppInput> = relm4::MessageBroker::new();

fn main() {
    let base_app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(relm4::gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    let sender = BASE_BROKER.sender();

    base_app.connect_open(
        gtk::glib::clone!(@strong sender => move |application, files, _hint| {
            let uris = files.iter().map(|file| file.uri().to_string()).collect::<Vec<String>>();

            for uri in uris {
                sender.send(gui::main_window::AppInput::RequestPagesByUri(uri)).unwrap();
            }

            application.activate();
        }),
    );

    // FIX: Close remote instance after sending the `open` signal.

    relm4::RelmApp::from_app(base_app)
        .with_args(std::env::args().collect::<Vec<String>>())
        .with_broker(&BASE_BROKER)
        .run_async::<gui::main_window::AppModel>(gui::main_window::AppInit {});
}
