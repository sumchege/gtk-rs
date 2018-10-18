
extern crate gtk;
extern crate gio;
extern crate chrono;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;
use chrono::Local;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

fn current_time() -> String
{
    return format!("{}", Local::now().format("%Y-%m-%d  %H: %M: %S"));
}


fn build_ui(application: &gtk::Application) 
{
    let glade_src = include_str!("builder_basics.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Could not load file");


    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
    let dialog: MessageDialog = builder.get_object("messagedialog1").expect("Couldn't get messagedialog1");

    window.set_application(application);

    // window.set_title("First GTK+ Clock");
    // window.set_border_width(10);
    // window.set_position(gtk::WindowPosition::Center);
    // window.set_default_size(260, 40);



    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    bigbutton.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
    });

    // let time = current_time();
    // let label = gtk::Label::new(None);
    // label.set_text(&time);

    // window.add(&label);
    window.show_all();

    // let tick = move || {
    //     let time = current_time();
    //     label.set_text(&time);
    //     // we could return gtk::Continue(false) to stop our clock after this tick
    //     gtk::Continue(true)
    //    };
    //    // executes the closure once every second
    //    gtk::timeout_add_seconds(1, tick);
}

fn main() {
    let application = gtk::Application::new("com.sandtech.basic", gio::ApplicationFlags::empty()).expect("Failed to init");

    application.connect_startup( |app| 
    {
        build_ui(app);
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}
