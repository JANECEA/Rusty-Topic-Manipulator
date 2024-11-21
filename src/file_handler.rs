use crate::topic_handler::TopicHandler;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};

const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct FileHandler {
    topics_file_dir: PathBuf,
    topics_file_path: PathBuf,
    settings_file_path: PathBuf,
    topics_file_old_path: PathBuf,
}

impl FileHandler {
    pub fn new() -> Self {
        let documents_dir: PathBuf = FileHandler::init_documents_dir();
        let topics_file_dir: PathBuf = documents_dir.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join("topics.happypus");
        let topics_file_old_path: PathBuf = topics_file_dir.join("topics.happypus.old");
        let settings_file_path: PathBuf = topics_file_dir.join(SETTINGS_FILE_NAME);
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
            settings_file_path,
        }
    }

    pub fn write(&self, topics: &TopicHandler) -> io::Result<()> {
        let mut file: fs::File = fs::File::create(&self.topics_file_path)?;
        for line in topics.get_topics() {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn try_write(&self, topics: &TopicHandler) {
        if let Ok(mut file) = fs::File::create(&self.topics_file_path) {
            for line in topics.get_topics() {
                _ = writeln!(file, "{}", line);
            }
        }
    }

    pub fn overwrite_old(&self) -> io::Result<()> {
        fs::copy(&self.topics_file_path, &self.topics_file_old_path)?;
        Ok(())
    }

    pub fn check_files_exist(&self) {
        if !&self.topics_file_dir.exists() {
            fs::create_dir_all(&self.topics_file_dir).expect("Failed to create directory");
        }
        if !&self.topics_file_path.exists() {
            fs::File::create(&self.topics_file_path).expect("Failed to create file");
        }
    }

    pub fn load_settings(&self) -> String {
        fs::read_to_string(&self.settings_file_path).unwrap()
    }

    pub fn read_list(&self) -> io::Result<Vec<String>> {
        self.check_files_exist();

        io::BufReader::new(fs::File::open(&self.topics_file_path)?)
            .lines()
            .collect()
    }

    pub fn init_documents_dir() -> PathBuf {
        if let Some(user_dirs) = directories::UserDirs::new() {
            user_dirs
                .document_dir()
                .map_or_else(|| PathBuf::from("/"), PathBuf::from)
        } else {
            PathBuf::from("/")
        }
    }
}
