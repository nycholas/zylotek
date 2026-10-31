use anyhow::{Result, anyhow};
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
use zylotek_domain::woods::value_objects::{SourceReference, WebUrl, WoodDetails, WoodImages};

use crate::ScrapedWood;

mod helpers;
mod selectors;

pub mod basic_information;
pub mod mechanical_properties;
pub mod physical_properties;
pub mod workabilities;

pub fn parse_wood(html: &str, source_url: &str) -> Result<ScrapedWood> {
    let document = Html::parse_document(html);
    let base_url = Url::parse(source_url)?;
    let species_id = species_id(source_url)?;
    let container = content_container(&document)?;

    let table_selector = selectors::query("table")?;
    if container.select(&table_selector).count() > 8 {
        log::warn!("Page tables were not being processed: {source_url}");
    }

    let basic_information = basic_information::parse(container)?;
    let workabilities = workabilities::parse(container)?;
    let physical_properties = physical_properties::parse(container)?;
    let mechanical_properties = mechanical_properties::parse(container)?;

    let image = |src: Option<String>| -> Result<Option<WebUrl>> {
        Ok(src
            .and_then(|src| absolute_image_url(&base_url, &src))
            .map(WebUrl::new)
            .transpose()?)
    };
    let sources = basic_information.images;
    let main_image = image(Some(sources.main))?.ok_or_else(|| anyhow!("image not found"))?;

    Ok(ScrapedWood {
        name: basic_information.name,
        scientific_name: basic_information.scientific_name,
        details: WoodDetails {
            family: Some(basic_information.family),
            collection_location: Some(basic_information.collection_location),
            common_names: basic_information.common_names,
            source: Some(SourceReference {
                species_id,
                page_url: WebUrl::new(source_url)?,
            }),
            description: basic_information.description,
            kiln_drying: basic_information.kiln_drying,
            images: WoodImages {
                main: Some(main_image),
                tree: image(sources.tree)?,
                log: image(sources.log)?,
                bark: image(sources.bark)?,
                tangential_cut: image(sources.tangential_cut)?,
                cross_section: image(sources.cross_section)?,
                radial_cut: image(sources.radial_cut)?,
                wood: image(sources.wood)?,
            },
            physical_properties,
            mechanical_properties,
            workabilities,
        },
    })
}

fn species_id(url: &str) -> Result<u64> {
    let url = Url::parse(url)?;

    url.query_pairs()
        .find(|(key, _)| key == "especieestudadaid")
        .and_then(|(_, value)| value.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("especieestudadaid not found in URL"))
}

fn absolute_image_url(base_url: &Url, src: &str) -> Option<String> {
    base_url.join(src).ok().map(|url| url.to_string())
}

fn content_container<'a>(document: &'a Html) -> Result<ElementRef<'a>> {
    // The species page is contained inside the Joomla component area.
    //
    // We select the component body structurally.
    //
    // If you inspect the exact HTML and have a more specific stable
    // container class/id, replace this selector with that selector.
    let candidates = [
        ".item-page",
        ".com_madeirasbrasileiras",
        "#component",
        "main",
    ];

    for css in candidates {
        let selector = Selector::parse(css).unwrap();

        if let Some(element) = document.select(&selector).next() {
            return Ok(element);
        }
    }

    let body_selector = selectors::query("body")?;
    document
        .select(&body_selector)
        .next()
        .ok_or_else(|| anyhow!("page content container not found"))
}
