use crate::topic_writer::TopicWriter;
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
    topics_file_old_path: PathBuf,
}

impl TopicWriter for FileHandler {
    fn write(&self, list: &[String]) -> io::Result<()> {
        let mut file: fs::File = fs::File::create(&self.topics_file_path)?;
        for line in list {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    fn try_write(&self, list: &[String]) {
        _ = self.write(list);
    }

    fn overwrite_old(&self) -> io::Result<()> {
        fs::copy(&self.topics_file_path, &self.topics_file_old_path)?;
        Ok(())
    }

    fn check_source_exist(&self) {
        if !&self.topics_file_dir.exists() {
            fs::create_dir_all(&self.topics_file_dir).expect("Failed to create directory");
        }
        if !&self.topics_file_path.exists() {
            fs::File::create(&self.topics_file_path).expect("Failed to create file");
        }
    }

    fn read_list(&self) -> io::Result<Vec<String>> {
        self.check_source_exist();

        io::BufReader::new(fs::File::open(&self.topics_file_path)?)
            .lines()
            .collect()
    }
}

impl FileHandler {
    pub fn new(topics_file_name: &str, topics_file_old_name: &str) -> Self {
        let documents_dir: PathBuf = FileHandler::init_documents_dir();
        let topics_file_dir: PathBuf = documents_dir.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join(topics_file_name);
        let topics_file_old_path: PathBuf = topics_file_dir.join(topics_file_old_name);
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
        }
    }

    fn init_documents_dir() -> PathBuf {
        if let Some(user_dirs) = directories::UserDirs::new() {
            user_dirs
                .document_dir()
                .map_or_else(|| PathBuf::from("/"), PathBuf::from)
        } else {
            PathBuf::from("/")
        }
    }
}
