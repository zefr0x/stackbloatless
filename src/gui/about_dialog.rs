use relm4::{
    adw::prelude::*,
    component::{Component, ComponentParts},
    prelude::*,
    ComponentSender,
};

use super::main_window::{build_time, APP_NAME};
use crate::fl;

pub struct AboutWindow {
    developers_list: Vec<String>,
    windowing_backend_name: String,
}

pub struct AboutWindowWidgets {
    main_parent_window: gtk::Window,
}

#[derive(Debug)]
pub enum AboutWindowInput {
    ShowWindow,
}

impl Component for AboutWindow {
    type Init = gtk::Window;
    type Root = ();
    type Widgets = AboutWindowWidgets;
    type Input = AboutWindowInput;
    type CommandOutput = ();
    type Output = ();

    fn init_root() -> Self::Root {}

    fn init(
        main_window: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let developers_list = env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(|s| s.to_owned())
            .collect();

        let windowing_backend_name = match gtk::gdk::Display::default() {
            Some(display) => {
                match display.backend() {
                    gtk::gdk::Backend::Wayland => "Wayland".to_owned(),
                    gtk::gdk::Backend::X11 => "X11".to_owned(),
                    // When unsupported windowing system is used: win32, macos, broadway.
                    _ => "Unsupported".to_owned(),
                }
            }
            None => "Undetected".to_owned(),
        };

        let model = Self {
            developers_list,
            windowing_backend_name,
        };

        let widgets = AboutWindowWidgets {
            main_parent_window: main_window,
        };

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AboutWindowInput::ShowWindow => {
                let about_window = adw::AboutWindow::builder()
                    .application_name(APP_NAME)
                    .version(env!("CARGO_PKG_VERSION"))
                    .license_type(gtk::License::Gpl30Only)
                    .comments(env!("CARGO_PKG_DESCRIPTION"))
                    .developers(self.developers_list.clone())
                    .website(env!("CARGO_PKG_HOMEPAGE"))
                    .issue_url("https://github.com/zefr0x/stackbloatless/issues")
                    .application(&relm4::main_application())
                    .transient_for(&widgets.main_parent_window)
                    .debug_info(format!(
                        "[rust]\n\
                        {}\n\
                        {}\n\
                        {}\n\
                        {}\n\n\
                        [traget]\n\
                        {}\n\n\
                        [source]\n\
                        branch: {}\n\
                        commit: {}\n\
                        clean: {}\n\n\
                        [runtime]\n\
                        GTK: {}.{}.{}\n\
                        Adwaita: {}.{}.{}\n\
                        Cairo: {}\n\
                        Pango: {}\n\
                        GDK Windowing Backend: {}\n\
                        Session Desktop: {}\n\
                        Current Desktop: {}",
                        build_time::BUILD_OS,
                        build_time::CARGO_VERSION,
                        build_time::RUST_CHANNEL,
                        build_time::RUST_VERSION,
                        build_time::BUILD_TARGET,
                        build_time::BRANCH,
                        build_time::COMMIT_HASH,
                        build_time::GIT_CLEAN,
                        gtk::major_version(),
                        gtk::minor_version(),
                        gtk::micro_version(),
                        adw::major_version(),
                        adw::minor_version(),
                        adw::micro_version(),
                        gtk::cairo::version_string(),
                        gtk::pango::version_string(),
                        self.windowing_backend_name,
                        std::env::var("XDG_SESSION_DESKTOP").unwrap_or("Undetected".to_owned()),
                        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("Undetected".to_owned()),
                    ))
                    .build();

                about_window.add_link(
                    &fl!("release-notes"),
                    "https://github.com/zefr0x/stackbloatless/blob/main/CHANGELOG.md",
                );

                about_window.present();
            }
        }
    }
}
