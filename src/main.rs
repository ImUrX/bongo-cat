#[macro_use]
extern crate lazy_static;

mod configuration;
use configuration::{Config};
use fltk::{prelude::*, *};

fn main() {
    let config = Config::get_config();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut window = window::Window::new(100, 100, 800, 800, "Bongo Cat");
    window.set_color(enums::Color::from_rgba_tuple(config.images.background_color))
}
