use std::cell::*;
use std::sync::OnceLock;

use gtk::glib;
use glib::subclass::Signal;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use glib::Properties;

use super::*;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::NumberedButton)]
pub struct NumberedButton {
    #[property(get, set)]
    number : Cell<i32>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NumberedButton {
    const NAME: &'static str = "NumberedButton";
    type Type = super::NumberedButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for NumberedButton {
    fn constructed(&self) {
        self.parent_constructed();

        let button = self.obj();
        button.bind_property("number", button.as_ref(), "label")
            .sync_create()
            .build();

        button.connect_number_notify(|button| {
            if button.number() > MAX_NUMBER {
                button.set_number(MAX_NUMBER);
            }
            else if button.number() < MIN_NUMBER {
                button.set_number(MIN_NUMBER);
            }
            else if button.number() == MAX_NUMBER {
                button.emit_by_name::<()>("max-number-reached", &[&MAX_NUMBER]);
            }
            else if button.number() == MIN_NUMBER {
                button.emit_by_name::<()>("min-number-reached", &[&MIN_NUMBER]);
            }
            else {
                button.emit_by_name::<()>("in-bounds", &[&true]);
            }
        });
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build(),
                
                Signal::builder("min-number-reached")
                .param_types([i32::static_type()])
                .build(),

                Signal::builder("in-bounds")
                .param_types([bool::static_type()])
                .build(),]
        })
    }
}

// Trait shared by all widgets
impl WidgetImpl for NumberedButton {}

// Trait shared by all buttons
impl ButtonImpl for NumberedButton {}
