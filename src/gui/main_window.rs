use adw::prelude::*;
use relm4::{
    actions::AccelsPlus,
    adw::traits::AdwWindowExt,
    component::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncComponentSender,
        AsyncController, Connector,
    },
    gtk::{
        prelude::ApplicationExt,
        traits::{GtkApplicationExt, WidgetExt},
    },
    loading_widgets::LoadingWidgets,
    prelude::*,
};
use relm4_icons::icon_name;

use super::about_dialog::{AboutWindow, AboutWindowInput};
use super::componant_builders;
use super::side_bar;
use crate::api::stackexchange;

// Save build-time informations
shadow_rs::shadow!(build_time);

pub const APP_NAME: &str = "StackBloatLess";

#[derive(Debug, Clone)]
pub enum AppInput {
    RequestPagesByUri(stackexchange::Uri),
    ShowAboutWindow,
    ToggleSideBar,
    Quit,
    ToggleSelectedTabPin,
    CloseTab,
    ClosePinnedTab,
}

pub struct AppInit {
    pub receiver: relm4::Receiver<AppInput>,
}

pub struct AppModel {
    stackexchange_client: stackexchange::StackExchange,
    side_bar_controller: AsyncController<side_bar::SideBarModel>,
    about_window_connector: Connector<AboutWindow>,
}

pub struct AppWidgets {
    tab_view: adw::TabView,
    sidebar_toggle_button: gtk::ToggleButton,
}

#[relm4::async_trait::async_trait(?Send)]
impl AsyncComponent for AppModel {
    type Init = AppInit;
    type Root = adw::Window;
    type Widgets = AppWidgets;
    type Input = AppInput;
    type CommandOutput = ();
    type Output = ();

    fn init_root() -> Self::Root {
        adw::Window::builder().title(APP_NAME).build()
    }

    fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
        let spinner = gtk::Spinner::builder()
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        spinner.start();

        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = AppModel {
            stackexchange_client: stackexchange::StackExchange::new(),
            side_bar_controller: side_bar::SideBarModel::builder()
                .launch(())
                .forward(sender.input_sender(), |_message| unreachable!()),
            about_window_connector: AboutWindow::builder()
                .launch(relm4::main_application().active_window().unwrap()),
        };

        // Load CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Load icons
        relm4_icons::initialize_icons();

        // Listen to messages sent from the main function.
        sender.oneshot_command(
            init.receiver
                .forward(sender.input_sender().to_owned(), |msg| msg),
        );

        let main_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Create search entry
        // TODO: Use the search entry for searching inside a page.
        // TODO: Use the new tab page as a search page.
        let search_entry = gtk::SearchEntry::builder()
            // TODO: Make clickable icon to select a stackexchange site to search in.
            .placeholder_text("Enter a search term or question id")
            .build();

        search_entry.connect_activate(gtk::glib::clone!(@strong sender => move |entry| {
            let search_term = entry.text();
            // TODO: Change how search_term is parsed to support urls and terms at the same time.
            // TODO: Connect it to search api
            // TODO: Don't accept uris.
            // TODO: Support all stackexchange sites: https://api.stackexchange.com/docs/sites
            sender.input(AppInput::RequestPagesByUri(format!("stackexchange://stackoverflow/{search_term}")));
            entry.delete_text(0, search_term.len() as i32);
        }));

        // Create header bar
        let header = adw::HeaderBar::builder()
            .title_widget(&search_entry)
            .hexpand(true)
            .show_end_title_buttons(true)
            .build();

        main_layout.append(&header);

        // Create menu actions
        // TODO: Create action to show GtkShortcutsWindow.
        relm4::new_action_group!(MenuActionGroup, "menu");
        relm4::new_stateless_action!(AboutAction, MenuActionGroup, "about");
        relm4::new_stateless_action!(ToggleSideBarAction, MenuActionGroup, "toggle_sidebar");
        relm4::new_stateless_action!(QuitAction, MenuActionGroup, "quit");
        {
            let mut group = relm4::actions::RelmActionGroup::<MenuActionGroup>::new();

            let about_action: relm4::actions::RelmAction<AboutAction> =
                relm4::actions::RelmAction::new_stateless(
                    gtk::glib::clone!(@strong sender => move |_| {
                        sender.input(AppInput::ShowAboutWindow);
                    }),
                );
            group.add_action(about_action);

            let toggle_sidebar_action: relm4::actions::RelmAction<ToggleSideBarAction> =
                relm4::actions::RelmAction::new_stateless(
                    gtk::glib::clone!(@strong sender => move |_| {
                        sender.input(AppInput::ToggleSideBar);
                    }),
                );
            group.add_action(toggle_sidebar_action);

            let quit_action: relm4::actions::RelmAction<QuitAction> =
                relm4::actions::RelmAction::new_stateless(
                    gtk::glib::clone!(@strong sender => move |_| {
                        sender.input(AppInput::Quit);
                    }),
                );
            group.add_action(quit_action);

            root.insert_action_group("menu", Some(&group.into_action_group()))
        }

        relm4::menu! {
            main_menu: {
                "About" => AboutAction,
                "Quit" => QuitAction
            }
        }

        relm4::main_application().set_accelerators_for_action::<ToggleSideBarAction>(&["F9"]);
        relm4::main_application().set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        // Create hamburger menu
        let menu_button = gtk::MenuButton::builder()
            .icon_name(icon_name::MENU_LARGE)
            .menu_model(&main_menu)
            .build();

        header.pack_start(&menu_button);

        // Create tab actions
        relm4::new_action_group!(TabActionGroup, "tab");
        relm4::new_stateless_action!(PinTabAction, TabActionGroup, "toggle_pin");
        relm4::new_stateless_action!(CloseTabAction, TabActionGroup, "close");
        {
            let mut group = relm4::actions::RelmActionGroup::<TabActionGroup>::new();

            let tab_pin_action: relm4::actions::RelmAction<PinTabAction> =
                relm4::actions::RelmAction::new_stateless(
                    gtk::glib::clone!(@strong sender => move |_| {
                        sender.input(AppInput::ToggleSelectedTabPin);
                    }),
                );
            group.add_action(tab_pin_action);

            let close_tab_action: relm4::actions::RelmAction<CloseTabAction> =
                relm4::actions::RelmAction::new_stateless(
                    gtk::glib::clone!(@strong sender => move |_| {
                        sender.input(AppInput::CloseTab);
                    }),
                );
            group.add_action(close_tab_action);

            root.insert_action_group("tab", Some(&group.into_action_group()))
        }

        relm4::menu! {
            tab_menu: {
                "Pin/Unpin" => PinTabAction,
                "Close" => CloseTabAction,
            }
        }

        relm4::main_application().set_accelerators_for_action::<CloseTabAction>(&["<Control>w"]);

        // Create tab bar
        let tab_bar = adw::TabBar::builder().css_classes(["inline"]).build();
        main_layout.append(&tab_bar);

        // Create tab view
        let tab_view = adw::TabView::builder()
            .menu_model(&tab_menu)
            .margin_top(5)
            .build();

        tab_bar.set_view(Some(&tab_view));

        tab_view.connect_setup_menu(|view, page| {
            if let Some(page) = page {
                view.set_selected_page(page);
            }
        });

        // Create split view for the sidebar and the tab view
        let split_view = adw::OverlaySplitView::builder()
            .content(&tab_view)
            .sidebar(model.side_bar_controller.widget())
            .sidebar_position(gtk::PackType::End)
            .show_sidebar(false)
            .build();
        main_layout.append(&split_view);

        // Create tab button in the header
        let tab_button = adw::TabButton::builder()
            .view(&tab_view)
            .action_name("overview.open")
            .build();
        header.pack_end(&tab_button);

        // Create tabs overview
        // FIX: Whene the last tab is closed, close the overview.
        let tab_overview = adw::TabOverview::builder()
            .view(&tab_view)
            // TODO: Implement new tab
            // .enable_new_tab(true)
            .child(&main_layout)
            .build();
        root.set_content(Some(&tab_overview));

        // Create sidebar button in the header
        let sidebar_toggle_button = gtk::ToggleButton::builder()
            .icon_name(icon_name::DOCK_RIGHT)
            .build();
        header.pack_end(&sidebar_toggle_button);

        sidebar_toggle_button.connect_clicked(
            gtk::glib::clone!(@strong split_view => move |button| {
                split_view.set_show_sidebar(button.is_active());
            }),
        );

        // When sidebar is revealed or hidden change button status.
        split_view.connect_show_sidebar_notify(
            gtk::glib::clone!(@strong sidebar_toggle_button => move |split_view| {
                sidebar_toggle_button.set_active(split_view.shows_sidebar());
            }),
        );

        let widgets = AppWidgets {
            tab_view,
            sidebar_toggle_button,
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::RequestPagesByUri(uri) => {
                // TODO: Handle request errors.
                let questions = self
                    .stackexchange_client
                    .get_questions_from_uri(&uri)
                    .await
                    .unwrap();

                for question in questions {
                    let question_box = componant_builders::st_question(&question);

                    let tab_page = widgets.tab_view.append(
                        &gtk::ScrolledWindow::builder()
                            .child(&question_box)
                            .vexpand(true)
                            .hexpand(true)
                            .build(),
                    );

                    // TODO: Pass question tags as keywords.
                    // tab_page.set_keyword(keyword);

                    tab_page.set_title(&question.title);
                }
            }
            AppInput::ShowAboutWindow => {
                self.about_window_connector
                    .sender()
                    .send(AboutWindowInput::ShowWindow)
                    .unwrap();
            }
            AppInput::ToggleSideBar => {
                widgets.sidebar_toggle_button.emit_clicked();
            }
            AppInput::Quit => {
                relm4::main_application().quit();
            }
            AppInput::ToggleSelectedTabPin => {
                let selected_page = widgets.tab_view.selected_page().unwrap();

                widgets
                    .tab_view
                    .set_page_pinned(&selected_page, !selected_page.is_pinned())
            }
            AppInput::CloseTab => {
                let selected_page = widgets.tab_view.selected_page().unwrap();

                // Ask before closing a pinned tab
                if selected_page.is_pinned() {
                    let warning_message = adw::MessageDialog::builder()
                        .transient_for(&relm4::main_application().active_window().unwrap())
                        .heading("Close pinned tab?")
                        .body("Do you really want to close a pinned tab?")
                        .build();

                    warning_message.add_responses(&[("yes", "Yes"), ("no", "No")]);
                    warning_message.set_default_response(Some("no"));
                    warning_message
                        .set_response_appearance("yes", adw::ResponseAppearance::Destructive);

                    warning_message.show();

                    warning_message.connect_response(
                        None,
                        gtk::glib::clone!(@strong sender => move |dialog, responde| {
                            if responde == "yes" {
                                sender.input(AppInput::ClosePinnedTab);
                            }
                            dialog.close();
                        }),
                    );
                } else {
                    widgets.tab_view.close_page(&selected_page);
                }
            }
            AppInput::ClosePinnedTab => {
                let selected_page = widgets.tab_view.selected_page().unwrap();

                widgets.tab_view.set_page_pinned(&selected_page, false);
                widgets.tab_view.close_page(&selected_page);
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {}
}
