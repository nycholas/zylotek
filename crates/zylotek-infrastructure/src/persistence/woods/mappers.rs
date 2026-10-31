use std::collections::BTreeMap;

use toasty::Json;
use uuid::Uuid;
use zylotek_domain::woods::{
    entities as domain,
    errors::WoodError,
    value_objects::{self as vo, Measurement},
};

use crate::persistence::woods::models;

/// Column values of a domain wood, ready for Toasty's create and update builders.
pub(crate) struct WoodRecord {
    pub id: Uuid,
    pub name: String,
    pub scientific_name: String,
    pub family: Option<String>,
    pub collection_location: Option<String>,
    pub common_names: Json<Vec<String>>,
    pub source_species_id: Option<u64>,
    pub source_page_url: Option<String>,
    pub tree: Json<BTreeMap<String, String>>,
    pub general_characteristics: Json<BTreeMap<String, String>>,
    pub description: models::Description,
    pub kiln_drying_schedule: Json<BTreeMap<String, String>>,
    pub kiln_drying_days: Option<f64>,
    pub images: models::Images,
    pub physical_properties: models::PhysicalProperties,
    pub mechanical_properties: models::MechanicalProperties,
    pub workabilities: models::Workabilities,
}

impl From<&domain::Wood> for WoodRecord {
    fn from(wood: &domain::Wood) -> Self {
        let details = wood.details();
        let description = &details.description;
        let images = &details.images;

        Self {
            id: wood.id().as_uuid(),
            name: wood.name().to_string(),
            scientific_name: wood.scientific_name().to_string(),
            family: details.family.as_ref().map(ToString::to_string),
            collection_location: details.collection_location.clone(),
            common_names: Json(details.common_names.as_slice().to_vec()),
            source_species_id: details.source.as_ref().map(|source| source.species_id),
            source_page_url: details
                .source
                .as_ref()
                .map(|source| source.page_url.to_string()),
            tree: attributes(&description.tree),
            general_characteristics: attributes(&description.general_characteristics),
            description: models::Description {
                end_uses: description.end_uses.clone(),
                preservation: description.preservation.clone(),
                workability_notes: description.workability_notes.clone(),
                heartwood_color_classification: description.heartwood_color_classification.clone(),
            },
            kiln_drying_schedule: attributes(&details.kiln_drying.schedule),
            kiln_drying_days: value(details.kiln_drying.duration),
            images: models::Images {
                main: url(&images.main),
                tree: url(&images.tree),
                log: url(&images.log),
                bark: url(&images.bark),
                tangential_cut: url(&images.tangential_cut),
                cross_section: url(&images.cross_section),
                radial_cut: url(&images.radial_cut),
                wood: url(&images.wood),
            },
            physical_properties: (&details.physical_properties).into(),
            mechanical_properties: models::MechanicalProperties {
                green: (&details.mechanical_properties.green).into(),
                dry: (&details.mechanical_properties.dry).into(),
            },
            workabilities: (&details.workabilities).into(),
        }
    }
}

impl TryFrom<models::Wood> for domain::Wood {
    type Error = WoodError;

    fn try_from(model: models::Wood) -> Result<Self, Self::Error> {
        let source = match (model.source_species_id, model.source_page_url) {
            (Some(species_id), Some(page_url)) => Some(vo::SourceReference {
                species_id,
                page_url: vo::WebUrl::new(page_url)?,
            }),
            _ => None,
        };

        let details = vo::WoodDetails {
            family: model.family.map(vo::BotanicalFamily::new).transpose()?,
            collection_location: model.collection_location,
            common_names: vo::CommonNames::new(model.common_names.0),
            source,
            description: vo::WoodDescription {
                tree: vo::Attributes::new(model.tree.0),
                general_characteristics: vo::Attributes::new(model.general_characteristics.0),
                end_uses: model.description.end_uses,
                preservation: model.description.preservation,
                workability_notes: model.description.workability_notes,
                heartwood_color_classification: model.description.heartwood_color_classification,
            },
            kiln_drying: vo::KilnDrying {
                schedule: vo::Attributes::new(model.kiln_drying_schedule.0),
                duration: vo::Days::from_optional(model.kiln_drying_days)?,
            },
            images: vo::WoodImages {
                main: optional_url(model.images.main)?,
                tree: optional_url(model.images.tree)?,
                log: optional_url(model.images.log)?,
                bark: optional_url(model.images.bark)?,
                tangential_cut: optional_url(model.images.tangential_cut)?,
                cross_section: optional_url(model.images.cross_section)?,
                radial_cut: optional_url(model.images.radial_cut)?,
                wood: optional_url(model.images.wood)?,
            },
            physical_properties: model.physical_properties.try_into()?,
            mechanical_properties: vo::MechanicalProperties {
                green: model.mechanical_properties.green.try_into()?,
                dry: model.mechanical_properties.dry.try_into()?,
            },
            workabilities: model.workabilities.into(),
        };

        Ok(domain::Wood::restore(
            domain::WoodId::from_uuid(model.id),
            vo::WoodName::new(model.name)?,
            vo::ScientificName::new(model.scientific_name)?,
            details,
        ))
    }
}

impl From<&vo::PhysicalProperties> for models::PhysicalProperties {
    fn from(properties: &vo::PhysicalProperties) -> Self {
        let density = &properties.density;
        let shrinkage = &properties.shrinkage;

        Self {
            density: models::Density {
                dry: value(density.dry),
                green: value(density.green),
                basic: value(density.basic),
                apparent: value(density.apparent),
            },
            shrinkage: models::Shrinkage {
                tangential: value(shrinkage.tangential),
                radial: value(shrinkage.radial),
                volumetric: value(shrinkage.volumetric),
            },
        }
    }
}

impl TryFrom<models::PhysicalProperties> for vo::PhysicalProperties {
    type Error = WoodError;

    fn try_from(model: models::PhysicalProperties) -> Result<Self, Self::Error> {
        Ok(Self {
            density: vo::Density {
                dry: Measurement::from_optional(model.density.dry)?,
                green: Measurement::from_optional(model.density.green)?,
                basic: Measurement::from_optional(model.density.basic)?,
                apparent: Measurement::from_optional(model.density.apparent)?,
            },
            shrinkage: vo::Shrinkage {
                tangential: Measurement::from_optional(model.shrinkage.tangential)?,
                radial: Measurement::from_optional(model.shrinkage.radial)?,
                volumetric: Measurement::from_optional(model.shrinkage.volumetric)?,
            },
        })
    }
}

impl From<&vo::MechanicalPropertiesByCondition> for models::MechanicalPropertiesByCondition {
    fn from(properties: &vo::MechanicalPropertiesByCondition) -> Self {
        Self {
            bending_modulus_of_rupture: value(properties.static_bending.modulus_of_rupture),
            bending_modulus_of_elasticity: value(properties.static_bending.modulus_of_elasticity),
            compression_parallel_to_grain: value(properties.compression.parallel_to_grain),
            compression_perpendicular_to_grain: value(
                properties.compression.perpendicular_to_grain,
            ),
            janka_hardness_parallel_to_grain: value(properties.janka_hardness.parallel_to_grain),
            janka_hardness_end_grain: value(properties.janka_hardness.end_grain),
            tension_perpendicular_to_grain: value(properties.tension_perpendicular_to_grain),
            cleavage_strength: value(properties.cleavage_strength),
            shear_parallel_to_grain: value(properties.shear_parallel_to_grain),
            nail_extraction_parallel_to_grain: value(properties.nail_extraction.parallel_to_grain),
            nail_extraction_end_grain: value(properties.nail_extraction.end_grain),
        }
    }
}

impl TryFrom<models::MechanicalPropertiesByCondition> for vo::MechanicalPropertiesByCondition {
    type Error = WoodError;

    fn try_from(model: models::MechanicalPropertiesByCondition) -> Result<Self, Self::Error> {
        Ok(Self {
            static_bending: vo::StaticBending {
                modulus_of_rupture: Measurement::from_optional(model.bending_modulus_of_rupture)?,
                modulus_of_elasticity: Measurement::from_optional(
                    model.bending_modulus_of_elasticity,
                )?,
            },
            compression: vo::Compression {
                parallel_to_grain: Measurement::from_optional(model.compression_parallel_to_grain)?,
                perpendicular_to_grain: Measurement::from_optional(
                    model.compression_perpendicular_to_grain,
                )?,
            },
            janka_hardness: vo::JankaHardness {
                parallel_to_grain: Measurement::from_optional(
                    model.janka_hardness_parallel_to_grain,
                )?,
                end_grain: Measurement::from_optional(model.janka_hardness_end_grain)?,
            },
            tension_perpendicular_to_grain: Measurement::from_optional(
                model.tension_perpendicular_to_grain,
            )?,
            cleavage_strength: Measurement::from_optional(model.cleavage_strength)?,
            shear_parallel_to_grain: Measurement::from_optional(model.shear_parallel_to_grain)?,
            nail_extraction: vo::NailExtraction {
                parallel_to_grain: Measurement::from_optional(
                    model.nail_extraction_parallel_to_grain,
                )?,
                end_grain: Measurement::from_optional(model.nail_extraction_end_grain)?,
            },
        })
    }
}

impl From<&vo::Workabilities> for models::Workabilities {
    fn from(workabilities: &vo::Workabilities) -> Self {
        let equipment = &workabilities.equipment_finishing;
        let surface = &workabilities.surface_finishing;

        Self {
            molding_planer: (&equipment.molding_planer).into(),
            narrow_belt_sander: (&equipment.narrow_belt_sander).into(),
            lathe: (&equipment.lathe).into(),
            drill_bit: (&equipment.drill_bit).into(),
            surface_finishing: models::SurfaceFinishing {
                planer: surface.planer.into(),
                sandpaper: surface.sandpaper.into(),
                vise: surface.vise.into(),
                drill: surface.drill.into(),
                nail_crack: surface.nail_crack.into(),
            },
            heartwood_treatability: workabilities.treatabilities.heartwood.into(),
            sapwood_treatability: workabilities.treatabilities.sapwood.into(),
        }
    }
}

impl From<models::Workabilities> for vo::Workabilities {
    fn from(model: models::Workabilities) -> Self {
        let surface = model.surface_finishing;

        Self {
            equipment_finishing: vo::EquipmentFinishing {
                molding_planer: model.molding_planer.into(),
                narrow_belt_sander: model.narrow_belt_sander.into(),
                lathe: model.lathe.into(),
                drill_bit: model.drill_bit.into(),
            },
            surface_finishing: vo::SurfaceFinishing {
                planer: surface.planer.into(),
                sandpaper: surface.sandpaper.into(),
                vise: surface.vise.into(),
                drill: surface.drill.into(),
                nail_crack: surface.nail_crack.into(),
            },
            treatabilities: vo::Treatabilities {
                heartwood: model.heartwood_treatability.into(),
                sapwood: model.sapwood_treatability.into(),
            },
        }
    }
}

impl From<&vo::EquipmentFinishingResult> for models::EquipmentFinishingResult {
    fn from(result: &vo::EquipmentFinishingResult) -> Self {
        Self {
            number_of_samples: result.number_of_samples,
            surface_finish: result.surface_finish.into(),
            defect: result.defect.into(),
            equipment_settings: result.equipment_settings.into(),
        }
    }
}

impl From<models::EquipmentFinishingResult> for vo::EquipmentFinishingResult {
    fn from(model: models::EquipmentFinishingResult) -> Self {
        Self {
            number_of_samples: model.number_of_samples,
            surface_finish: model.surface_finish.into(),
            defect: model.defect.into(),
            equipment_settings: model.equipment_settings.into(),
        }
    }
}

/// Maps a stored enum to its domain twin and back, variant by variant.
macro_rules! enum_mapping {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        impl From<vo::$name> for models::$name {
            fn from(value: vo::$name) -> Self {
                match value {
                    $(vo::$name::$variant => Self::$variant,)*
                }
            }
        }

        impl From<models::$name> for vo::$name {
            fn from(value: models::$name) -> Self {
                match value {
                    $(models::$name::$variant => Self::$variant,)*
                }
            }
        }
    };
}

enum_mapping!(WorkabilityRating {
    NotApplicable,
    VeryPoor,
    Poor,
    Fair,
    Easy,
    Good,
    VeryGood,
    Excellent,
});

enum_mapping!(WorkabilityDefect {
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
});

enum_mapping!(WorkabilityEquipmentSettings {
    NotApplicable,
    NotSuitable,
    Suitable,
});

enum_mapping!(NailCrackBehavior {
    NotApplicable,
    DoesNotCrack,
    DoesNotCrackExceptCorners,
    Cracks,
});

enum_mapping!(TreatabilityRating {
    NotApplicable,
    Green,
    Dry,
    VeryHard,
    Hard,
    ModeratelyHard,
    ModeratelyEasy,
    Easy,
    VeryEasy,
});

fn value<M: Measurement>(measurement: Option<M>) -> Option<f64> {
    measurement.map(|measurement| measurement.value())
}

fn url(url: &Option<vo::WebUrl>) -> Option<String> {
    url.as_ref().map(ToString::to_string)
}

fn optional_url(url: Option<String>) -> Result<Option<vo::WebUrl>, WoodError> {
    url.map(vo::WebUrl::new).transpose()
}

fn attributes(attributes: &vo::Attributes) -> Json<BTreeMap<String, String>> {
    Json(
        attributes
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect(),
    )
}
