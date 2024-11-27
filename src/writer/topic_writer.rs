use std::io;

pub trait TopicWriter {
    fn write(&self, list: &[String]) -> io::Result<()>;

    fn try_write(&self, list: &[String]);

    fn overwrite_old(&self) -> io::Result<()>;

    fn check_source_exist(&self);

    fn read_list(&self) -> io::Result<Vec<String>>;

    fn get_banner(&self) -> &str;
}
