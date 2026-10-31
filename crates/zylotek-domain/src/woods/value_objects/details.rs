use std::{collections::BTreeMap, fmt};

use crate::woods::{
    errors::WoodError,
    value_objects::{
        measurements::Days,
        mechanical_properties::MechanicalProperties,
        names::{BotanicalFamily, CommonNames},
        physical_properties::PhysicalProperties,
        workabilities::Workabilities,
    },
};

/// Everything known about a wood besides its identity (name and scientific name).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WoodDetails {
    pub family: Option<BotanicalFamily>,
    pub collection_location: Option<String>,
    pub common_names: CommonNames,
    pub source: Option<SourceReference>,
    pub description: WoodDescription,
    pub kiln_drying: KilnDrying,
    pub images: WoodImages,
    pub physical_properties: PhysicalProperties,
    pub mechanical_properties: MechanicalProperties,
    pub workabilities: Workabilities,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WoodDescription {
    pub tree: Attributes,
    pub general_characteristics: Attributes,
    pub end_uses: Option<String>,
    pub preservation: Option<String>,
    pub workability_notes: Option<String>,
    pub heartwood_color_classification: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KilnDrying {
    pub schedule: Attributes,
    pub duration: Option<Days>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WoodImages {
    pub main: Option<WebUrl>,
    pub tree: Option<WebUrl>,
    pub log: Option<WebUrl>,
    pub bark: Option<WebUrl>,
    pub tangential_cut: Option<WebUrl>,
    pub cross_section: Option<WebUrl>,
    pub radial_cut: Option<WebUrl>,
    pub wood: Option<WebUrl>,
}

/// Where the data was collected from: the LPF species page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceReference {
    pub species_id: u64,
    pub page_url: WebUrl,
}

/// Free-form labelled attributes, e.g. tree height or general characteristics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Attributes(BTreeMap<String, String>);

impl Attributes {
    pub fn new<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        Self(
            pairs
                .into_iter()
                .map(|(key, value)| {
                    (
                        key.as_ref().trim().to_owned(),
                        value.as_ref().trim().to_owned(),
                    )
                })
                .filter(|(key, _)| !key.is_empty())
                .collect(),
        )
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// An absolute http(s) URL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebUrl(String);

impl WebUrl {
    pub fn new(value: impl AsRef<str>) -> Result<Self, WoodError> {
        let value = value.as_ref().trim();
        let rest = value
            .strip_prefix("https://")
            .or_else(|| value.strip_prefix("http://"));

        match rest {
            Some(rest) if !rest.is_empty() && !rest.contains(char::is_whitespace) => {
                Ok(Self(value.to_owned()))
            }
            _ => Err(WoodError::InvalidUrl(value.to_owned())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WebUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
