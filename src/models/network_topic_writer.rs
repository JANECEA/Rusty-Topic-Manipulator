use crate::{
    models::TopicWriter,
    settings::{BannerColor, List},
};
use anyhow::anyhow;
use reqwest::{self, blocking::Client};
use std::io;

pub struct NetworkTopicWriter {
    client: Client,
    old_list: Option<Vec<String>>,
    endpoint_url: String,
    backup_url: String,
    banner: String,
    banner_color: BannerColor,
}

impl TopicWriter for NetworkTopicWriter {
    fn write(&self, list: &[String]) -> anyhow::Result<()> {
        self.put_data(list, &self.endpoint_url)
    }

    fn try_write(&self, list: &[String]) {
        let _ = self.write(list);
    }

    fn close(&self) -> anyhow::Result<()> {
        let Some(list) = &self.old_list else {
            return Ok(());
        };

        self.put_data(list, &self.backup_url)
    }

    fn check_source_exist(&self) {
        let response = self.client.head(&self.endpoint_url).send();

        match response {
            Ok(resp) if resp.status().is_success() => (),
            _ => panic!("Source does not exist or is inaccessible."),
        }
    }

    fn read_list(&mut self) -> anyhow::Result<Vec<String>> {
        let response = self.client.get(&self.endpoint_url).send()?;

        if response.status().is_success() {
            let text = response.text()?;

            let list: Vec<String> = text.lines().map(|line| line.to_string()).collect();
            self.old_list = Some(list.clone());
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
        let backup_url = format!("{}.old", list.path());

        Self {
            client: Client::new(),
            old_list: None,
            endpoint_url,
            backup_url,
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
