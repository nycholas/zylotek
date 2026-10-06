use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use scraper::ElementRef;

use crate::parsers::{helpers, selectors};

#[derive(Debug, Clone)]
pub struct MechanicalProperties {
    pub green: MechanicalPropertiesByCondition,
    pub dry: MechanicalPropertiesByCondition,
}

impl MechanicalProperties {
    pub(crate) fn parse(container: ElementRef<'_>) -> Result<MechanicalProperties> {
        let bending = parse_table_bending_mechanical_properties(container)?;
        let compression = parse_table_compression_mechanical_properties(container)?;
        let janka_hardness = parse_table_janka_hardness_mechanical_properties(container)?;
        let tension = parse_table_tension_mechanical_properties(container)?;
        let cleavage_strength = parse_table_cleavage_strenght_mechanical_properties(container)?;
        let shear_parallel_to_grain =
            parse_table_shear_parallel_to_grain_mechanical_properties(container)?;
        let nail_extaction = parse_table_nail_extraction_mechanical_properties(container)?;

        Ok(MechanicalProperties {
            green: MechanicalPropertiesByCondition {
                bending: bending
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                compression: compression
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                janka_hardness: janka_hardness
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                tension: tension
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                cleavage_strength: cleavage_strength
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                shear_parallel_to_grain: shear_parallel_to_grain
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
                nail_extraction: nail_extaction
                    .get(&MechanicalPropertiesCondition::Green)
                    .unwrap()
                    .to_owned(),
            },
            dry: MechanicalPropertiesByCondition {
                bending: bending
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                compression: compression
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                janka_hardness: janka_hardness
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                tension: tension
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                cleavage_strength: cleavage_strength
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                shear_parallel_to_grain: shear_parallel_to_grain
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
                nail_extraction: nail_extaction
                    .get(&MechanicalPropertiesCondition::Dry)
                    .unwrap()
                    .to_owned(),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct MechanicalPropertiesByCondition {
    pub bending: StaticBending,
    pub compression: Compression,
    pub janka_hardness: JankaHardness,

    pub tension: Tension,
    pub cleavage_strength: CleavageStrength,
    pub shear_parallel_to_grain: ShearParallelToGrain,
    pub nail_extraction: NailExtraction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanicalPropertiesCondition {
    Green,
    Dry,
}

#[derive(Debug, Clone)]
pub struct StaticBending {
    pub modulus_of_rupture: Option<f64>,
    pub modulus_of_elasticity: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Compression {
    pub parallel_to_grain: MaximumCompression,
    pub perpendicular_to_grain: StressAtProportionalLimit,
}

#[derive(Debug, Clone)]
pub struct MaximumCompression {
    pub value: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct StressAtProportionalLimit {
    pub value: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct JankaHardness {
    pub parallel_to_grain: Option<f64>,
    pub end_grain: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Tension {
    pub perpendicular_to_grain: PerpendicularOoGrain,
}

#[derive(Debug, Clone)]
pub struct PerpendicularOoGrain {
    pub maximum_compression: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CleavageStrength {
    pub maximum_compression: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ShearParallelToGrain {
    pub maximum_compression: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct NailExtraction {
    pub parallel_to_grain: Option<f64>,
    pub end_grain: Option<f64>,
}

fn parse_table_bending_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, StaticBending>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_STATIC_BENDING_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table bending not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties bending green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties bending dry row missing"))?;

    let mut static_bending = HashMap::new();
    static_bending.insert(
        MechanicalPropertiesCondition::Green,
        StaticBending {
            modulus_of_rupture: helpers::optional_decimal(green_row.first()),
            modulus_of_elasticity: helpers::optional_decimal(green_row.get(1)),
        },
    );

    static_bending.insert(
        MechanicalPropertiesCondition::Dry,
        StaticBending {
            modulus_of_rupture: helpers::optional_decimal(dry_row.first()),
            modulus_of_elasticity: helpers::optional_decimal(dry_row.get(1)),
        },
    );

    Ok(static_bending)
}

fn parse_table_compression_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, Compression>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_COMPRESSION_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table compression not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties compression green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties compression dry row missing"))?;

    let mut compression = HashMap::new();
    compression.insert(
        MechanicalPropertiesCondition::Green,
        Compression {
            parallel_to_grain: MaximumCompression {
                value: helpers::optional_decimal(green_row.first()),
            },
            perpendicular_to_grain: StressAtProportionalLimit {
                value: helpers::optional_decimal(green_row.get(1)),
            },
        },
    );

    compression.insert(
        MechanicalPropertiesCondition::Dry,
        Compression {
            parallel_to_grain: MaximumCompression {
                value: helpers::optional_decimal(dry_row.first()),
            },
            perpendicular_to_grain: StressAtProportionalLimit {
                value: helpers::optional_decimal(dry_row.get(1)),
            },
        },
    );

    Ok(compression)
}

fn parse_table_janka_hardness_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, JankaHardness>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_JANKA_HARDNESS_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table janka hardness not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties janka hardness green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties janka hardness dry row missing"))?;

    let mut janka_hardness = HashMap::new();
    janka_hardness.insert(
        MechanicalPropertiesCondition::Green,
        JankaHardness {
            parallel_to_grain: helpers::optional_decimal(green_row.first()),
            end_grain: helpers::optional_decimal(green_row.get(1)),
        },
    );

    janka_hardness.insert(
        MechanicalPropertiesCondition::Dry,
        JankaHardness {
            parallel_to_grain: helpers::optional_decimal(dry_row.first()),
            end_grain: helpers::optional_decimal(dry_row.get(1)),
        },
    );

    Ok(janka_hardness)
}

fn parse_table_tension_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, Tension>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_TENSION_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table tension not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties tension green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties tension dry row missing"))?;

    let mut tension = HashMap::new();
    tension.insert(
        MechanicalPropertiesCondition::Green,
        Tension {
            perpendicular_to_grain: PerpendicularOoGrain {
                maximum_compression: helpers::optional_decimal(green_row.first()),
            },
        },
    );

    tension.insert(
        MechanicalPropertiesCondition::Dry,
        Tension {
            perpendicular_to_grain: PerpendicularOoGrain {
                maximum_compression: helpers::optional_decimal(dry_row.first()),
            },
        },
    );

    Ok(tension)
}

fn parse_table_cleavage_strenght_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, CleavageStrength>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_CLEAVAGE_STRENGTH_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table cleavage strenght not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties cleavage strenght green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties cleavage strenght dry row missing"))?;

    let mut cleavage_strenght = HashMap::new();
    cleavage_strenght.insert(
        MechanicalPropertiesCondition::Green,
        CleavageStrength {
            maximum_compression: helpers::optional_decimal(green_row.first()),
        },
    );

    cleavage_strenght.insert(
        MechanicalPropertiesCondition::Dry,
        CleavageStrength {
            maximum_compression: helpers::optional_decimal(dry_row.first()),
        },
    );

    Ok(cleavage_strenght)
}

fn parse_table_shear_parallel_to_grain_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, ShearParallelToGrain>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_SHEAR_PARALLEL_TO_GRAIN_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table shear parallel to grain not found")?;

    let green_row = table_rows.first().ok_or_else(|| {
        anyhow!("mechanical properties shear parallel to grain green row missing")
    })?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties shear parallel to grain dry row missing"))?;

    let mut shear_parallel_to_grain = HashMap::new();
    shear_parallel_to_grain.insert(
        MechanicalPropertiesCondition::Green,
        ShearParallelToGrain {
            maximum_compression: helpers::optional_decimal(green_row.first()),
        },
    );

    shear_parallel_to_grain.insert(
        MechanicalPropertiesCondition::Dry,
        ShearParallelToGrain {
            maximum_compression: helpers::optional_decimal(dry_row.first()),
        },
    );

    Ok(shear_parallel_to_grain)
}

fn parse_table_nail_extraction_mechanical_properties(
    container: ElementRef<'_>,
) -> Result<HashMap<MechanicalPropertiesCondition, NailExtraction>> {
    let table_rows = selectors::table_rows_by_header(
        container,
        html::MECHANICAL_PROPERTIES_NAIL_EXTRACTION_TABLE_HEADER_COLUMN,
    )
    .with_context(|| "mechanical properties table nail extraction not found")?;

    let green_row = table_rows
        .first()
        .ok_or_else(|| anyhow!("mechanical properties nail extraction green row missing"))?;

    let dry_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("mechanical properties nail extraction dry row missing"))?;

    let mut nail_extraction = HashMap::new();
    nail_extraction.insert(
        MechanicalPropertiesCondition::Green,
        NailExtraction {
            parallel_to_grain: helpers::optional_decimal(green_row.first()),
            end_grain: helpers::optional_decimal(green_row.get(1)),
        },
    );

    nail_extraction.insert(
        MechanicalPropertiesCondition::Dry,
        NailExtraction {
            parallel_to_grain: helpers::optional_decimal(dry_row.first()),
            end_grain: helpers::optional_decimal(dry_row.get(1)),
        },
    );

    Ok(nail_extraction)
}

mod html {
    pub const MECHANICAL_PROPERTIES_STATIC_BENDING_TABLE_HEADER_COLUMN: &str =
        "flexão estática (kgf/cm²)";
    pub const MECHANICAL_PROPERTIES_COMPRESSION_HEADER_COLUMN: &str = "compressão (kgf/cm²)";
    pub const MECHANICAL_PROPERTIES_JANKA_HARDNESS_TABLE_HEADER_COLUMN: &str = "dureza janka (kgf)";
    pub const MECHANICAL_PROPERTIES_TENSION_TABLE_HEADER_COLUMN: &str = "tração (kgf/cm²)";
    pub const MECHANICAL_PROPERTIES_CLEAVAGE_STRENGTH_TABLE_HEADER_COLUMN: &str =
        "fendilhamento (kgf/cm)";
    pub const MECHANICAL_PROPERTIES_SHEAR_PARALLEL_TO_GRAIN_TABLE_HEADER_COLUMN: &str =
        "cisalhamento (kgf/cm²)";
    pub const MECHANICAL_PROPERTIES_NAIL_EXTRACTION_TABLE_HEADER_COLUMN: &str =
        "extração de pregos (kgf)";
}
