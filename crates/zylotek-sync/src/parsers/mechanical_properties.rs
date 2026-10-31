use anyhow::{Context, Result, anyhow};
use scraper::ElementRef;
use zylotek_domain::woods::value_objects::{
    Compression, JankaHardness, MechanicalProperties, MechanicalPropertiesByCondition,
    MoistureCondition, NailExtraction, StaticBending,
};

use crate::parsers::{helpers, selectors};

pub(crate) fn parse(container: ElementRef<'_>) -> Result<MechanicalProperties> {
    Ok(MechanicalProperties {
        green: parse_condition(container, MoistureCondition::Green)?,
        dry: parse_condition(container, MoistureCondition::Dry)?,
    })
}

fn parse_condition(
    container: ElementRef<'_>,
    condition: MoistureCondition,
) -> Result<MechanicalPropertiesByCondition> {
    let bending = condition_row(
        container,
        html::STATIC_BENDING_TABLE_HEADER_COLUMN,
        condition,
    )?;
    let compression = condition_row(container, html::COMPRESSION_TABLE_HEADER_COLUMN, condition)?;
    let janka_hardness = condition_row(
        container,
        html::JANKA_HARDNESS_TABLE_HEADER_COLUMN,
        condition,
    )?;
    let tension = condition_row(container, html::TENSION_TABLE_HEADER_COLUMN, condition)?;
    let cleavage_strength = condition_row(
        container,
        html::CLEAVAGE_STRENGTH_TABLE_HEADER_COLUMN,
        condition,
    )?;
    let shear_parallel_to_grain = condition_row(
        container,
        html::SHEAR_PARALLEL_TO_GRAIN_TABLE_HEADER_COLUMN,
        condition,
    )?;
    let nail_extraction = condition_row(
        container,
        html::NAIL_EXTRACTION_TABLE_HEADER_COLUMN,
        condition,
    )?;

    Ok(MechanicalPropertiesByCondition {
        static_bending: StaticBending {
            modulus_of_rupture: helpers::measurement(bending.first())?,
            modulus_of_elasticity: helpers::measurement(bending.get(1))?,
        },
        compression: Compression {
            parallel_to_grain: helpers::measurement(compression.first())?,
            perpendicular_to_grain: helpers::measurement(compression.get(1))?,
        },
        janka_hardness: JankaHardness {
            parallel_to_grain: helpers::measurement(janka_hardness.first())?,
            end_grain: helpers::measurement(janka_hardness.get(1))?,
        },
        tension_perpendicular_to_grain: helpers::measurement(tension.first())?,
        cleavage_strength: helpers::measurement(cleavage_strength.first())?,
        shear_parallel_to_grain: helpers::measurement(shear_parallel_to_grain.first())?,
        nail_extraction: NailExtraction {
            parallel_to_grain: helpers::measurement(nail_extraction.first())?,
            end_grain: helpers::measurement(nail_extraction.get(1))?,
        },
    })
}

/// Cells under the `header` column for one moisture condition: each table lists
/// green specimens on the first row and dry specimens on the second.
fn condition_row(
    container: ElementRef<'_>,
    header: &str,
    condition: MoistureCondition,
) -> Result<Vec<String>> {
    let table_rows = selectors::table_rows_by_header(container, header)
        .with_context(|| format!("mechanical properties table {header} not found"))?;

    let index = match condition {
        MoistureCondition::Green => 0,
        MoistureCondition::Dry => 1,
    };

    table_rows
        .into_iter()
        .nth(index)
        .ok_or_else(|| anyhow!("mechanical properties {header} {condition:?} row missing"))
}

mod html {
    pub const STATIC_BENDING_TABLE_HEADER_COLUMN: &str = "flexão estática (kgf/cm²)";
    pub const COMPRESSION_TABLE_HEADER_COLUMN: &str = "compressão (kgf/cm²)";
    pub const JANKA_HARDNESS_TABLE_HEADER_COLUMN: &str = "dureza janka (kgf)";
    pub const TENSION_TABLE_HEADER_COLUMN: &str = "tração (kgf/cm²)";
    pub const CLEAVAGE_STRENGTH_TABLE_HEADER_COLUMN: &str = "fendilhamento (kgf/cm)";
    pub const SHEAR_PARALLEL_TO_GRAIN_TABLE_HEADER_COLUMN: &str = "cisalhamento (kgf/cm²)";
    pub const NAIL_EXTRACTION_TABLE_HEADER_COLUMN: &str = "extração de pregos (kgf)";
}
