use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};

use crate::commands::CommandResult;

pub const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug)]
pub struct Settings {
    parsed_settings: ParsedSettings,
    previous_open_in: String,
    path_to_settings_file: PathBuf,
    path_to_settings_dir: PathBuf,
    documents_path: PathBuf,
}

impl Settings {
    fn new(parsed_settings: ParsedSettings, documents_path: PathBuf) -> Self {
        let previous_open_in = parsed_settings.open_in.to_owned();
        let path_to_settings_dir = documents_path.join(SETTINGS_DIR_NAME);
        let path_to_settings_file = path_to_settings_dir.join(SETTINGS_FILE_NAME);
        Self {
            parsed_settings,
            previous_open_in,
            path_to_settings_file,
            path_to_settings_dir,
            documents_path,
        }
    }

    pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
        let documents_dir = get_documents_dir();
        let json_file_path = documents_dir
            .join(SETTINGS_DIR_NAME)
            .join(SETTINGS_FILE_NAME);

        Ok(Settings::new(
            if json_file_path.is_file() {
                serde_json::from_reader(File::open(json_file_path)?)?
            } else {
                get_default_settings()
            },
            documents_dir,
        ))
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn Error>> {
        if !self.settings_changed() {
            return Ok(());
        }
        let file = File::create(&self.path_to_settings_file)?;
        serde_json::to_writer_pretty(file, &self.parsed_settings)?;
        Ok(())
    }

    fn settings_changed(&self) -> bool {
        self.parsed_settings.open_last && self.previous_open_in != self.parsed_settings.open_in
    }

    pub fn open_in(&self) -> &str {
        &self.parsed_settings.open_in
    }

    pub fn lists(&self) -> &[List] {
        &self.parsed_settings.lists
    }

    pub fn get_list(&mut self, query: &str) -> Option<List> {
        if let Ok(index) = str::parse::<usize>(query) {
            if index <= self.parsed_settings.lists.len() {
                Some(self.get_list_by_index(index - 1))
            } else {
                None
            }
        } else {
            for (index, list) in &mut self.parsed_settings.lists.iter_mut().enumerate() {
                if list.name == query {
                    return Some(self.get_list_by_index(index));
                }
            }
            None
        }
    }

    pub fn get_list_by_index(&mut self, index: usize) -> List {
        let mut list = self.parsed_settings.lists[index].clone();
        list.banner_path = self
            .path_to_settings_dir
            .join(&list.banner_path)
            .display()
            .to_string();

        list
    }

    pub fn set_open_in_list(&mut self, list: &List) {
        self.parsed_settings.open_in = list.name().to_string();
    }

    pub fn set_open_in(&mut self, query: &str) -> CommandResult {
        if let Some(list) = self.get_list(query) {
            self.parsed_settings.open_in = list.name().to_string();
            CommandResult::Success
        } else {
            CommandResult::Fail(format!("Couldn't find: \"{query}\" in lists."))
        }
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

impl Clone for List {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            banner_path: self.banner_path.clone(),
            banner_color: self.banner_color.clone(),
            list_type: self.list_type.clone(),
            path: self.path.clone(),
        }
    }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ListType {
    Local,
    Network,
}

fn get_documents_dir() -> PathBuf {
    match directories::UserDirs::new() {
        Some(user_dirs) => user_dirs
            .document_dir()
            .map_or_else(|| PathBuf::from("/"), PathBuf::from),
        None => PathBuf::from("/"),
    }
}

fn get_default_settings() -> ParsedSettings {
    ParsedSettings {
        open_in: "New List".to_string(),
        open_last: true,
        lists: vec![List {
            name: "New List".to_string(),
            banner_path: "NewListBanner.txt".to_string(),
            banner_color: BannerColor::White,
            list_type: ListType::Local,
            path: "newList.txt".to_string(),
        }],
    }
}
