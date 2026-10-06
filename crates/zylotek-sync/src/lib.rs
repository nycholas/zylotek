mod parsers;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use tokio::{self, task::JoinHandle};

use crate::parsers::{
    mechanical_properties::MechanicalProperties, physical_properties::PhysicalProperties,
    workabilities::Workabilities,
};

const TOTAL_PAGE_NUMBER: usize = 13;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Syncronizer {}

impl Syncronizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> SyncronizerRunner {
        SyncronizerRunner {}
    }
}

impl Default for Syncronizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SyncronizerRunner {}

impl SyncronizerRunner {
    pub fn run(self) -> Result<(), Error> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let mut handles: Vec<tokio::task::JoinHandle<Result<_, Error>>> = Vec::new();
        let client = reqwest::Client::builder()
            .connection_verbose(false)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:156.0) Gecko/20100101 Firefox/156.0")
            .build()?;
        let url = "https://lpf.florestal.gov.br/pt-br/madeiras-brasileiras";

        let num_pages = TOTAL_PAGE_NUMBER;
        for i in 0..=num_pages {
            let client = client.clone();
            let next_page = format!("{}?start={}", url, i * 20);

            handles.push(rt.spawn(async move {
                let wood_tasks = fetch_woods(&client, &next_page).await?;
                let woods: Vec<Wood> = futures::future::try_join_all(wood_tasks)
                    .await?
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(woods)
            }));
        }

        for handle in handles {
            let woods = rt
                .block_on(handle)
                .map_err(|e| Error::from(e.to_string()))??;
            log::debug!("{woods:#?}");
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct WoodFetcher {
    id: u64,
    name: String,
    scientific_name: String,
    collection_location: String,
    source_page_link: String,
}

impl WoodFetcher {
    fn new(
        name: String,
        scientific_name: String,
        collection_location: String,
        source_page_link: String,
    ) -> Self {
        Self {
            id: Self::calculate_id(&scientific_name),
            name,
            scientific_name,
            collection_location,
            source_page_link,
        }
    }

    async fn fetch(&self, client: &reqwest::Client) -> Result<Wood> {
        fetch_wood(client, &self.source_page_link).await
    }

    fn calculate_id(str: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        str.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone)]
pub struct Wood {
    pub id: u64,
    pub name: String,
    pub image_url: String,
    pub scientific_name: String,
    pub family: String,
    pub collection_location: String,
    pub source_page_link: String,
    pub common_names: Option<Vec<String>>,
    pub tree: Option<HashMap<String, String>>,
    pub general_characteristics: Option<HashMap<String, String>>,
    pub end_uses: Option<String>,
    pub klin_drying: Option<HashMap<String, String>>,
    pub preservation: Option<String>,
    pub workability: Option<String>,
    pub klin_drying_days: Option<f64>,
    pub heartwood_color_classification: Option<String>,
    pub tree_image_url: Option<String>,
    pub log_image_url: Option<String>,
    pub bark_image_url: Option<String>,
    pub tangencial_cut_image_url: Option<String>,
    pub cross_section_image_url: Option<String>,
    pub radial_cut_image_url: Option<String>,
    pub wood_image_url: Option<String>,
    pub workabilities: Workabilities,
    pub physical_properties: PhysicalProperties,
    pub mechanical_properties: MechanicalProperties,
}

async fn fetch_wood(client: &reqwest::Client, url: &str) -> Result<Wood> {
    log::info!(" + Fetching wood {url}");
    let response = client
        .get(url)
        .header("Accept", "text/html")
        .send()
        .await?
        .error_for_status()?;
    let body = response.text().await?;

    parsers::parse_wood(&body, url).with_context(|| format!("failed to parse the page: {}", url))
}

type WoodFetcherTask = JoinHandle<Result<Wood>>;

async fn fetch_woods(client: &reqwest::Client, url: &str) -> Result<Vec<WoodFetcherTask>> {
    log::info!("Fetching page: {url}");
    let response = client
        .get(url)
        .header("Accept", "text/html")
        .send()
        .await?
        .error_for_status()?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let line_selector = Selector::parse(r#"table[id=caracteristicaList] > tbody > tr"#).unwrap();
    let column_selector = Selector::parse(r#"td"#).unwrap();
    let link_selector = Selector::parse(r#"a"#).unwrap();

    let mut tasks = Vec::<WoodFetcherTask>::with_capacity(20);
    for line in document.select(&line_selector) {
        let mut columns = line.select(&column_selector);
        let scientific_name_and_link = columns
            .next()
            .unwrap()
            .select(&link_selector)
            .next()
            .unwrap();

        let scientific_name = scientific_name_and_link.inner_html().trim().to_string();
        let link = scientific_name_and_link
            .value()
            .attr("href")
            .unwrap()
            .trim();
        let name = columns
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let collection_location = columns
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();

        let wood_fetcher =
            WoodFetcher::new(name, scientific_name, collection_location, link.to_string());
        let client = client.clone();
        tasks.push(tokio::spawn(
            async move { wood_fetcher.fetch(&client).await },
        ));
    }

    Ok(tasks)
}
