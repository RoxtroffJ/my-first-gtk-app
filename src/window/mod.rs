mod imp;
mod builder;
use builder::*;

use gtk::glib;

glib::wrapper! {
    pub struct SavingWindow(ObjectSubclass<imp::SavingWindow>)
        @extends gtk::Widget, gtk::Window, 
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SavingWindow {
    pub fn builder() -> SavingWindowBuilder {
        SavingWindowBuilder::new()
    }
}