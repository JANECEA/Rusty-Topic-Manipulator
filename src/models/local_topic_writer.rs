use crate::{
    models::TopicWriter,
    settings::{BannerColor, List, SETTINGS_DIR_NAME},
};
use std::{
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

pub struct LocalTopicWriter {
    topics_file_dir: PathBuf,
    topics_file_path: PathBuf,
    topics_file_old_path: PathBuf,
    banner: String,
    banner_color: BannerColor,
}

impl TopicWriter for LocalTopicWriter {
    fn write(&self, list: &[String]) -> anyhow::Result<()> {
        let mut file: fs::File = fs::File::create(&self.topics_file_path)?;
        for line in list {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    fn try_write(&self, list: &[String]) {
        _ = self.write(list);
    }

    fn close(&self) -> anyhow::Result<()> {
        fs::copy(&self.topics_file_path, &self.topics_file_old_path)?;
        Ok(())
    }

    fn read_list(&mut self) -> anyhow::Result<Vec<String>> {
        self.check_source_exist();
        let file = fs::File::open(&self.topics_file_path)?;
        let reader = io::BufReader::new(file);

        let mut vec = Vec::new();
        for line in reader.lines() {
            vec.push(line?);
        }
        Ok(vec)
    }

    fn get_banner(&self) -> &str {
        self.banner.as_str()
    }

    fn get_banner_color(&self) -> &BannerColor {
        &self.banner_color
    }
}

impl LocalTopicWriter {
    pub fn new(list: &List, documents_path: &Path) -> Self {
        let topics_file_dir: PathBuf = documents_path.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join(list.path());
        let topics_file_old_path: PathBuf = topics_file_dir.join(format!("{}.old", list.path()));
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
            banner: Self::set_banner(list.banner_path()),
            banner_color: list.banner_color().clone(),
        }
    }

    fn set_banner(banner_path: &str) -> String {
        fs::read_to_string(banner_path).unwrap_or_default()
    }

    fn check_source_exist(&self) {
        if !&self.topics_file_dir.exists() {
            fs::create_dir_all(&self.topics_file_dir).expect("Failed to create directory");
        }
        if !&self.topics_file_path.exists() {
            fs::File::create(&self.topics_file_path).expect("Failed to create file");
        }
    }
}
