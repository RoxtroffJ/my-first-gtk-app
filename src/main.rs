use gtk::gio::Settings;
use gtk::glib::clone;
use gtk::{prelude::*, Button};
use gtk::{glib, Application};

mod numbered_button;
use numbered_button::*;

mod window;
use window::*;

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

    button_print.connect_clicked(|button| println!("{}", button.number()));
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

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button_print);
    gtk_box.append(&button_incr);
    gtk_box.append(&button_decr);

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
