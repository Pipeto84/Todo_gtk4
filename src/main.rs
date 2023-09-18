mod window;
mod task_object;
mod utils;
use gtk::{gio,glib,prelude::*,gdk,CssProvider};
use gdk::Display;
use window::Window;
const APP_ID:&str="org.gtk_rs.Todo";
fn main()->glib::ExitCode {
    gio::resources_register_include!("todo.gresource")
        .expect("fallo el registro de recursos");
    let app=adw::Application::builder()
        .application_id(APP_ID)
        .build();
    // app.connect_startup(setup_shortcuts);
    app.connect_startup(|app|{
        setup_shortcuts(app);
        load_css()
    });
    app.connect_activate(build_ui);
    app.run()
}
fn load_css() {
    let provider=CssProvider::new();
    provider.load_from_resource("org/gtk_rs/Todo/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("no pudo conectar el display"), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
fn build_ui(app:&adw::Application) {
    let window=Window::new(app);
    window.present();
}
fn setup_shortcuts(app:&adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}
