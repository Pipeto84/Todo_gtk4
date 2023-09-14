mod window;
mod task_object;
mod task_row;

use gtk::{gio,glib,prelude::*,Application};
use window::Window;
const APP_ID:&str="org.gtk_rs.Todo";
fn main()->glib::ExitCode {
    gio::resources_register_include!("todo.gresource")
        .expect("fallo el registro de recursos");
    let app=Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app:&Application) {
    let window=Window::new(app);
    window.present();
}
fn setup_shortcuts(app:&Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}
