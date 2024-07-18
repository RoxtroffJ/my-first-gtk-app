mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl From<i32> for IntegerObject {
    fn from(value: i32) -> Self {
        Object::builder().property("value", value).build()
    }
}

impl Default for IntegerObject {
    fn default() -> Self {
        0.into()
    }
}