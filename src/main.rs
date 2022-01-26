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
    config: Arc<Config>,
    _win: Window,
    frames: Frames,
    device_state: DeviceState,
    receiver: app::Receiver<State>,
    sender: app::Sender<State>,
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
            app, device_state, receiver, sender,
            _win: win,
            config: Arc::new(config),
            frames: Frames {
                neutral, left, right, both
            }
        }
    }

    pub fn run(mut self) {
        let _guard = {
            let s = self.sender.clone();
            let config = self.config.clone();
            self.device_state.on_key_down(move |key| {
                if let Some(side) = config.side_of_key(key) {
                    s.send(State::Down(side));
                }
            })
        };
        let _guard = {
            let s = self.sender.clone();
            let config = self.config.clone();
            self.device_state.on_key_up(move |key| {
                if let Some(side) = config.side_of_key(key) {
                    s.send(State::Up(side));
                }
            })
        };
        let _guard = {
            let s = self.sender.clone();
            let config = self.config.clone();
            self.device_state.on_mouse_down(move |button| {
                if let Some(side) = config.side_of_button(button) {
                    s.send(State::Down(side));
                }
            })
        };
        let _guard = {
            let s = self.sender.clone();
            let config = self.config.clone();
            self.device_state.on_mouse_up(move |button| {
                if let Some(side) = config.side_of_button(button) {
                    s.send(State::Up(side));
                }
            })
        };
        let mut state = (0, 0);
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                let previous = state.clone();
                match msg {
                    State::Down(side) => {
                        match side {
                            Side::Left => state.0 += 1,
                            Side::Right => state.1 += 1,
                        }
                    }
                    State::Up(side) => {
                        match side {
                            Side::Left => state.0 -= 1,
                            Side::Right => state.1 -= 1,
                        }
                    }
                }

                match state {
                    (1, 1) => self.frames.both.show(),
                    (1, 0) => self.frames.left.show(),
                    (0, 1) => self.frames.right.show(),
                    (0, 0) => self.frames.neutral.show(),
                    _ => (),
                }
                match (previous, state) {
                    ((a, b), (x, y)) if a > 0 && b > 0 && (x == 0 || y == 0) => self.frames.both.hide(),
                    ((a, 0), (x, y)) if a > 0 && (x == 0 || y > 0) => self.frames.left.hide(),
                    ((0, b), (x, y)) if b > 0 && (x > 0 || y == 0) => self.frames.right.hide(),
                    ((0, 0), (x, y)) if x > 0 || y > 0 => self.frames.neutral.hide(),
                    _ => (),
                }
            }
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
