use gtk::{
    self,
    gio::Settings,
    glib::{self},
    prelude::*,
    Application, ApplicationWindow, ColorDialogButton,
};

const APP_ID: &str = "org.Waytrogen.Waytrogen";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Waytrogen")
        .build();

    window.present();

    let settings = Settings::new(APP_ID);

    let color_dialog_button = ColorDialogButton::builder().build();
    settings.bind("color", &color_dialog_button, "rgba").build();
}
