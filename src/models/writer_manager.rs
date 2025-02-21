use crate::{models::TopicWriter, settings::BannerColor};
use anyhow::Result;

pub struct WriterManager {
    inner_writer: Box<dyn TopicWriter>,
    old_list: Option<Vec<String>>,
}

impl TopicWriter for WriterManager {
    fn write(&mut self, list: &[String]) -> Result<()> {
        if self.compare_to_old(list) {
            return Ok(());
        }
        self.old_list = Some(list.to_vec());
        self.inner_writer.write(list)
    }

    fn try_write(&mut self, list: &[String]) {
        _ = self.write(list);
    }

    fn close(&self) -> Result<()> {
        self.inner_writer.close()
    }

    fn read_list(&mut self) -> Result<Vec<String>> {
        let list = self.inner_writer.read_list()?;
        self.old_list = Some(list.clone());
        Ok(list)
    }

    fn get_banner(&self) -> &str {
        self.inner_writer.get_banner()
    }

    fn get_banner_color(&self) -> &BannerColor {
        self.inner_writer.get_banner_color()
    }
}

impl WriterManager {
    pub fn new(inner_writer: Box<dyn TopicWriter>) -> Self {
        Self {
            inner_writer,
            old_list: None,
        }
    }

    fn compare_to_old(&self, list: &[String]) -> bool {
        match &self.old_list {
            Some(old) => {
                if old.len() != list.len() {
                    return false;
                }
                for i in 0..old.len() {
                    if list[i] != old[i] {
                        return false;
                    }
                }
                true
            }
            None => false,
        }
    }
}
