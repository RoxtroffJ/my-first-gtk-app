use std::cell::OnceCell;

use gtk::gio::Settings;
use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use glib::Properties;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::SavingWindow)]
pub struct SavingWindow{
    #[property(get, set)]
    settings: OnceCell<Settings>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SavingWindow {
    const NAME: &'static str = "SavingWindow";
    type Type = super::SavingWindow;
    type ParentType = gtk::Window;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for SavingWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.bind_property("settings", obj.as_ref(), "default-width")
            .transform_to(|_, settings: Settings| {
                Some(settings.int("window-width"))
            })
            .build();
        obj.bind_property("settings", obj.as_ref(), "default-height")
            .transform_to(|_, settings: Settings| {
                Some(settings.int("window-height"))
            })
            .build();
        obj.bind_property("settings", obj.as_ref(), "maximized")
            .transform_to(|_, settings: Settings| {
                Some(settings.boolean("is-maximized"))
            })
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SavingWindow {}

// Trait shared by all windows
impl WindowImpl for SavingWindow {
    fn close_request(&self) -> glib::Propagation {
        let settings = self.settings.get().expect("Properties not set at closing, which should be impossible");
        let obj = self.obj();

        settings.set_int("window-width", obj.width()).expect("Couldn't set setting \"window-width\"");
        settings.set_int("window-height", obj.height()).expect("Couldn't set setting \"window-height\"");
        settings.set_boolean("is-maximized", obj.is_maximized()).expect("Couldn't set setting \"is-maximized\"");

        glib::Propagation::Proceed
    }
}