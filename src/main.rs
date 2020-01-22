// Tablam_rs 2020
// errz99 20200122

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

extern crate gio;
extern crate gdk;
extern crate gtk;

use gtk::*;
use gio::prelude::*;

fn main() {

    let application = gtk::Application::new(Some("com.errz99.tablamrs"),
				gio::ApplicationFlags::empty())
                .expect("GTK INIT FAILED");

    application.connect_startup(move |app| {

        mainwin(&app);

    });

    application.connect_activate(|_| {});

    application.run(&Vec::new());

}

pub fn mainwin(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Tablam Test");
    window.set_default_size(600, 400);
    window.set_wmclass("stum", "Stum");
    window.set_position(gtk::WindowPosition::Center);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

	let vbox = gtk::Box::new(Orientation::Vertical, 0);
	vbox.set_can_focus(false);
	vbox.set_border_width(5);
	let main_box = gtk::Box::new(Orientation::Vertical, 0);
	main_box.set_can_focus(false);

	window.add(&vbox);
    window.show_all();
}
