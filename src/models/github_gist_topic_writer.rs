use crate::{
    models::TopicWriter,
    settings::{BannerColor, List},
};
use anyhow::anyhow;
use reqwest::blocking::Client;
use serde_json::json;

const GITHUB_API_PREFIX: &str = "https://api.github.com/gists";
const GITHUB_API_HEADER: &str = "application/vnd.github.v3+json";
const RAW_BASE_URL: &str = "https://gist.githubusercontent.com/";
const DELIMITER: &str = "/raw/";

pub struct GithubGistTopicWriter {
    token: String,
    client: Client,
    gist_id: String,
    file_name: String,
    banner: String,
    banner_color: BannerColor,
    old_list: Option<Vec<String>>,
}

impl TopicWriter for GithubGistTopicWriter {
    fn write(&self, list: &[String]) -> anyhow::Result<()> {
        let payload = json!({
            "files": {
                self.file_name.clone(): {
                    "content": list.join("\n")
                }
            }
        });

        let response = self
            .client
            .patch(format!("{GITHUB_API_PREFIX}/{}", self.gist_id))
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", GITHUB_API_HEADER)
            .header("User-Agent", "rust-gist-updater")
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to update gist: {}", response.status()))
        }
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
            .get(format!("{GITHUB_API_PREFIX}/{}", self.gist_id))
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", GITHUB_API_HEADER)
            .header("User-Agent", "rust-gist-reader")
            .send()?;

        if !response.status().is_success() {
            return Err(anyhow!(format!(
                "Failed to read gist: HTTP {}",
                response.status()
            )));
        }

        let gist_data: serde_json::Value = response.json()?;
        if let Some(file_content) = gist_data["files"]
            .get(&self.file_name)
            .and_then(|file| file["content"].as_str())
        {
            let list: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
            self.old_list = Some(list.clone());
            Ok(list)
        } else {
            Err(anyhow!("File not found in gist response"))
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
        let (gist_id, file_name) =
            Self::parse_gist_url(list.path()).expect("Incorrect gist url format.");

        let client = Client::new();

        Self {
            token: list.access_token().to_string(),
            old_list: None,
            gist_id,
            file_name,
            banner: Self::fetch_banner(&client, list.banner_path()),
            banner_color: list.banner_color().clone(),
            client,
        }
    }

    fn parse_gist_url(url: &str) -> Option<(String, String)> {
        let base_idx = url.find(RAW_BASE_URL)?;
        let url_after_base = &url[base_idx + RAW_BASE_URL.len()..];
        let raw_idx = url_after_base.find(DELIMITER)?;

        let mut gist_id = &url_after_base[..raw_idx];
        let slash_index = gist_id.find('/')?;
        gist_id = &gist_id[(slash_index + 1)..];

        let mut file_name = &url_after_base[raw_idx + DELIMITER.len()..];
        let slash_index = file_name.find('/')?;
        file_name = &file_name[(slash_index + 1)..];

        Some((gist_id.to_string(), file_name.to_string()))
    }

    fn fetch_banner(client: &Client, banner_url: &str) -> String {
        match client.get(banner_url).send() {
            Ok(response) => response.text().unwrap_or_default(),
            Err(_) => String::new(),
        }
    }
}
