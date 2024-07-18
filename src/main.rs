use gtk::gio::Settings;
use gtk::glib::clone;
use gtk::{prelude::*, Button, Label, ListView, ScrolledWindow, SignalListItemFactory, SingleSelection};
use gtk::{glib, Application};
use gtk::gio;

mod numbered_button;
use numbered_button::*;

mod window;
use window::*;

mod integer_object;
use integer_object::*;

const APP_ID: &str = "auth.arthurJOLIVET.myFirstApp.huiefhzeyfhz";
const MARGIN: i32 = 12;


fn build_ui(app: &Application) {
    
    let settings = Settings::new(APP_ID);
    let init_number = settings.int("number");   

    let button_print = NumberedButton::new();
    button_print.set_margin_bottom(MARGIN/2);
    button_print.set_margin_end(MARGIN);
    button_print.set_margin_start(MARGIN);
    button_print.set_margin_top(MARGIN);
    button_print.set_number(init_number);

    settings.bind("number", &button_print, "number").build();

    let button_incr = Button::builder()
        .label("+")
        .margin_bottom(MARGIN/4)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN/2)
        .build();

    button_incr.connect_clicked(clone!(#[weak] button_print, move |_| {
        button_print.set_number(button_print.number()+1);
    }));

    button_print.bind_property("number", &button_incr, "sensitive")
        .transform_to(|_, number: i32| {
            Some(number < MAX_NUMBER)
        })
        .sync_create()
        .build();
    
    let button_decr = Button::builder()
        .label("-")
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN/4)
        .build();


    button_decr.connect_clicked(clone!(#[weak] button_print, move |_| {
        button_print.set_number(button_print.number()-1);
    }));

    button_print.bind_property("number", &button_decr, "sensitive")
        .transform_to(|_, number: i32| {
            Some(number > MIN_NUMBER)
        })
        .sync_create()
        .build();
    
    let model = gio::ListStore::new::<IntegerObject>();

    button_print.connect_clicked(clone!(#[weak] model, move |button| {
        let value: IntegerObject = button.number().into();
        model.append(&value)
    }));

    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, list_item| {
        let label = Label::new(None);
        list_item.set_child(Some(&label));
    });

    factory.connect_bind(|_, list_item| {
        let object = list_item
            .item()
            .expect("list item needs to have an item for bind step");
        let integer_object = object
            .downcast_ref::<IntegerObject>()
            .expect("list item's item needs to be an integer object");
        
        let widget = list_item
            .child()
            .expect("list item needs a child for bind step");
        let label = widget.downcast_ref::<Label>()
            .expect("list item's child needs to be a label");

        label.set_label(&integer_object.value().to_string());
    });

    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&list_view)
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(0)
        .propagate_natural_height(true)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button_print);
    gtk_box.append(&button_incr);
    gtk_box.append(&button_decr);
    gtk_box.append(&scrolled_window);

    let window = SavingWindow::builder()
        .application(app)
        .title("My App")
        .child(&gtk_box)
        .build(settings);

    window.present();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
