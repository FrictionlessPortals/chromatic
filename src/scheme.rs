//! The main color scheme template.
//!
//! Uses ``askama`` to generate a vim template from the given TOML file.

use askama::Template;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, convert::TryFrom, error::Error, fs::File, io::Read, path::PathBuf,
};

/// Main Color Scheme Template
#[derive(Template)]
#[template(path = "colorscheme")]
pub struct ChromaticTemplate {
    /// Information Section
    pub information: Information,

    /// Highlights Section
    pub highlights: HashMap<String, Highlight>,
}

impl TryFrom<ChromaticConfig> for ChromaticTemplate {
    type Error = Box<Error>;

    fn try_from(config: ChromaticConfig) -> Result<Self, Self::Error> {
        // Match the colors into highlights
        let mut matched_highlights = HashMap::new();
        for (n, mut h) in config.highlights {
            println!(
                "BG: {:?}, FG: {:?}, ST: {:?}",
                h.background_color, h.foreground_color, h.style_color
            );

            // Match the background color
            match h.background_color {
                Some(x) => {
                    let bg_color = match config.colors.get(&x) {
                        Some(x) => x.0.to_owned(),
                        None => String::from("none"),
                    };
                    println!("BG: Matched with {}", bg_color);
                    h.background_color = Some(bg_color);
                }
                None => println!("BG: Matched None"),
            }

            // Match the foreground color
            match h.foreground_color {
                Some(x) => {
                    let fg_color = match config.colors.get(&x) {
                        Some(x) => x.0.to_owned(),
                        None => String::from("none"),
                    };
                    println!("FG: Matched with {}", fg_color);
                    h.foreground_color = Some(fg_color);
                }
                None => println!("FG: Matched None"),
            }

            // Match the style color
            match h.style_color {
                Some(x) => {
                    let st_color = match config.colors.get(&x) {
                        Some(x) => x.0.to_owned(),
                        None => String::from("none"),
                    };
                    println!("ST: Matched with {}", st_color);
                    h.style_color = Some(st_color);
                }
                None => println!("ST: Matched None"),
            }

            matched_highlights.insert(n, h);
        }

        Ok(Self {
            information: config.information,
            highlights: matched_highlights,
        })
    }
}

/// Main Color Scheme Config
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChromaticConfig {
    /// Information Section
    pub information: Information,

    /// Colors Section
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub colors: HashMap<String, Color>,

    /// Highlights Section
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub highlights: HashMap<String, Highlight>,
}

/// Generate a config from a ``PathBuf``.
impl TryFrom<PathBuf> for ChromaticConfig {
    type Error = Box<Error>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(path.as_path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents)?)
    }
}

/// Information Structure for the configuration file.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Information {
    pub name: String,
    pub author: String,
    pub background: String,
    pub description: Option<String>,
    pub license: Option<String>,
}

/// Color Structure for the configuration file.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Color(pub String, pub String);

/// Highlight Structure for the configuration file.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Highlight {
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
    pub style: Option<String>,
    pub style_color: Option<String>,
}
