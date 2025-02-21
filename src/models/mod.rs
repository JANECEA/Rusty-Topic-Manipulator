pub mod github_gist_topic_writer;
pub mod local_topic_writer;
pub mod model;
pub mod network_topic_writer;
pub mod topic_handler;
pub mod undo_redo_handler;

use crate::settings::BannerColor;

pub trait TopicWriter {
    fn write(&mut self, list: &[String]) -> anyhow::Result<()>;

    fn try_write(&mut self, list: &[String]);

    fn close(&self) -> anyhow::Result<()>;

    fn read_list(&mut self) -> anyhow::Result<Vec<String>>;

    fn get_banner(&self) -> &str;

    fn get_banner_color(&self) -> &BannerColor;
}
