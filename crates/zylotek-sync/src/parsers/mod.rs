use anyhow::{Result, anyhow};
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

use crate::{
    Wood,
    parsers::{
        basic_information::BasicInformation, mechanical_properties::MechanicalProperties,
        physical_properties::PhysicalProperties, workabilities::Workabilities,
    },
};

mod helpers;
mod selectors;

pub mod basic_information;
pub mod mechanical_properties;
pub mod physical_properties;
pub mod workabilities;

pub fn parse_wood(html: &str, source_url: &str) -> Result<Wood> {
    let document = Html::parse_document(html);
    let base_url = Url::parse(source_url)?;
    let id = species_id(source_url)?;
    let container = content_container(&document)?;

    let table_selector = selectors::query("table")?;
    if container.select(&table_selector).count() > 8 {
        log::warn!("Page tables were not being processed: {source_url}");
    }

    let basic_information = BasicInformation::parse(container)?;
    let workabilities = Workabilities::parse(container)?;
    let physical_properties = PhysicalProperties::parse(container)?;
    let mechanical_properties = MechanicalProperties::parse(container)?;
    let image_url = absolute_image_url(&base_url, Some(&basic_information.image_url))
        .ok_or_else(|| anyhow!("image not found"))?;

    Ok(Wood {
        id,
        name: basic_information.name,
        image_url,
        scientific_name: basic_information.scientific_name,
        family: basic_information.family,
        collection_location: basic_information.collection_location,
        source_page_link: source_url.to_owned(),

        common_names: basic_information.common_names,
        tree: basic_information.tree,
        general_characteristics: basic_information.general_characteristics,
        end_uses: basic_information.end_uses,
        klin_drying: basic_information.klin_drying,
        preservation: basic_information.preservation,
        workability: basic_information.workability,
        klin_drying_days: basic_information.klin_drying_days,
        heartwood_color_classification: basic_information.heartwood_color_classification,

        tree_image_url: basic_information
            .tree_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        log_image_url: basic_information
            .log_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        bark_image_url: basic_information
            .bark_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        tangencial_cut_image_url: basic_information
            .tangencial_cut_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        cross_section_image_url: basic_information
            .cross_section_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        radial_cut_image_url: basic_information
            .radial_cut_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),
        wood_image_url: basic_information
            .wood_image_url
            .and_then(|image_url| absolute_image_url(&base_url, Some(&image_url))),

        workabilities,
        physical_properties,
        mechanical_properties,
    })
}

fn species_id(url: &str) -> Result<u64> {
    let url = Url::parse(url)?;

    url.query_pairs()
        .find(|(key, _)| key == "especieestudadaid")
        .and_then(|(_, value)| value.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("especieestudadaid not found in URL"))
}

fn absolute_image_url(base_url: &Url, src: Option<&str>) -> Option<String> {
    src.and_then(|src| base_url.join(src).ok().map(|url| url.to_string()))
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
