use anyhow::{Context, Result, anyhow};
use scraper::ElementRef;

use crate::parsers::{helpers, selectors};

#[derive(Debug, Clone)]
pub struct PhysicalProperties {
    pub density: Density,
    pub shrinkage: Shrinkage,
}

impl PhysicalProperties {
    pub(crate) fn parse(container: ElementRef<'_>) -> Result<PhysicalProperties> {
        parse_table_physical_properties(container)
    }
}

#[derive(Debug, Clone)]
pub struct Density {
    pub dry: Option<f64>,
    pub green: Option<f64>,
    pub basic: Option<f64>,
    pub apparent: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Shrinkage {
    pub tangential_percent: Option<f64>,
    pub radial_percent: Option<f64>,
    pub volumetric_percent: Option<f64>,
}

impl Shrinkage {
    pub fn tangential_radial_ratio(&self) -> Option<f64> {
        if self.tangential_percent.is_none() || self.radial_percent.is_none() {
            return None;
        }
        self.radial_percent
            .map(|radial_percent| self.tangential_percent.unwrap() / radial_percent)
    }
}

fn parse_table_physical_properties(container: ElementRef<'_>) -> Result<PhysicalProperties> {
    let table_rows =
        selectors::table_rows_by_heading(container, html::PHYSICS_PROPERTIES_TABLE_HEADING)
            .with_context(|| "physical properties table not found")?;

    let physical_properties_row = table_rows
        .get(2)
        .ok_or_else(|| anyhow!("physical properties row missing"))?;

    Ok(PhysicalProperties {
        density: Density {
            dry: helpers::optional_decimal(physical_properties_row.first()),
            green: helpers::optional_decimal(physical_properties_row.get(1)),
            basic: helpers::optional_decimal(physical_properties_row.get(2)),
            apparent: helpers::optional_decimal(physical_properties_row.get(3)),
        },

        shrinkage: Shrinkage {
            tangential_percent: helpers::optional_decimal(physical_properties_row.get(4)),
            radial_percent: helpers::optional_decimal(physical_properties_row.get(5)),
            volumetric_percent: helpers::optional_decimal(physical_properties_row.get(6)),
        },
    })
}

mod html {
    pub const PHYSICS_PROPERTIES_TABLE_HEADING: &str = "propriedades físicas";
}
