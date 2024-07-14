use gtk::glib::{self, Object};

mod imp;

pub static MIN_NUMBER: i32 = -5;
pub static MAX_NUMBER: i32 = 5;

glib::wrapper! {
    pub struct NumberedButton(ObjectSubclass<imp::NumberedButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl NumberedButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for NumberedButton {
    fn default() -> Self {
        Self::new()
    }
}