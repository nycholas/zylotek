use std::{collections::HashMap, vec};

use anyhow::{Result, anyhow};
use scraper::ElementRef;

use crate::parsers::{helpers, selectors};

#[derive(Debug, Clone)]
pub struct Workabilities {
    pub equipment_finishing: EquipmentFinishing,
    pub surface_finishing: SurfaceFinishing,
    pub treatabilities: Treatabilities,
}

impl Workabilities {
    pub(crate) fn parse(container: ElementRef<'_>) -> Result<Workabilities> {
        Ok(Workabilities {
            equipment_finishing: parse_table_equipment_finishing(container)?,
            surface_finishing: parse_table_type_of_action(container)?,
            treatabilities: parse_table_treatability_of_wood(container)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceFinishing {
    pub planer: WorkabilityRating,
    pub sandpaper: WorkabilityRating,
    pub vise: WorkabilityRating,
    pub drill: WorkabilityRating,
    pub nail_crack: NailCrackBehavior,
}

#[derive(Debug, Clone)]
pub struct EquipmentFinishing {
    pub molding_planer: EquipmentFinishingByResults,
    pub narrow_belt_sander: EquipmentFinishingByResults,
    pub lathe: EquipmentFinishingByResults,
    pub drill_bit: EquipmentFinishingByResults,
}

#[derive(Debug, Clone)]
pub struct EquipmentFinishingByResults {
    pub number_of_samples: Option<i64>,
    pub surface_finish: WorkabilityRating,
    pub defect: WorkabilityDefect,
    pub equipment_settings: WorkabilityEquipmentSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkabilityRating {
    NotApplicable,
    Poor,
    VeryPoor,
    Fair,
    Easy,
    Good,
    VeryGood,
    Excellent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkabilityDefect {
    NotApplicable,
    Fair,
    FluffyAndCrushed,
    FluffyAndRippedOff,
    FluffyAndScratched,
    FluffyAndRippedOffAndLifted,
    FluffyAndCrushedAndLifted,
    Fluffy,
    CrushedAndRippedOff,
    CrushedAndFluffyAndRippedOff,
    Crushed,
    RippedOff,
    LiftedAndRippedOff,
    Lifted,
    Scratched,
    Flawless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkabilityEquipmentSettings {
    NotApplicable,
    NotSuitable,
    Suitable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NailCrackBehavior {
    NotApplicable,
    DoesNotCrack,
    DoesNotCrackExceptCorners,
    Cracks,
}

#[derive(Debug, Clone)]
pub struct Treatabilities {
    pub heartwood: TreatabilityRating,
    pub spawood: TreatabilityRating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreatabilityRating {
    NotApplicable,
    Green,
    Dry,
    Hard,
    ModeratelyHard,
    VeryHard,
    Easy,
    ModeratelyEasy,
    VeryEasy,
}

fn parse_workability_defect(value: &str) -> Result<WorkabilityDefect> {
    match helpers::normalize(value).to_lowercase().as_str() {
        "-" | "" => Ok(WorkabilityDefect::NotApplicable),
        "arrancada, esmagada e felpuda" => Ok(WorkabilityDefect::CrushedAndFluffyAndRippedOff),
        "arrancada e esmagada" => Ok(WorkabilityDefect::CrushedAndRippedOff),
        "esmagada" => Ok(WorkabilityDefect::Crushed),
        "felpuda e arrancada" | "arrancada e felpuda" => Ok(WorkabilityDefect::FluffyAndRippedOff),
        "felpuda e riscada" | "riscada e felpuda" => Ok(WorkabilityDefect::FluffyAndScratched),
        "felpuda, arrancada e levantada" => Ok(WorkabilityDefect::FluffyAndRippedOffAndLifted),
        "felpuda esmagada levantada" => Ok(WorkabilityDefect::FluffyAndCrushedAndLifted),
        "felpuda e esmagada" => Ok(WorkabilityDefect::FluffyAndCrushed),
        "felpuda" => Ok(WorkabilityDefect::Fluffy),
        "arrancada" => Ok(WorkabilityDefect::RippedOff),
        "levantada e arrancada" => Ok(WorkabilityDefect::LiftedAndRippedOff),
        "levantada" => Ok(WorkabilityDefect::Lifted),
        "riscada" => Ok(WorkabilityDefect::Scratched),
        "aceitável" | "aceitavel" => Ok(WorkabilityDefect::Fair),
        "sem defeito" => Ok(WorkabilityDefect::Flawless),
        other => Err(anyhow!("unknown workability defect value: {other:?}")),
    }
}

fn parse_workability_equipment(value: &str) -> Result<WorkabilityEquipmentSettings> {
    match helpers::normalize(value).to_lowercase().as_str() {
        "-" | "" => Ok(WorkabilityEquipmentSettings::NotApplicable),
        "não adequada" => Ok(WorkabilityEquipmentSettings::NotSuitable),
        "adequada" => Ok(WorkabilityEquipmentSettings::Suitable),
        other => Err(anyhow!(
            "unknown workability equipment settings value: {other:?}"
        )),
    }
}

fn parse_workability_rating(value: &str) -> Result<WorkabilityRating> {
    match helpers::normalize(value).to_lowercase().as_str() {
        "-" | "" => Ok(WorkabilityRating::NotApplicable),
        "ruim" => Ok(WorkabilityRating::Poor),
        "muito ruim" => Ok(WorkabilityRating::VeryPoor),
        "regular" | "aceitável" => Ok(WorkabilityRating::Fair),
        "fácil" => Ok(WorkabilityRating::Easy),
        "bom" => Ok(WorkabilityRating::Good),
        "muito bom" => Ok(WorkabilityRating::VeryGood),
        "excelente" => Ok(WorkabilityRating::Excellent),
        other => Err(anyhow!("unknown workability rating value: {other:?}")),
    }
}

fn parse_nail_crack(value: &str) -> Result<NailCrackBehavior> {
    match helpers::normalize(value).to_lowercase().as_str() {
        "-" | "" => Ok(NailCrackBehavior::NotApplicable),
        "não trinca" => Ok(NailCrackBehavior::DoesNotCrack),
        "não trinca, exceto cantos" => Ok(NailCrackBehavior::DoesNotCrackExceptCorners),
        "trinca" => Ok(NailCrackBehavior::Cracks),
        other => Err(anyhow!("unknown nail-crack value: {other:?}")),
    }
}

fn parse_treatability(value: &str) -> Result<TreatabilityRating> {
    match helpers::normalize(value).to_lowercase().as_str() {
        "-" | "" => Ok(TreatabilityRating::NotApplicable),
        "verde" => Ok(TreatabilityRating::Dry),
        "seca" => Ok(TreatabilityRating::Green),
        "muito difícil" => Ok(TreatabilityRating::VeryHard),
        "moderadamente difícil" => Ok(TreatabilityRating::ModeratelyHard),
        "difícil" => Ok(TreatabilityRating::Hard),
        "fácil" => Ok(TreatabilityRating::Easy),
        "moderadamente fácil" => Ok(TreatabilityRating::ModeratelyEasy),
        "muito fácil" => Ok(TreatabilityRating::VeryEasy),
        other => Err(anyhow!("unknown treatability value: {other:?}")),
    }
}

fn parse_table_equipment_finishing(container: ElementRef<'_>) -> Result<EquipmentFinishing> {
    let table_selector = selectors::query("table")?;
    let rows_selector = selectors::query(r#"table > tbody > tr"#)?;
    let td_selector = selectors::query("td")?;

    let table_equipment_finishing_header_rows = [
        html::EQUIPMENT_FINISH_NUMBER_OF_SAMPLES_TABLE_HEADER_ROW,
        html::EQUIPMENT_FINISH_SURFICE_FINISHING_TABLE_HEADER_ROW,
        html::EQUIPMENT_FINISH_DEFECT_TABLE_HEADER_ROW,
        html::EQUIPMENT_FINISH_EQUIPMENT_SETTINGS_TABLE_HEADER_ROW,
    ];

    let mut table_rows = HashMap::new();
    let first_table = container.select(&table_selector).next().unwrap();
    for row_element in first_table.select(&rows_selector) {
        let mut cell_elements = row_element.select(&td_selector);
        if let Some(cell_element) = cell_elements.next() {
            let cell_text = cell_element
                .text()
                .collect::<String>()
                .trim()
                .to_lowercase();
            let attr_name = cell_text.clone();
            if table_equipment_finishing_header_rows.contains(&attr_name.as_str()) {
                let cells: Vec<String> = cell_elements.map(selectors::text).collect();
                table_rows.insert(attr_name, cells);
                continue;
            }
        }
    }

    let number_of_samples_row = table_rows
        .remove(html::EQUIPMENT_FINISH_NUMBER_OF_SAMPLES_TABLE_HEADER_ROW)
        .unwrap_or(vec!["".to_string(); 4]);

    let surface_finishing_row = table_rows
        .remove(html::EQUIPMENT_FINISH_SURFICE_FINISHING_TABLE_HEADER_ROW)
        .unwrap_or(vec!["".to_string(); 4]);

    let defect_row = table_rows
        .remove(html::EQUIPMENT_FINISH_DEFECT_TABLE_HEADER_ROW)
        .unwrap_or(vec!["".to_string(); 4]);

    let equipment_settings_row = table_rows
        .remove(html::EQUIPMENT_FINISH_EQUIPMENT_SETTINGS_TABLE_HEADER_ROW)
        .unwrap_or(vec!["".to_string(); 4]);

    Ok(EquipmentFinishing {
        molding_planer: EquipmentFinishingByResults {
            number_of_samples: helpers::optional_number(number_of_samples_row.first()),
            surface_finish: parse_workability_rating(
                surface_finishing_row.first().unwrap_or(&String::new()),
            )?,
            defect: parse_workability_defect(defect_row.first().unwrap_or(&String::new()))?,
            equipment_settings: parse_workability_equipment(
                equipment_settings_row.first().unwrap_or(&String::new()),
            )?,
        },
        narrow_belt_sander: EquipmentFinishingByResults {
            number_of_samples: helpers::optional_number(number_of_samples_row.get(1)),
            surface_finish: parse_workability_rating(
                surface_finishing_row.get(1).unwrap_or(&String::new()),
            )?,
            defect: parse_workability_defect(defect_row.get(1).unwrap_or(&String::new()))?,
            equipment_settings: parse_workability_equipment(
                equipment_settings_row.get(1).unwrap_or(&String::new()),
            )?,
        },
        lathe: EquipmentFinishingByResults {
            number_of_samples: helpers::optional_number(number_of_samples_row.get(2)),
            surface_finish: parse_workability_rating(
                surface_finishing_row.get(2).unwrap_or(&String::new()),
            )?,
            defect: parse_workability_defect(defect_row.get(2).unwrap_or(&String::new()))?,
            equipment_settings: parse_workability_equipment(
                equipment_settings_row.get(2).unwrap_or(&String::new()),
            )?,
        },
        drill_bit: EquipmentFinishingByResults {
            number_of_samples: helpers::optional_number(number_of_samples_row.get(3)),
            surface_finish: parse_workability_rating(
                surface_finishing_row.get(3).unwrap_or(&String::new()),
            )?,
            defect: parse_workability_defect(defect_row.get(3).unwrap_or(&String::new()))?,
            equipment_settings: parse_workability_equipment(
                equipment_settings_row.get(3).unwrap_or(&String::new()),
            )?,
        },
    })
}

fn parse_table_type_of_action(container: ElementRef<'_>) -> Result<SurfaceFinishing> {
    let table_rows =
        selectors::table_rows_by_heading(container, html::TYPE_OF_ACTION_TABLE_HEADING)
            .unwrap_or(vec![vec![String::new(); 6]; 2]);

    let surface_finishing_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("workability - type of action surface finishing row missing"))?;

    Ok(SurfaceFinishing {
        planer: parse_workability_rating(&surface_finishing_row[1])?,
        sandpaper: parse_workability_rating(&surface_finishing_row[2])?,
        vise: parse_workability_rating(&surface_finishing_row[3])?,
        drill: parse_workability_rating(&surface_finishing_row[4])?,
        nail_crack: parse_nail_crack(&surface_finishing_row[5])?,
    })
}

fn parse_table_treatability_of_wood(container: ElementRef<'_>) -> Result<Treatabilities> {
    let table_rows =
        selectors::table_rows_by_heading(container, html::TREATABILITY_OF_WOOD_TABLE_HEADING)
            .unwrap_or(vec![vec![String::new(); 2]; 2]);

    let treatability_of_wood_row = table_rows
        .get(1)
        .ok_or_else(|| anyhow!("workability - treatability of wood row missing"))?;

    Ok(Treatabilities {
        heartwood: parse_treatability(&treatability_of_wood_row[0])?,
        spawood: parse_treatability(&treatability_of_wood_row[1])?,
    })
}

mod html {
    // table equipment finish
    pub const EQUIPMENT_FINISH_NUMBER_OF_SAMPLES_TABLE_HEADER_ROW: &str = "número de amostras";
    pub const EQUIPMENT_FINISH_SURFICE_FINISHING_TABLE_HEADER_ROW: &str = "acabamento superficial";
    pub const EQUIPMENT_FINISH_DEFECT_TABLE_HEADER_ROW: &str = "defeito";
    pub const EQUIPMENT_FINISH_EQUIPMENT_SETTINGS_TABLE_HEADER_ROW: &str =
        "regulagem de equipamento";

    // table type of action
    pub const TYPE_OF_ACTION_TABLE_HEADING: &str = "trabalhabilidade - tipo de ação";

    // table treatability of wood
    pub const TREATABILITY_OF_WOOD_TABLE_HEADING: &str = "tratabilidade da madeira";
}
