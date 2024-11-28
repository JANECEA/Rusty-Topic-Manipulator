use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    open_in: String,
    open_last: bool,
    lists: Vec<List>,
}

impl Settings {
    pub fn get_open_in(&self) -> &str {
        &self.open_in
    }

    pub fn get_open_last(&self) -> bool {
        self.open_last
    }

    pub fn get_lists(&self) -> &[List] {
        &self.lists
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    name: String,
    banner_path: String,
    banner_color: BannerColor,
    #[serde(rename = "type")]
    list_type: ListType,
    path: String,
}

impl List {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_banner_path(&self) -> &str {
        &self.banner_path
    }

    pub fn get_banner_color(&self) -> &BannerColor {
        &self.banner_color
    }

    pub fn get_list_type(&self) -> &ListType {
        &self.list_type
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BannerColor {
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
}

impl BannerColor {
    pub fn as_crossterm_color(&self) -> Color {
        match self {
            BannerColor::Black => Color::Black,
            BannerColor::DarkGrey => Color::DarkGrey,
            BannerColor::Red => Color::Red,
            BannerColor::DarkRed => Color::DarkRed,
            BannerColor::Green => Color::Green,
            BannerColor::DarkGreen => Color::DarkGreen,
            BannerColor::Yellow => Color::Yellow,
            BannerColor::DarkYellow => Color::DarkYellow,
            BannerColor::Blue => Color::Blue,
            BannerColor::DarkBlue => Color::DarkBlue,
            BannerColor::Magenta => Color::Magenta,
            BannerColor::DarkMagenta => Color::DarkMagenta,
            BannerColor::Cyan => Color::Cyan,
            BannerColor::DarkCyan => Color::DarkCyan,
            BannerColor::White => Color::White,
            BannerColor::Grey => Color::Grey,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ListType {
    GoogleDrive,
    Local,
}

pub fn parse_json_file(json_file_path: &str) -> Result<Settings, Box<dyn Error>> {
    Ok(serde_json::from_reader(File::open(json_file_path)?)?)
}
