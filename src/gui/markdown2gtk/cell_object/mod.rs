mod imp;

use relm4::gtk::glib;

glib::wrapper! {
    pub struct CellObject(ObjectSubclass<imp::CellObject>);
}

impl CellObject {
    pub fn new(string: Option<&str>, is_header: bool) -> Self {
        glib::Object::builder()
            .property("string", string)
            .property("isheader", is_header)
            .build()
    }
}
