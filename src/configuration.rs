use serde::{Serialize, Deserialize};
use std::{fs, io::{Write, Read}, path};

const CONFIG_FOLDER: &str = "bongocat";
const CONFIG_FILENAME: &str = "config.toml";
const NEUTRAL_DEFAULT: &str = "neutral.png";
const LEFT_DEFAULT: &str = "left.png";
const RIGHT_DEFAULT: &str = "right.png";
const BOTH_DEFAULT: &str = "both.png";

lazy_static! {
    static ref CONFIG_PATH: path::PathBuf = dirs::config_dir().expect("Couldn't find default config path").join(CONFIG_FOLDER);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub images: ImageConfig
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
        let mut file = fs::File::create(CONFIG_PATH.join(CONFIG_FILENAME)).expect("Couldn't create config file");
        let vec = toml::to_vec(&Config::default()).unwrap();
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
            }
        }
    }
}
