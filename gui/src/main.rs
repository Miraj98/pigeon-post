use gtk::{ prelude::*, glib, Application, ApplicationWindow, Button };

const APP_ID: &str = "org.miraj.PigeonPost";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 24);

    let menubutton = gtk::MenuButton::builder()
        .label("GET")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    container.append(&menubutton);

    let text = gtk::Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .placeholder_text("Enter a URL...")
        .build();
    text.set_width_request(700);

    container.append(&text);

    text.connect_activate(|text| {
        let a = text.buffer();
        println!("[TEXTFIELD] {}", a.text());
    });

    let button = Button::builder().label("Send")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    container.append(&button);

    button.connect_clicked(|button| {
        button.set_label("Hello world");
    });


    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pigeon Post")
        .child(&container)
        .build();
    window.present();
}
