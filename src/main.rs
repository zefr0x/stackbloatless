mod api;
mod gui;

const APP_ID: &str = "io.github.zer0-x.stackbloatless";

fn main() {
    let app = relm4::RelmApp::new(APP_ID);

    app.run_async::<gui::main_window::AppModel>(());
}
