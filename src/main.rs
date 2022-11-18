use fltk::{prelude::*, *};
use fltk_theme::colors::aqua::light::{controlAccentColor, controlColor};
use fltk_theme::widget_schemes::aqua::frames::OS_DEFAULT_BUTTON_UP_BOX;
use fltk_theme::{ThemeType, WidgetTheme};

fn main() {
    let app = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Metro);
    theme.apply();
    let mut window = window::Window::default()
        .with_size(400, 320)
        .with_label("SSH Helper");
    let mut title = frame::Frame::new(100, 40, 200, 30, "SSH Helper");
    title.set_label_size(32);
    let host_input = host_input();
    let mut key_input = key_input();
    let mut key_select_btn = key_select_btn();
    let cloned_key_input = key_input.clone();
    key_select_btn.set_callback(move |_| {
        if let Some(path) = key_file_chooser() {
            key_input.set_value(&path);
        }
    });
    let port_input = port_input();
    let mut connect_btn = connect_btn();
    connect_btn.set_callback(move |_| {
        connect(
            cloned_key_input.value(),
            host_input.clone().value(),
            port_input.clone().value(),
        )
    });
    window.end();
    window.make_resizable(false);
    window.show();

    window.set_callback(move |_| {
        if app::event() == enums::Event::Close {
            app.quit();
        }
    });

    app.run().unwrap();
}

fn host_input() -> input::Input {
    let mut input = input::Input::new(150, 100, 200, 30, None);
    input.set_label("Host ");
    input.set_color(*controlColor);
    input
}

fn key_input() -> input::Input {
    let mut input = input::Input::new(150, 150, 170, 30, None);
    input.set_label("Private Key ");
    input.set_color(*controlColor);
    input
}

fn key_select_btn() -> button::Button {
    let mut btn = button::Button::new(320, 150, 30, 30, None);
    btn.set_label("@fileopen");
    btn.set_color(*controlColor);
    btn.set_selection_color(*controlAccentColor);
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    btn
}

fn key_file_chooser() -> Option<String> {
    let mut chooser = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    chooser.show();
    println!("{:?}", chooser.filenames());
    let path = chooser.filenames()[0].clone().into_os_string();
    Some(path.into_string().unwrap())
}

fn port_input() -> input::Input {
    let mut input = input::Input::new(150, 200, 200, 30, None);
    input.set_label("Ports to Tunnel ");
    input.set_color(*controlColor);
    input
}

fn connect_btn() -> button::Button {
    let mut btn = button::Button::new(120, 250, 170, 30, None);
    btn.set_label("Connect!");
    btn.set_color(*controlColor);
    btn.set_selection_color(*controlAccentColor);
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    btn
}

fn connect(
    cloned_key_input_value: String,
    cloned_host_input_value: String,
    cloned_port_input_value: String,
) {
    let mut command = std::process::Command::new("ssh");
    command.args(&["-N", "-C"]);
    if cloned_key_input_value.chars().count() != 0 {
        command.arg(format!("-i {cloned_key_input_value}"));
    }
    cloned_port_input_value
        .split(",")
        .into_iter()
        .for_each(|port| {
            let trimed_port = port.trim();
            command.arg(format!("-L {trimed_port}:127.0.0.1:{trimed_port}"));
        });
    command.arg(cloned_host_input_value);
    match command.spawn() {
        Ok(_process) => (),
        Err(err) => panic!("{}", err),
    }
}
