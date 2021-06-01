use gtk::prelude::*;

mod gtk4wf;

fn main() {
    if !gtk::init().is_err(){
        let app = gtk::Application::new(Some("in.arunraj.g4wf"), Default::default());
        app.connect_activate(app_activate);
        app.run();
    }else{
        println!("Failed to init GTK!!")
    }
}

fn app_activate(app:&gtk::Application){
    let main_win = gtk::ApplicationWindow::builder()
        .application(app)
        .title("GTK4-rs Widget Factory")
        .resizable(true)
        .default_width(1850).default_height(1000)
        .build();
    main_win.set_child(Some(&gtk4wf::main::build_all(10)));
    main_win.show();
}