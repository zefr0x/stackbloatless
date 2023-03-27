use adw::prelude::*;
use relm4::gtk;

mod api;
mod gui;

const APP_ID: &str = "io.github.zer0_x.stackbloatless";

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

    let args = std::env::args().collect::<Vec<String>>();

    let init = gui::main_window::AppInit { receiver };

    relm4::RelmApp::with_app(base_app)
        .run_async_with_args::<gui::main_window::AppModel, String>(init, &args);
}
