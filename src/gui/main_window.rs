use relm4::{
    actions::AccelsPlus,
    adw::{self, prelude::*},
    component::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncComponentSender,
        AsyncController, Connector,
    },
    factory::{FactoryVecDeque, FactoryView},
    loading_widgets::LoadingWidgets,
    prelude::*,
};
use relm4_icons::icon_names;

use super::about_dialog::{AboutWindow, AboutWindowInput};
use super::componant_builders;
use super::side_bar;
use crate::api::stackexchange;
use crate::fl;

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

pub struct AppInit {}

pub struct AppModel {
    stackexchange_client: stackexchange::StackExchange,
    side_bar_controller: AsyncController<side_bar::SideBarModel>,
    about_window_connector: Connector<AboutWindow>,
}

pub struct AppWidgets {
    questions_tabs: FactoryVecDeque<componant_builders::QuestionPageModel>,
    sidebar_toggle_button: gtk::ToggleButton,
}

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

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        let spinner = gtk::Spinner::builder()
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        spinner.start();

        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        _init: Self::Init,
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
        provider.load_from_string(include_str!("style.css"));
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Load icons
        relm4_icons::initialize_icons();

        let main_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Create search entry
        // TODO: Use the search entry for searching inside a page.
        // TODO: Use the new tab page as a search page.
        let search_entry = gtk::SearchEntry::builder()
            // TODO: Make clickable icon to select a stackexchange site to search in.
            .placeholder_text(fl!("search-entry-placeholder"))
            .build();

        search_entry.connect_activate(gtk::glib::clone!(@strong sender => move |entry| {
            let search_term = entry.text();
            // TODO: Change how search_term is parsed to support urls and terms at the same time.
            // TODO: Connect it to search api
            // TODO: Don't accept uris.
            // TODO: Support all stackexchange sites: https://api.stackexchange.com/docs/sites
            sender.input(AppInput::RequestPagesByUri(format!("stackbloatless://stackoverflow/{search_term}")));
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
                &fl!("about") => AboutAction,
                &fl!("quit") => QuitAction
            }
        }

        relm4::main_application().set_accelerators_for_action::<ToggleSideBarAction>(&["F9"]);
        relm4::main_application().set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        // Create hamburger menu
        let menu_button = gtk::MenuButton::builder()
            .icon_name(icon_names::MENU_LARGE)
            .menu_model(&main_menu)
            .build();

        header.pack_start(&menu_button);

        // TODO: Create copy question link button (after encapsulating questions in relm components).

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
                &fl!("toggle-pin") => PinTabAction,
                &fl!("close") => CloseTabAction,
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

        // Create tabs factory to manage tabs.
        let questions_tabs = FactoryVecDeque::<componant_builders::QuestionPageModel>::builder()
            .launch(tab_view)
            .forward(sender.input_sender(), |_message| unreachable!());
        // A refrence for TabView that is owned by FactoryVecDeque.
        let tab_view = questions_tabs.widget();

        // TODO: Implement close event handler.
        tab_view.connect_close_page(gtk::glib::clone!(@strong sender => move |tab_view, page| {
            // TODO: Remove them from the FactoryVecDeque.
            tab_view.factory_remove(page);
            true
        }));

        // FIX: Sync position: https://github.com/Relm4/Relm4/issues/573
        // tab_view.connect_page_reordered(|tab_view, tab_page, position| {
        //     tab_view.factory_update_position(tab_page, &());
        // });

        tab_bar.set_view(Some(tab_view));

        tab_view.connect_setup_menu(|view, page| {
            if let Some(page) = page {
                view.set_selected_page(page);
            }
        });

        // Create split view for the sidebar and the tab view
        let split_view = adw::OverlaySplitView::builder()
            .content(tab_view)
            .sidebar(model.side_bar_controller.widget())
            .sidebar_position(gtk::PackType::End)
            .show_sidebar(false)
            .build();
        main_layout.append(&split_view);

        // Create tab button in the header
        let tab_button = adw::TabButton::builder()
            .view(tab_view)
            .action_name("overview.open")
            .build();
        header.pack_end(&tab_button);

        // Create tabs overview
        // FIX: Whene the last tab is closed, close the overview.
        let tab_overview = adw::TabOverview::builder()
            .view(tab_view)
            // TODO: Implement new tab
            // .enable_new_tab(true)
            .child(&main_layout)
            .build();
        root.set_content(Some(&tab_overview));

        // Create sidebar button in the header
        let sidebar_toggle_button = gtk::ToggleButton::builder()
            .icon_name(icon_names::DOCK_RIGHT)
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
            questions_tabs,
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
                    widgets
                        .questions_tabs
                        .guard()
                        .push_front(componant_builders::QuestionPageInit { question });
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
                let selected_page = widgets.questions_tabs.widget().selected_page().unwrap();

                widgets
                    .questions_tabs
                    .widget()
                    .set_page_pinned(&selected_page, !selected_page.is_pinned())
            }
            AppInput::CloseTab => {
                let tab_view = widgets.questions_tabs.widget();
                let selected_page = tab_view.selected_page().unwrap();

                // Ask before closing a pinned tab
                if selected_page.is_pinned() {
                    let warning_message = adw::AlertDialog::builder()
                        .heading(fl!("close-pinned-tab-confirmation-header"))
                        .body(fl!("close-pinned-tab-confirmation-body"))
                        .build();

                    warning_message.add_responses(&[("yes", &fl!("yes")), ("no", &fl!("no"))]);
                    warning_message.set_default_response(Some("no"));
                    warning_message
                        .set_response_appearance("yes", adw::ResponseAppearance::Destructive);

                    warning_message.present(&relm4::main_application().active_window().unwrap());

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
                    // FIX: Find the right index.
                    let page_index = tab_view.page_position(&selected_page) as usize;

                    widgets.questions_tabs.guard().remove(page_index);
                }
            }
            AppInput::ClosePinnedTab => {
                let tab_view = widgets.questions_tabs.widget();
                let selected_page = tab_view.selected_page().unwrap();

                tab_view.set_page_pinned(&selected_page, false);

                // FIX: Find the right index.
                let page_index = tab_view.page_position(&selected_page) as usize;

                widgets.questions_tabs.guard().remove(page_index);
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {}
}
