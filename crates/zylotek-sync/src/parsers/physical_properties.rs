use anyhow::{Context, Result, anyhow};
use scraper::ElementRef;
use zylotek_domain::woods::value_objects::{Density, PhysicalProperties, Shrinkage};

use crate::parsers::{helpers, selectors};

pub(crate) fn parse(container: ElementRef<'_>) -> Result<PhysicalProperties> {
    let table_rows =
        selectors::table_rows_by_heading(container, html::PHYSICS_PROPERTIES_TABLE_HEADING)
            .with_context(|| "physical properties table not found")?;

    let physical_properties_row = table_rows
        .get(2)
        .ok_or_else(|| anyhow!("physical properties row missing"))?;

    Ok(PhysicalProperties {
        density: Density {
            dry: helpers::measurement(physical_properties_row.first())?,
            green: helpers::measurement(physical_properties_row.get(1))?,
            basic: helpers::measurement(physical_properties_row.get(2))?,
            apparent: helpers::measurement(physical_properties_row.get(3))?,
        },

        shrinkage: Shrinkage {
            tangential: helpers::measurement(physical_properties_row.get(4))?,
            radial: helpers::measurement(physical_properties_row.get(5))?,
            volumetric: helpers::measurement(physical_properties_row.get(6))?,
        },
    })
}

mod html {
    pub const PHYSICS_PROPERTIES_TABLE_HEADING: &str = "propriedades físicas";
}
