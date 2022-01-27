use serde::{Serialize, Deserialize, Serializer, Deserializer};
use device_query::Keycode;

#[derive(Deserialize, Serialize)]
pub enum KeycodeDef {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Meta,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    CapsLock,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadSubtract,
    NumpadAdd,
    NumpadDivide,
    NumpadMultiply,
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    BackSlash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,
}

fn to_external(key: &KeycodeDef) -> Keycode {
    match key {
        KeycodeDef::Key0 => Keycode::Key0,
        KeycodeDef::Key1 => Keycode::Key1,
        KeycodeDef::Key2 => Keycode::Key2,
        KeycodeDef::Key3 => Keycode::Key3,
        KeycodeDef::Key4 => Keycode::Key4,
        KeycodeDef::Key5 => Keycode::Key5,
        KeycodeDef::Key6 => Keycode::Key6,
        KeycodeDef::Key7 => Keycode::Key7,
        KeycodeDef::Key8 => Keycode::Key8,
        KeycodeDef::Key9 => Keycode::Key9,
        KeycodeDef::A => Keycode::A,
        KeycodeDef::B => Keycode::B,
        KeycodeDef::C => Keycode::C,
        KeycodeDef::D => Keycode::D,
        KeycodeDef::E => Keycode::E,
        KeycodeDef::F => Keycode::F,
        KeycodeDef::G => Keycode::G,
        KeycodeDef::H => Keycode::H,
        KeycodeDef::I => Keycode::I,
        KeycodeDef::J => Keycode::J,
        KeycodeDef::K => Keycode::K,
        KeycodeDef::L => Keycode::L,
        KeycodeDef::M => Keycode::M,
        KeycodeDef::N => Keycode::N,
        KeycodeDef::O => Keycode::O,
        KeycodeDef::P => Keycode::P,
        KeycodeDef::Q => Keycode::Q,
        KeycodeDef::R => Keycode::R,
        KeycodeDef::S => Keycode::S,
        KeycodeDef::T => Keycode::T,
        KeycodeDef::U => Keycode::U,
        KeycodeDef::V => Keycode::V,
        KeycodeDef::W => Keycode::W,
        KeycodeDef::X => Keycode::X,
        KeycodeDef::Y => Keycode::Y,
        KeycodeDef::Z => Keycode::Z,
        KeycodeDef::F1 => Keycode::F1,
        KeycodeDef::F2 => Keycode::F2,
        KeycodeDef::F3 => Keycode::F3,
        KeycodeDef::F4 => Keycode::F4,
        KeycodeDef::F5 => Keycode::F5,
        KeycodeDef::F6 => Keycode::F6,
        KeycodeDef::F7 => Keycode::F7,
        KeycodeDef::F8 => Keycode::F8,
        KeycodeDef::F9 => Keycode::F9,
        KeycodeDef::F10 => Keycode::F10,
        KeycodeDef::F11 => Keycode::F11,
        KeycodeDef::F12 => Keycode::F12,
        KeycodeDef::Escape => Keycode::Escape,
        KeycodeDef::Space => Keycode::Space,
        KeycodeDef::LControl => Keycode::LControl,
        KeycodeDef::RControl => Keycode::RControl,
        KeycodeDef::LShift => Keycode::LShift,
        KeycodeDef::RShift => Keycode::RShift,
        KeycodeDef::LAlt => Keycode::LAlt,
        KeycodeDef::RAlt => Keycode::RAlt,
        KeycodeDef::Meta => Keycode::Meta,
        KeycodeDef::Enter => Keycode::Enter,
        KeycodeDef::Up => Keycode::Up,
        KeycodeDef::Down => Keycode::Down,
        KeycodeDef::Left => Keycode::Left,
        KeycodeDef::Right => Keycode::Right,
        KeycodeDef::Backspace => Keycode::Backspace,
        KeycodeDef::CapsLock => Keycode::CapsLock,
        KeycodeDef::Tab => Keycode::Tab,
        KeycodeDef::Home => Keycode::Home,
        KeycodeDef::End => Keycode::End,
        KeycodeDef::PageUp => Keycode::PageUp,
        KeycodeDef::PageDown => Keycode::PageDown,
        KeycodeDef::Insert => Keycode::Insert,
        KeycodeDef::Delete => Keycode::Delete,
        KeycodeDef::Numpad0 => Keycode::Numpad0,
        KeycodeDef::Numpad1 => Keycode::Numpad1,
        KeycodeDef::Numpad2 => Keycode::Numpad2,
        KeycodeDef::Numpad3 => Keycode::Numpad3,
        KeycodeDef::Numpad4 => Keycode::Numpad4,
        KeycodeDef::Numpad5 => Keycode::Numpad5,
        KeycodeDef::Numpad6 => Keycode::Numpad6,
        KeycodeDef::Numpad7 => Keycode::Numpad7,
        KeycodeDef::Numpad8 => Keycode::Numpad8,
        KeycodeDef::Numpad9 => Keycode::Numpad9,
        KeycodeDef::NumpadSubtract => Keycode::NumpadSubtract,
        KeycodeDef::NumpadAdd => Keycode::NumpadAdd,
        KeycodeDef::NumpadDivide => Keycode::NumpadDivide,
        KeycodeDef::NumpadMultiply => Keycode::NumpadMultiply,
        KeycodeDef::Grave => Keycode::Grave,
        KeycodeDef::Minus => Keycode::Minus,
        KeycodeDef::Equal => Keycode::Equal,
        KeycodeDef::LeftBracket => Keycode::LeftBracket,
        KeycodeDef::RightBracket => Keycode::RightBracket,
        KeycodeDef::BackSlash => Keycode::BackSlash,
        KeycodeDef::Semicolon => Keycode::Semicolon,
        KeycodeDef::Apostrophe => Keycode::Apostrophe,
        KeycodeDef::Comma => Keycode::Comma,
        KeycodeDef::Dot => Keycode::Dot,
        KeycodeDef::Slash => Keycode::Slash,
    }
}

fn to_local(key: &Keycode) -> KeycodeDef {
    match key {
        Keycode::Key0 => KeycodeDef::Key0,
        Keycode::Key1 => KeycodeDef::Key1,
        Keycode::Key2 => KeycodeDef::Key2,
        Keycode::Key3 => KeycodeDef::Key3,
        Keycode::Key4 => KeycodeDef::Key4,
        Keycode::Key5 => KeycodeDef::Key5,
        Keycode::Key6 => KeycodeDef::Key6,
        Keycode::Key7 => KeycodeDef::Key7,
        Keycode::Key8 => KeycodeDef::Key8,
        Keycode::Key9 => KeycodeDef::Key9,
        Keycode::A => KeycodeDef::A,
        Keycode::B => KeycodeDef::B,
        Keycode::C => KeycodeDef::C,
        Keycode::D => KeycodeDef::D,
        Keycode::E => KeycodeDef::E,
        Keycode::F => KeycodeDef::F,
        Keycode::G => KeycodeDef::G,
        Keycode::H => KeycodeDef::H,
        Keycode::I => KeycodeDef::I,
        Keycode::J => KeycodeDef::J,
        Keycode::K => KeycodeDef::K,
        Keycode::L => KeycodeDef::L,
        Keycode::M => KeycodeDef::M,
        Keycode::N => KeycodeDef::N,
        Keycode::O => KeycodeDef::O,
        Keycode::P => KeycodeDef::P,
        Keycode::Q => KeycodeDef::Q,
        Keycode::R => KeycodeDef::R,
        Keycode::S => KeycodeDef::S,
        Keycode::T => KeycodeDef::T,
        Keycode::U => KeycodeDef::U,
        Keycode::V => KeycodeDef::V,
        Keycode::W => KeycodeDef::W,
        Keycode::X => KeycodeDef::X,
        Keycode::Y => KeycodeDef::Y,
        Keycode::Z => KeycodeDef::Z,
        Keycode::F1 => KeycodeDef::F1,
        Keycode::F2 => KeycodeDef::F2,
        Keycode::F3 => KeycodeDef::F3,
        Keycode::F4 => KeycodeDef::F4,
        Keycode::F5 => KeycodeDef::F5,
        Keycode::F6 => KeycodeDef::F6,
        Keycode::F7 => KeycodeDef::F7,
        Keycode::F8 => KeycodeDef::F8,
        Keycode::F9 => KeycodeDef::F9,
        Keycode::F10 => KeycodeDef::F10,
        Keycode::F11 => KeycodeDef::F11,
        Keycode::F12 => KeycodeDef::F12,
        Keycode::Escape => KeycodeDef::Escape,
        Keycode::Space => KeycodeDef::Space,
        Keycode::LControl => KeycodeDef::LControl,
        Keycode::RControl => KeycodeDef::RControl,
        Keycode::LShift => KeycodeDef::LShift,
        Keycode::RShift => KeycodeDef::RShift,
        Keycode::LAlt => KeycodeDef::LAlt,
        Keycode::RAlt => KeycodeDef::RAlt,
        Keycode::Meta => KeycodeDef::Meta,
        Keycode::Enter => KeycodeDef::Enter,
        Keycode::Up => KeycodeDef::Up,
        Keycode::Down => KeycodeDef::Down,
        Keycode::Left => KeycodeDef::Left,
        Keycode::Right => KeycodeDef::Right,
        Keycode::Backspace => KeycodeDef::Backspace,
        Keycode::CapsLock => KeycodeDef::CapsLock,
        Keycode::Tab => KeycodeDef::Tab,
        Keycode::Home => KeycodeDef::Home,
        Keycode::End => KeycodeDef::End,
        Keycode::PageUp => KeycodeDef::PageUp,
        Keycode::PageDown => KeycodeDef::PageDown,
        Keycode::Insert => KeycodeDef::Insert,
        Keycode::Delete => KeycodeDef::Delete,
        Keycode::Numpad0 => KeycodeDef::Numpad0,
        Keycode::Numpad1 => KeycodeDef::Numpad1,
        Keycode::Numpad2 => KeycodeDef::Numpad2,
        Keycode::Numpad3 => KeycodeDef::Numpad3,
        Keycode::Numpad4 => KeycodeDef::Numpad4,
        Keycode::Numpad5 => KeycodeDef::Numpad5,
        Keycode::Numpad6 => KeycodeDef::Numpad6,
        Keycode::Numpad7 => KeycodeDef::Numpad7,
        Keycode::Numpad8 => KeycodeDef::Numpad8,
        Keycode::Numpad9 => KeycodeDef::Numpad9,
        Keycode::NumpadSubtract => KeycodeDef::NumpadSubtract,
        Keycode::NumpadAdd => KeycodeDef::NumpadAdd,
        Keycode::NumpadDivide => KeycodeDef::NumpadDivide,
        Keycode::NumpadMultiply => KeycodeDef::NumpadMultiply,
        Keycode::Grave => KeycodeDef::Grave,
        Keycode::Minus => KeycodeDef::Minus,
        Keycode::Equal => KeycodeDef::Equal,
        Keycode::LeftBracket => KeycodeDef::LeftBracket,
        Keycode::RightBracket => KeycodeDef::RightBracket,
        Keycode::BackSlash => KeycodeDef::BackSlash,
        Keycode::Semicolon => KeycodeDef::Semicolon,
        Keycode::Apostrophe => KeycodeDef::Apostrophe,
        Keycode::Comma => KeycodeDef::Comma,
        Keycode::Dot => KeycodeDef::Dot,
        Keycode::Slash => KeycodeDef::Slash,
    }
}

pub fn keycode_vec_ser<S: Serializer>(
    vec: &[Keycode],
    serializer: S
) -> Result<S::Ok, S::Error> {
    let vec2: Vec<KeycodeDef> = vec.iter().map(to_local).collect();
    vec2.serialize(serializer)
}

pub fn keycode_vec_deser<'de, D: Deserializer<'de>>(
    deserializer: D
) -> Result<Vec<Keycode>, D::Error> {
    let vec: Vec<KeycodeDef> = Deserialize::deserialize(deserializer)?;
    Ok(vec.iter().map(to_external).collect())
}
