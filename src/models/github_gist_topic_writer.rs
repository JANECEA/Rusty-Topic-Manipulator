use crate::{
    models::TopicWriter,
    settings::{BannerColor, List},
};
use anyhow::anyhow;
use octocrab::Octocrab;
use reqwest::blocking::Client;
use std::io;
use tokio::runtime::Runtime;

pub struct GithubGistTopicWriter {
    octocrab: Octocrab,
    client: Client,
    raw_list_url: String,
    gist_id: String,
    file_name: String,
    token: String,
    banner: String,
    banner_color: BannerColor,
    old_list: Option<Vec<String>>,
}

impl TopicWriter for GithubGistTopicWriter {
    fn write(&self, list: &[String]) -> anyhow::Result<()> {
        println!("{}", &self.gist_id);
        println!("{}", &self.raw_list_url);
        println!("{}", &self.file_name);
        println!("{}", &self.token);
        let response = Runtime::new()?.block_on(async {
            self.octocrab
                .gists()
                .update(&self.gist_id)
                .file(&self.file_name)
                .with_content(list.join("\n"))
                .send()
                .await
        })?;

        Ok(())
    }

    fn try_write(&self, list: &[String]) {
        let _ = self.write(list);
    }

    fn close(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn check_source_exist(&self) {}

    fn read_list(&mut self) -> anyhow::Result<Vec<String>> {
        let response = self
            .client
            .get(&self.raw_list_url)
            .send()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            let text = response
                .text()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let list: Vec<String> = text.lines().map(|line| line.to_string()).collect();
            self.old_list = Some(list.clone());
            Ok(list)
        } else {
            Err(anyhow!(format!(
                "Failed to read gist: HTTP {}",
                response.status()
            )))
        }
    }
    fn get_banner(&self) -> &str {
        self.banner.as_str()
    }

    fn get_banner_color(&self) -> &BannerColor {
        &self.banner_color
    }
}

impl GithubGistTopicWriter {
    pub fn new(list: &List) -> Self {
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let octocrab = runtime.block_on(async {
            Octocrab::builder()
                .personal_token(list.access_token().to_string())
                .build()
                .unwrap()
        });

        let (gist_id, file_name) = Self::parse_gist_url(list.path()).unwrap();
        let client = Client::new();

        Self {
            old_list: None,
            raw_list_url: list.path().to_string(),
            gist_id,
            file_name,
            token: list.access_token().to_string(),
            banner: Self::fetch_banner(&client, list.banner_path()),
            banner_color: list.banner_color().clone(),
            octocrab,
            client,
        }
    }

    fn parse_gist_url(url: &str) -> Option<(String, String)> {
        let base_url = "https://gist.githubusercontent.com/";
        if let Some(base_idx) = url.find(base_url) {
            let url_after_base = &url[base_idx + base_url.len()..];
            if let Some(raw_idx) = url_after_base.find("/raw/") {
                let mut gist_id = &url_after_base[..raw_idx]; // Gist ID is before '/raw/'
                let slash_index = gist_id.find('/').unwrap();
                gist_id = &gist_id[(slash_index+1)..];
                
                let mut file_name = &url_after_base[raw_idx + 5..]; // File name comes after '/raw/'
                let slash_index = file_name.find('/').unwrap();
                file_name = &file_name[(slash_index+1)..];

                return Some((gist_id.to_string(), file_name.to_string()));
            }
        }

        None
    }

    fn fetch_banner(client: &Client, banner_url: &str) -> String {
        match client.get(banner_url).send() {
            Ok(response) => response.text().unwrap_or_default(),
            Err(_) => String::new(),
        }
    }
}
