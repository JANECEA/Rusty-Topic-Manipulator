use crate::settings::{List, SETTINGS_DIR_NAME};
use crate::writer::topic_writer::TopicWriter;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

pub struct LocalTopicFileHandler {
    topics_file_dir: PathBuf,
    topics_file_path: PathBuf,
    topics_file_old_path: PathBuf,
    banner: String,
}

impl TopicWriter for LocalTopicFileHandler {
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

    fn get_banner(&self) -> &str {
        self.banner.as_str()
    }
}

impl LocalTopicFileHandler {
    pub fn new(list: &List, documents_path: &Path) -> Self {
        let topics_file_dir: PathBuf = documents_path.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join(list.path());
        let topics_file_old_path: PathBuf = topics_file_dir.join(format!("{}.old", list.path()));
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
            banner: Self::set_banner(list.banner_path()),
        }
    }

    fn set_banner(banner_path: &str) -> String {
        fs::read_to_string(banner_path).unwrap_or_default()
    }
}
