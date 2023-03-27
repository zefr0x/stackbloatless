use adw::prelude::*;

mod api;
mod gui;

const APP_ID: &str = "io.github.zer0-x.stackbloatless";

fn main() {
    let base_app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(relm4::gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    base_app.connect_open(|_application, _files, _hint| {
        // TODO: Handle open before startup, so it work when first start the application.
    });

    base_app.connect_startup(|application| {
        application.activate();
    });

    let args = std::env::args().collect::<Vec<String>>();

    relm4::RelmApp::with_app(base_app).run_async_with_args::<gui::main_window::AppModel, String>(
        (),
        &args,
    );
}
