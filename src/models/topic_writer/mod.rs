pub mod local_file_handler;
pub mod network_file_handler;

use crate::settings::BannerColor;
use std::io;

pub trait TopicWriter {
    fn write(&self, list: &[String]) -> io::Result<()>;

    fn try_write(&self, list: &[String]);

    fn overwrite_old(&self) -> io::Result<()>;

    fn check_source_exist(&self);

    fn read_list(&self) -> io::Result<Vec<String>>;

    fn get_banner(&self) -> &str;

    fn get_banner_color(&self) -> &BannerColor;
}
