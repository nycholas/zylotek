use crate::woods::value_objects::measurements::{Kgf, KgfPerCentimeter, KgfPerSquareCentimeter};

/// Moisture condition of the specimens a mechanical test was run on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoistureCondition {
    Green,
    Dry,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MechanicalProperties {
    pub green: MechanicalPropertiesByCondition,
    pub dry: MechanicalPropertiesByCondition,
}

impl MechanicalProperties {
    pub fn by_condition(&self, condition: MoistureCondition) -> &MechanicalPropertiesByCondition {
        match condition {
            MoistureCondition::Green => &self.green,
            MoistureCondition::Dry => &self.dry,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MechanicalPropertiesByCondition {
    pub static_bending: StaticBending,
    pub compression: Compression,
    pub janka_hardness: JankaHardness,
    pub tension_perpendicular_to_grain: Option<KgfPerSquareCentimeter>,
    pub cleavage_strength: Option<KgfPerCentimeter>,
    pub shear_parallel_to_grain: Option<KgfPerSquareCentimeter>,
    pub nail_extraction: NailExtraction,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StaticBending {
    pub modulus_of_rupture: Option<KgfPerSquareCentimeter>,
    pub modulus_of_elasticity: Option<KgfPerSquareCentimeter>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Compression {
    /// Maximum crushing strength parallel to grain.
    pub parallel_to_grain: Option<KgfPerSquareCentimeter>,
    /// Stress at proportional limit perpendicular to grain.
    pub perpendicular_to_grain: Option<KgfPerSquareCentimeter>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JankaHardness {
    pub parallel_to_grain: Option<Kgf>,
    pub end_grain: Option<Kgf>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NailExtraction {
    pub parallel_to_grain: Option<Kgf>,
    pub end_grain: Option<Kgf>,
}
