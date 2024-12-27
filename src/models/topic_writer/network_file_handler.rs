use crate::{models::topic_writer::TopicWriter, settings::BannerColor};
use std::io::{self};

pub struct NetworkFileHandler {
    banner: String,
    banner_color: BannerColor,
}

impl TopicWriter for NetworkFileHandler {
    fn write(&self, list: &[String]) -> io::Result<()> {
        todo!()
    }

    fn try_write(&self, list: &[String]) {
        todo!()
    }

    fn overwrite_old(&self) -> io::Result<()> {
        todo!()
    }

    fn check_source_exist(&self) {
        todo!()
    }

    fn read_list(&self) -> io::Result<Vec<String>> {
        todo!()
    }

    fn get_banner(&self) -> &str {
        &self.banner
    }

    fn get_banner_color(&self) -> &BannerColor {
        &self.banner_color
    }
}

impl NetworkFileHandler {
    pub fn new() -> Self {
        todo!()
    }
}
