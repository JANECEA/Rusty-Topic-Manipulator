use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};

pub const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct Settings {
    settings: ParsedSettings,
    path_to_settings_file: PathBuf,
    path_to_settings_dir: PathBuf,
    documents_path: PathBuf,
}

impl Settings {
    fn new(settings: ParsedSettings, documents_path: PathBuf) -> Self {
        let path_to_settings_dir = documents_path.join(SETTINGS_DIR_NAME);
        let path_to_settings_file = path_to_settings_dir.join(SETTINGS_FILE_NAME);
        Self {
            settings,
            documents_path,
            path_to_settings_dir,
            path_to_settings_file,
        }
    }

    pub fn open_in(&self) -> &str {
        &self.settings.open_in
    }

    pub fn lists(&self) -> &[List] {
        &self.settings.lists
    }

    pub fn get_list_by_name(&mut self, name: &str) -> Option<&List> {
        for (index, list) in &mut self.settings.lists.iter_mut().enumerate() {
            if list.name == name {
                return Some(self.get_list_by_index(index));
            }
        }
        None
    }

    pub fn get_list_by_index(&mut self, index: usize) -> &List {
        let list = &mut self.settings.lists[index];
        list.banner_path = self
            .path_to_settings_dir
            .join(&list.banner_path)
            .display()
            .to_string();

        if self.settings.open_last && self.settings.open_in != list.name {
            list.name.clone_into(&mut self.settings.open_in);
        }
        &self.settings.lists[index]
    }

    pub fn path_to_settings_file(&self) -> &PathBuf {
        &self.path_to_settings_file
    }

    pub fn path_to_settings_dir(&self) -> &PathBuf {
        &self.path_to_settings_dir
    }

    pub fn documents_path(&self) -> &PathBuf {
        &self.documents_path
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ParsedSettings {
    open_in: String,
    open_last: bool,
    lists: Vec<List>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    name: String,
    banner_path: String,
    banner_color: BannerColor,
    #[serde(rename = "type")]
    list_type: ListType,
    path: String,
}

impl List {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn banner_path(&self) -> &str {
        &self.banner_path
    }

    pub fn banner_color(&self) -> &BannerColor {
        &self.banner_color
    }

    pub fn list_type(&self) -> &ListType {
        &self.list_type
    }

    pub fn path(&self) -> &str {
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

fn get_documents_dir() -> PathBuf {
    match directories::UserDirs::new() {
        Some(user_dirs) => user_dirs
            .document_dir()
            .map_or_else(|| PathBuf::from("/"), PathBuf::from),
        None => PathBuf::from("/"),
    }
}

pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
    let documents_dir = get_documents_dir();
    let json_file_path = documents_dir
        .join(SETTINGS_DIR_NAME)
        .join(SETTINGS_FILE_NAME);

    Ok(Settings::new(
        serde_json::from_reader(File::open(json_file_path)?)?,
        documents_dir,
    ))
}
