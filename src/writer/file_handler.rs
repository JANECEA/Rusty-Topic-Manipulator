use crate::writer::topic_writer::TopicWriter;
use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
};

pub const SETTINGS_DIR_NAME: &str = "RustyTopicManipulator";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

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
    pub fn new(topics_file_name: &str, topics_file_old_name: &str) -> Self {
        let documents_dir: PathBuf = LocalTopicFileHandler::init_documents_dir();
        let topics_file_dir: PathBuf = documents_dir.join(SETTINGS_DIR_NAME);
        let topics_file_path: PathBuf = topics_file_dir.join(topics_file_name);
        let topics_file_old_path: PathBuf = topics_file_dir.join(topics_file_old_name);
        Self {
            topics_file_dir,
            topics_file_path,
            topics_file_old_path,
            banner: Self::set_banner(),
        }
    }

    fn set_banner() -> String {
        r"
         |@@@@@@@'                                                               ##^'     '^##
      @@@@@@@@@@@@@@@       ___  ___  ____  ____ ______ __ __  __   ___        #              '#
    @@@M@@@@@@@@@@@@@@@     ||\ //|| ||    ||    | || | || ||\ ||  // \       #                 #
   @@@@@@@  @@@  @@@@@@@    || \/ || ||==  ||==    ||   || ||\ || (( ___     #   .-.       .-.   #
   @@     @@@@@@@     @@    ||    || ||___ ||___   ||   || || \||  \_||      #   ##-       -##   #
   @@     @@    @     @@                                                     +        '-'        #
    @@@@@@@ @@@ @@@@@@@          ______  ___   ____  __  ___   __          .- #-               .# --.
  @@  @@@@@@@M@@@@@@@   @        | || |  // \  ||  \ ||  //   (( \         +   ##+++++----++###-#   +
  @@@  @@@@@@@@@@@@@  @@@          ||   ((  )) ||_// || ((     \           '+ #     +      +    +.-'
    @@@@@  @@M@@@ @@@@             ||    \_//  ||    ||  \__  \_))           +      #      +     #
    @@@@@  @@@@@@ @@@@@@                                                      +    +#      #     #
           @@@@@@                                                              ''#' '-.__.+ '##''
        ".to_string()
    }

    fn init_documents_dir() -> PathBuf {
        match directories::UserDirs::new() {
            Some(user_dirs) => user_dirs
                .document_dir()
                .map_or_else(|| PathBuf::from("/"), PathBuf::from),
            None => PathBuf::from("/"),
        }
    }
}
