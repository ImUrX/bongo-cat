#[macro_use]
extern crate lazy_static;

mod keycode_serde;
mod configuration;
use configuration::{Config, Side};
use std::{path, sync::Arc};
use fltk::{prelude::*, enums::Color, app, window::Window, frame::Frame, image::SharedImage};
use device_query::{DeviceEvents, DeviceState};

enum State {
    Down(Side),
    Up(Side),
}

struct BongoCat {
    app: app::App,
    config: Config,
    win: Window,
    frames: Frames,
    device_state: DeviceState,
    receiver: app::Receiver<State>,
    sender: Arc<app::Sender<State>>,
}

struct Frames {
    neutral: Frame,
    left: Frame,
    right: Frame,
    both: Frame,
}

impl BongoCat {
    pub fn new() -> Self {
        let config = Config::get_config();
        let device_state = DeviceState::new();
        let (sender, receiver) = app::channel();
        let app = app::App::default().with_scheme(app::Scheme::Gleam);
        let mut win = Window::new(100, 100, 600, 600, "Bongo Cat");
    
        win.set_color(Color::from_rgba_tuple(config.images.background_color));
        let neutral = create_frame(&Config::get_path(&config.images.neutral));
        let mut left = create_frame(&Config::get_path(&config.images.left));
        left.hide();
        let mut right = create_frame(&Config::get_path(&config.images.right));
        right.hide();
        let mut both = create_frame(&Config::get_path(&config.images.both));
        both.hide();

        win.make_resizable(true);
        win.end();
        win.show();
        Self {
            app, win, device_state, config, receiver,
            sender: Arc::new(sender),
            frames: Frames {
                neutral, left, right, both
            }
        }
    }

    pub fn run(mut self) {
        let _guard = self.device_state.on_key_down(|key| {
            if let Some(side) = self.config.side_of_key(key) {
                self.sender.send(State::Down(side));
            }
        });
        let _guard = self.device_state.on_key_up(|key| {
            if let Some(side) = self.config.side_of_key(key) {
                self.sender.send(State::Up(side));
            }
        });
        let _guard = self.device_state.on_mouse_down(|button| {
            if let Some(side) = self.config.side_of_button(button) {
                self.sender.send(State::Down(side));
            }
        });
        let _guard = self.device_state.on_mouse_up(|button| {
            if let Some(side) = self.config.side_of_button(button) {
                self.sender.send(State::Up(side));
            }
        });
        while self.app.wait() {

        }
    }
}

fn create_frame(file: &path::Path) -> Frame {
    let mut frame = Frame::default().size_of_parent().center_of_parent();
    let image = SharedImage::load(file).unwrap();
    frame.set_image(Some(image));
    frame
}

fn main() {
    let app = BongoCat::new();
    app.run();
}
