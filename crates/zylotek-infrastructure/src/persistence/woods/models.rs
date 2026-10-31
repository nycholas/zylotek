use std::collections::BTreeMap;

use toasty::{Embed, Json, Model};
use uuid::Uuid;

#[derive(Debug, Model)]
pub struct Wood {
    #[key]
    pub id: Uuid,

    pub name: String,

    #[unique]
    pub scientific_name: String,

    pub family: Option<String>,

    pub collection_location: Option<String>,

    #[column(type = text)]
    pub common_names: Json<Vec<String>>,

    pub source_species_id: Option<u64>,

    pub source_page_url: Option<String>,

    #[column(type = text)]
    pub tree: Json<BTreeMap<String, String>>,

    #[column(type = text)]
    pub general_characteristics: Json<BTreeMap<String, String>>,

    pub description: Description,

    #[column(type = text)]
    pub kiln_drying_schedule: Json<BTreeMap<String, String>>,

    pub kiln_drying_days: Option<f64>,

    pub images: Images,

    pub physical_properties: PhysicalProperties,

    pub mechanical_properties: MechanicalProperties,

    pub workabilities: Workabilities,
}

#[derive(Debug, Embed)]
pub struct Description {
    pub end_uses: Option<String>,
    pub preservation: Option<String>,
    pub workability_notes: Option<String>,
    pub heartwood_color_classification: Option<String>,
}

#[derive(Debug, Embed)]
pub struct Images {
    pub main: Option<String>,
    pub tree: Option<String>,
    pub log: Option<String>,
    pub bark: Option<String>,
    pub tangential_cut: Option<String>,
    pub cross_section: Option<String>,
    pub radial_cut: Option<String>,
    pub wood: Option<String>,
}

#[derive(Debug, Embed)]
pub struct PhysicalProperties {
    pub density: Density,
    pub shrinkage: Shrinkage,
}

#[derive(Debug, Embed)]
pub struct Density {
    pub dry: Option<f64>,
    pub green: Option<f64>,
    pub basic: Option<f64>,
    pub apparent: Option<f64>,
}

#[derive(Debug, Embed)]
pub struct Shrinkage {
    pub tangential: Option<f64>,
    pub radial: Option<f64>,
    pub volumetric: Option<f64>,
}

#[derive(Debug, Embed)]
pub struct MechanicalProperties {
    pub green: MechanicalPropertiesByCondition,
    pub dry: MechanicalPropertiesByCondition,
}

#[derive(Debug, Embed)]
pub struct MechanicalPropertiesByCondition {
    pub bending_modulus_of_rupture: Option<f64>,
    pub bending_modulus_of_elasticity: Option<f64>,
    pub compression_parallel_to_grain: Option<f64>,
    pub compression_perpendicular_to_grain: Option<f64>,
    pub janka_hardness_parallel_to_grain: Option<f64>,
    pub janka_hardness_end_grain: Option<f64>,
    pub tension_perpendicular_to_grain: Option<f64>,
    pub cleavage_strength: Option<f64>,
    pub shear_parallel_to_grain: Option<f64>,
    pub nail_extraction_parallel_to_grain: Option<f64>,
    pub nail_extraction_end_grain: Option<f64>,
}

#[derive(Debug, Embed)]
pub struct Workabilities {
    pub molding_planer: EquipmentFinishingResult,
    pub narrow_belt_sander: EquipmentFinishingResult,
    pub lathe: EquipmentFinishingResult,
    pub drill_bit: EquipmentFinishingResult,
    pub surface_finishing: SurfaceFinishing,
    pub heartwood_treatability: TreatabilityRating,
    pub sapwood_treatability: TreatabilityRating,
}

#[derive(Debug, Embed)]
pub struct EquipmentFinishingResult {
    pub number_of_samples: Option<u32>,
    pub surface_finish: WorkabilityRating,
    pub defect: WorkabilityDefect,
    pub equipment_settings: WorkabilityEquipmentSettings,
}

#[derive(Debug, Embed)]
pub struct SurfaceFinishing {
    pub planer: WorkabilityRating,
    pub sandpaper: WorkabilityRating,
    pub vise: WorkabilityRating,
    pub drill: WorkabilityRating,
    pub nail_crack: NailCrackBehavior,
}

#[derive(Debug, Clone, Copy, Embed)]
#[column(rename_all = "snake_case")]
pub enum WorkabilityRating {
    NotApplicable,
    VeryPoor,
    Poor,
    Fair,
    Easy,
    Good,
    VeryGood,
    Excellent,
}

#[derive(Debug, Clone, Copy, Embed)]
#[column(rename_all = "snake_case")]
pub enum WorkabilityDefect {
    NotApplicable,
    Flawless,
    Fair,
    Fluffy,
    Crushed,
    RippedOff,
    Lifted,
    Scratched,
    FluffyAndCrushed,
    FluffyAndRippedOff,
    FluffyAndScratched,
    FluffyAndRippedOffAndLifted,
    FluffyAndCrushedAndLifted,
    CrushedAndRippedOff,
    CrushedAndFluffyAndRippedOff,
    LiftedAndRippedOff,
}

#[derive(Debug, Clone, Copy, Embed)]
#[column(rename_all = "snake_case")]
pub enum WorkabilityEquipmentSettings {
    NotApplicable,
    NotSuitable,
    Suitable,
}

#[derive(Debug, Clone, Copy, Embed)]
#[column(rename_all = "snake_case")]
pub enum NailCrackBehavior {
    NotApplicable,
    DoesNotCrack,
    DoesNotCrackExceptCorners,
    Cracks,
}

#[derive(Debug, Clone, Copy, Embed)]
#[column(rename_all = "snake_case")]
pub enum TreatabilityRating {
    NotApplicable,
    Green,
    Dry,
    VeryHard,
    Hard,
    ModeratelyHard,
    ModeratelyEasy,
    Easy,
    VeryEasy,
}
