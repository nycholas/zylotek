use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use scraper::ElementRef;

use crate::parsers::{helpers, selectors};

#[derive(Debug, Clone)]
pub struct BasicInformation {
    pub name: String,
    pub image_url: String,
    pub scientific_name: String,
    pub family: String,
    pub collection_location: String,
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
}

impl BasicInformation {
    pub(crate) fn parse(container: ElementRef<'_>) -> Result<BasicInformation> {
        let name_selector = selectors::query("h4")?;
        let name = container
            .select(&name_selector)
            .next()
            .map(selectors::text)
            .ok_or_else(|| anyhow!("species name not found"))?;

        let image_url_selector = selectors::query("img")?;
        let image_url = container
            .select(&image_url_selector)
            .next()
            .ok_or_else(|| anyhow!("image url not found"))
            .map(|img| img.attr("src").unwrap().to_string())?;

        let p_selector = selectors::query("p")?;
        let span_selector = selectors::query("span")?;

        let mut label_and_texts = HashMap::new();
        for p_element in container.select(&p_selector) {
            let inner_text = p_element.text().collect::<String>();
            let label = p_element
                .select(&span_selector)
                .next()
                .map(|span| span.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            let text = inner_text
                .strip_prefix(&label)
                .unwrap_or(&inner_text)
                .trim()
                .to_string();

            let attr_name = label.replace(':', "").to_lowercase();
            if attr_name.is_empty() {
                continue;
            }

            label_and_texts.insert(attr_name, text);
        }

        let mut images = HashMap::new();
        let img_selector = selectors::query(r#"div > a > img"#)?;
        for img_element in container.select(&img_selector) {
            let node = img_element.parent().unwrap().parent().unwrap();
            if let Some(element) = ElementRef::wrap(node) {
                let attr_name = element
                    .select(&p_selector)
                    .next()
                    .map(|p| p.text().collect::<String>().trim().to_lowercase())
                    .unwrap_or_default();
                if attr_name.is_empty() {
                    continue;
                }
                img_element
                    .attr("src")
                    .map(|src| images.insert(attr_name, src.to_string()));
            }
        }

        let scientific_name = label_and_texts
            .remove(html::SCIENTIFIC_NAME_LABEL)
            .with_context(|| "scientific name not found")?;

        let family = label_and_texts
            .remove(html::FAMILY_LABEL)
            .with_context(|| "family not found")?;

        let collection_location = label_and_texts
            .remove(html::COLLECTION_LOCATION_LABEL)
            .with_context(|| "collection location not found")?;

        let common_names = label_and_texts
            .remove(html::COMMON_NAMES_LABEL)
            .map(|text| {
                text.split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect()
            });

        let tree = label_and_texts.remove(html::TREE).map(|text| {
            text.split(';')
                .map(|str| str.splitn(2, ':').map(str::trim).map(str::to_string))
                .map(|mut list| {
                    (
                        list.next().unwrap_or("".to_string()),
                        list.next().unwrap_or("".to_string()),
                    )
                })
                .collect()
        });

        let general_characteristics = label_and_texts
            .remove(html::GENERAL_CHARACTERISTICS_LABEL)
            .map(|text| {
                text.split(';')
                    .map(|str| str.splitn(2, ':').map(str::trim).map(str::to_string))
                    .map(|mut list| {
                        (
                            list.next().unwrap_or("".to_string()),
                            list.next().unwrap_or("".to_string()),
                        )
                    })
                    .collect()
            });

        let end_uses = label_and_texts.remove(html::END_USES_LABEL);

        let klin_drying = label_and_texts.remove(html::KLIN_DRYING_LABEL).map(|text| {
            text.split(';')
                .map(|str| str.splitn(2, ':').map(str::trim).map(str::to_string))
                .map(|mut list| {
                    (
                        list.next().unwrap_or("".to_string()),
                        list.next().unwrap_or("".to_string()),
                    )
                })
                .collect()
        });

        let preservation = label_and_texts.remove(html::PRESERVATION_LABEL);

        let workability = label_and_texts.remove(html::WORKABILITY_LABEL);

        let klin_drying_days = helpers::optional_decimal(
            label_and_texts
                .remove(html::KLIN_DRYING_DAYS_LABEL)
                .as_ref(),
        );

        let heartwood_color_classification =
            label_and_texts.remove(html::HEARTWOOD_COLOR_CLASSIFICATION_LABEL);

        let tree_image_url = images.remove(html::TREE_IMAGE_LABEL);
        let log_image_url = images.remove(html::LOG_IMAGE_LABEL);
        let bark_image_url = images.remove(html::BARK_IMAGE_LABEL);
        let tangencial_cut_image_url = images.remove(html::TANGENCIAL_CUT_IMAGE_LABEL);
        let cross_section_image_url = images.remove(html::CROSS_SECTION_IMAGE_LABEL);
        let radial_cut_image_url = images.remove(html::RADIAL_CUT_IMAGE_LABEL);
        let wood_image_url = images.remove(html::WOOD_IMAGE_LABEL);

        if !label_and_texts.is_empty() || !images.is_empty() {
            log::warn!(
                "Attributes were not processed for {scientific_name}: {label_and_texts:?}, {images:?}"
            );
        }

        Ok(BasicInformation {
            name,
            image_url,
            scientific_name,
            family,
            collection_location,
            common_names,
            tree,
            general_characteristics,
            end_uses,
            klin_drying,
            preservation,
            workability,
            klin_drying_days,
            heartwood_color_classification,
            tree_image_url,
            log_image_url,
            bark_image_url,
            tangencial_cut_image_url,
            cross_section_image_url,
            radial_cut_image_url,
            wood_image_url,
        })
    }
}

mod html {
    pub const SCIENTIFIC_NAME_LABEL: &str = "nome científico";
    pub const FAMILY_LABEL: &str = "família";
    pub const COLLECTION_LOCATION_LABEL: &str = "local de coleta";
    pub const TREE: &str = "árvore";
    pub const COMMON_NAMES_LABEL: &str = "nomes comuns";
    pub const GENERAL_CHARACTERISTICS_LABEL: &str = "características gerais";
    pub const END_USES_LABEL: &str = "uso final";
    pub const KLIN_DRYING_LABEL: &str = "secagem em estufa";
    pub const PRESERVATION_LABEL: &str = "preservação";
    pub const WORKABILITY_LABEL: &str = "trabalhabilidade";
    pub const KLIN_DRYING_DAYS_LABEL: &str = "tempo de secagem (dias)";
    pub const HEARTWOOD_COLOR_CLASSIFICATION_LABEL: &str = "classificação da cor do cerne";
    pub const TREE_IMAGE_LABEL: &str = "árvore";
    pub const LOG_IMAGE_LABEL: &str = "tora";
    pub const BARK_IMAGE_LABEL: &str = "casca";
    pub const TANGENCIAL_CUT_IMAGE_LABEL: &str = "corte tangencial";
    pub const CROSS_SECTION_IMAGE_LABEL: &str = "seção transversal";
    pub const RADIAL_CUT_IMAGE_LABEL: &str = "corte radial";
    pub const WOOD_IMAGE_LABEL: &str = "madeira";
}
