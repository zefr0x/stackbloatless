use relm4::gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::{Cell, RefCell};

#[derive(glib::Properties, Default)]
#[properties(wrapper_type = super::CellObject)]
pub struct CellObject {
    #[property(get, set)]
    string: RefCell<Option<String>>,
    #[property(get, set)]
    isheader: Cell<bool>,
}

#[glib::object_subclass]
impl glib::subclass::types::ObjectSubclass for CellObject {
    const NAME: &'static str = "GtkAppTableCellObject";
    type Type = super::CellObject;
}

#[glib::derived_properties]
impl glib::subclass::object::ObjectImpl for CellObject {}
