use crate::keycode_serde::{keycode_vec_deser, keycode_vec_ser};
use serde::{Serialize, Deserialize};
use std::{fs, io::{Write, Read}, path};
use device_query::Keycode;

const CONFIG_FOLDER: &str = "bongocat";
const CONFIG_FILENAME: &str = "config.toml";
const NEUTRAL_DEFAULT: &str = "neutral.png";
const LEFT_DEFAULT: &str = "left.png";
const RIGHT_DEFAULT: &str = "right.png";
const BOTH_DEFAULT: &str = "both.png";

lazy_static! {
    static ref CONFIG_PATH: path::PathBuf = dirs::config_dir().expect("Couldn't find default config path").join(CONFIG_FOLDER);
}

pub enum Side {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub images: ImageConfig,
    pub input: InputConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputConfig {
    #[serde(serialize_with = "keycode_vec_ser")]
    #[serde(deserialize_with = "keycode_vec_deser")]
    pub keyboard_left: Vec<Keycode>,
    #[serde(serialize_with = "keycode_vec_ser")]
    #[serde(deserialize_with = "keycode_vec_deser")]
    pub keyboard_right: Vec<Keycode>,
    pub mouse_left: Vec<usize>,
    pub mouse_right: Vec<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageConfig {
    pub background_color: (u8, u8, u8, u8),
    pub neutral: String,
    pub left: String,
    pub right: String,
    pub both: String
}

impl Config {
    pub fn create_config() {
        fs::create_dir_all(CONFIG_PATH.clone()).unwrap();
        let vec = toml::to_vec(&Config::default()).unwrap();
        let mut file = fs::File::create(CONFIG_PATH.join(CONFIG_FILENAME)).expect("Couldn't create config file");
        file.write_all(&vec).unwrap();
    }

    pub fn get_config() -> Self {
        if !CONFIG_PATH.join(CONFIG_FILENAME).exists() {
            Self::create_config();
        }
        let mut file = fs::File::open(CONFIG_PATH.join(CONFIG_FILENAME)).expect("Config file couldn't be opened");
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        toml::from_slice(&vec).unwrap()
    }

    pub fn get_path(file: &String) -> path::PathBuf {
        CONFIG_PATH.join(file)
    }

    pub fn side_of_key(&self, key: &Keycode) -> Option<Side> {
        if self.input.keyboard_left.contains(key) { return Some(Side::Left); }
        if self.input.keyboard_right.contains(key) { return Some(Side::Right); }
        return None;
    }

    pub fn side_of_button(&self, button: &usize) -> Option<Side> {
        if self.input.mouse_left.contains(button) { return Some(Side::Left); }
        if self.input.mouse_right.contains(button) { return Some(Side::Right); }
        return None;
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            images: ImageConfig {
                background_color: (0, 255, 0, 255),
                neutral: NEUTRAL_DEFAULT.to_string(),
                left: LEFT_DEFAULT.to_string(),
                right: RIGHT_DEFAULT.to_string(),
                both: BOTH_DEFAULT.to_string()
            },
            input: InputConfig {
                keyboard_left: vec!(Keycode::Z.into(), Keycode::A.into()),
                keyboard_right: vec!(Keycode::C.into(), Keycode::D.into()),
                mouse_right: vec!(2),
                mouse_left: vec!(1)
            }
        }
    }
}
