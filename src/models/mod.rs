pub mod local_topic_writer;
pub mod model;
pub mod network_topic_writer;
pub mod topic_handler;
pub mod undo_redo_handler;

use crate::settings::BannerColor;
use std::io;

pub trait TopicWriter {
    fn write(&self, list: &[String]) -> io::Result<()>;

    fn try_write(&self, list: &[String]);

    fn overwrite_old(&self) -> io::Result<()>;

    fn check_source_exist(&self);

    fn read_list(&mut self) -> io::Result<Vec<String>>;

    fn get_banner(&self) -> &str;

    fn get_banner_color(&self) -> &BannerColor;
}
