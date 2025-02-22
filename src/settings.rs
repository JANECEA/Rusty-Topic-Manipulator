use crate::controllers::commands::CommandResult;
use anyhow::Result;
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};

pub const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug)]
pub struct Settings {
    parsed_settings: ParsedSettings,
    previous_open_in: String,
    path_to_settings_file: PathBuf,
    path_to_settings_dir: PathBuf,
    documents_path: PathBuf,
    default_settings_used: bool,
}

impl Settings {
    fn new(
        parsed_settings: ParsedSettings,
        documents_path: PathBuf,
        default_settings_used: bool,
    ) -> Self {
        let previous_open_in = parsed_settings.open_in.clone();
        let path_to_settings_dir = documents_path.join(SETTINGS_DIR_NAME);
        let path_to_settings_file = path_to_settings_dir.join(SETTINGS_FILE_NAME);
        Self {
            parsed_settings,
            previous_open_in,
            path_to_settings_file,
            path_to_settings_dir,
            documents_path,
            default_settings_used,
        }
    }

    pub fn get_settings() -> Result<Settings> {
        let documents_dir = get_documents_dir();
        let settings_file_path = documents_dir
            .join(SETTINGS_DIR_NAME)
            .join(SETTINGS_FILE_NAME);

        let default_settings_used = !settings_file_path.is_file();
        Ok(Settings::new(
            if default_settings_used {
                ParsedSettings::default(&settings_file_path)
            } else {
                serde_json::from_reader(File::open(settings_file_path)?)?
            },
            documents_dir,
            default_settings_used,
        ))
    }

    pub fn save_settings(&self) -> Result<()> {
        if !self.settings_changed() {
            return Ok(());
        }
        self.parsed_settings
            .save_settings(&self.path_to_settings_file)
    }

    fn settings_changed(&self) -> bool {
        self.default_settings_used
            || self.parsed_settings.open_last
                && self.previous_open_in != self.parsed_settings.open_in
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
        if let ListType::Local = list.list_type {
            list.banner_path = self
                .path_to_settings_dir
                .join(&list.banner_path)
                .display()
                .to_string();
        }
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

impl ParsedSettings {
    fn default(path_to_settings_file: &PathBuf) -> Self {
        let banner_file_name = "NewListBanner.txt";
        let default_banner_path = path_to_settings_file
            .parent()
            .unwrap()
            .join(banner_file_name);

        let settings = ParsedSettings {
            open_in: "New List".to_string(),
            open_last: true,
            lists: vec![List {
                name: "New List".to_string(),
                banner_path: banner_file_name.to_string(),
                banner_color: BannerColor::White,
                list_type: ListType::Local,
                path: "newList.txt".to_string(),
                access_token: String::new(),
            }],
        };
        settings
            .save_settings(path_to_settings_file)
            .expect("Could not save settings.");
        Self::save_default_banner(&default_banner_path).expect("Could not save default banner.");
        settings
    }

    fn save_default_banner(path: &PathBuf) -> std::io::Result<()> {
        let banner = r"
/==============================================\
||                                            ||
||   _   _                 _     _     _      ||
||  | \ | | _____      __ | |   (_)___| |_    ||
||  |  \| |/ _ \ \ /\ / / | |   | / __| __|   ||
||  | |\  |  __/\ V  V /  | |___| \__ \ |_    ||
||  |_| \_|\___| \_/\_/   |_____|_|___/\__|   ||
||                                            ||
\==============================================/
";
        fs::write(path, banner)
    }

    fn save_settings(&self, path_to_settings_file: &PathBuf) -> Result<()> {
        let file = File::create(path_to_settings_file)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
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
    access_token: String,
}

impl Clone for List {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            banner_path: self.banner_path.clone(),
            banner_color: self.banner_color.clone(),
            list_type: self.list_type.clone(),
            path: self.path.clone(),
            access_token: self.access_token.clone(),
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

    pub fn access_token(&self) -> &str {
        &self.access_token
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
    GithubGist,
}

fn get_documents_dir() -> PathBuf {
    match directories::UserDirs::new() {
        Some(user_dirs) => user_dirs
            .document_dir()
            .map_or_else(|| PathBuf::from("/"), PathBuf::from),
        None => PathBuf::from("/"),
    }
}
