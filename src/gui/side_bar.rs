use relm4::{
    component::{AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent},
    gtk::prelude::*,
    prelude::*,
};
use relm4_icons::icon_name;

use crate::fl;

pub struct SideBarModel;

pub struct SideBarWidgest;

#[derive(Debug)]
pub struct SideBarInput;

#[relm4::async_trait::async_trait(?Send)]
impl SimpleAsyncComponent for SideBarModel {
    type Init = ();
    type Root = gtk::Box;
    type Widgets = SideBarWidgest;
    type Input = SideBarInput;
    type Output = ();

    fn init_root() -> Self::Root {
        gtk::Box::new(gtk::Orientation::Vertical, 0)
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = SideBarModel {};

        let side_bar_view = adw::ViewStack::builder()
            .css_classes(["background"])
            .vexpand(true)
            .build();

        root.append(
            &adw::ViewSwitcher::builder()
                .stack(&side_bar_view)
                .css_classes(["background"])
                .policy(adw::ViewSwitcherPolicy::Wide)
                .build(),
        );

        root.append(&side_bar_view);

        // Side bar pages
        // TODO: Implement bookmarks
        side_bar_view.add_titled_with_icon(
            &adw::StatusPage::builder()
                .title("Bookmarks")
                .child(&gtk::Label::new(Some(&fl!("placeholder"))))
                .icon_name(icon_name::LIBRARY)
                .build(),
            None,
            "Bookmarks",
            icon_name::LIBRARY,
        );

        // TODO: Implement history
        side_bar_view.add_titled_with_icon(
            &adw::StatusPage::builder()
                .title("History")
                .child(&gtk::Label::new(Some(&fl!("placeholder"))))
                .icon_name(icon_name::HISTORY_UNDO)
                .build(),
            None,
            "History",
            icon_name::HISTORY_UNDO,
        );

        let widgets = SideBarWidgest {};

        AsyncComponentParts { model, widgets }
    }
}
