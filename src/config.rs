use serde::{Serialize, Deserialize};
use std::{fs, path};

const CONFIG_FOLDER: &str = "bongocat/";
const NEUTRAL_DEFAULT: &str = "neutral.png";
const LEFT_DEFAULT: &str = "left.png";
const RIGHT_DEFAULT: &str = "right.png";
const BOTH_DEFAULT: &str = "both.png";

lazy_static! {
    static ref CONFIG_PATH: path::PathBuf = dirs::config_dir().expect("Couldn't find default config path").join(CONFIG_FOLDER);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "get_default_image_config")]
    images: ImageConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageConfig {
    neutral: String,
    left: String,
    right: String,
    both: String
}

impl Config {
    pub fn create_config() {
        if CONFIG_PATH.join("config.toml").exists() {
            return;
        }
        fs::create_dir_all(CONFIG_PATH.clone()).unwrap();

    }
}

fn get_default_image_config() -> ImageConfig {
    ImageConfig {
        neutral: NEUTRAL_DEFAULT.into(),
        left: LEFT_DEFAULT.into(),
        right: RIGHT_DEFAULT.into(),
        both: BOTH_DEFAULT.into()
    }
}
