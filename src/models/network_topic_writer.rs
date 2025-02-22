use crate::{
    models::TopicWriter,
    settings::{BannerColor, List},
};
use anyhow::anyhow;
use reqwest::{self, blocking::Client};
use std::io;

pub struct NetworkTopicWriter {
    client: Client,
    endpoint_url: String,
    banner: String,
    banner_color: BannerColor,
}

impl TopicWriter for NetworkTopicWriter {
    fn write(&mut self, list: &[String]) -> anyhow::Result<()> {
        self.put_data(list, &self.endpoint_url)
    }

    fn close(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn read_list(&mut self) -> anyhow::Result<Vec<String>> {
        let response = self.client.get(&self.endpoint_url).send()?;

        if response.status().is_success() {
            let text = response.text()?;
            let list: Vec<String> = text.lines().map(|line| line.to_string()).collect();
            Ok(list)
        } else {
            Err(anyhow!(format!(
                "Failed to read list: HTTP {}",
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

impl NetworkTopicWriter {
    pub fn new(list: &List) -> Self {
        let endpoint_url = list.path().to_string();

        Self {
            client: Client::new(),
            endpoint_url,
            banner: Self::fetch_banner(list.banner_path()),
            banner_color: list.banner_color().clone(),
        }
    }

    fn put_data(&self, list: &[String], url: &str) -> anyhow::Result<()> {
        let response = self
            .client
            .put(url)
            .body(list.join("\n"))
            .send()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!(format!(
                "Failed to put data: HTTP {}",
                response.status()
            )))
        }
    }

    fn fetch_banner(banner_url: &str) -> String {
        let client = Client::new();
        client
            .get(banner_url)
            .send()
            .and_then(|resp| resp.text())
            .unwrap_or_default()
    }
}
