use crate::woods::value_objects::measurements::{GramsPerCubicCentimeter, Measurement, Percentage};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicalProperties {
    pub density: Density,
    pub shrinkage: Shrinkage,
}

/// Density at different moisture conditions.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Density {
    pub dry: Option<GramsPerCubicCentimeter>,
    pub green: Option<GramsPerCubicCentimeter>,
    pub basic: Option<GramsPerCubicCentimeter>,
    pub apparent: Option<GramsPerCubicCentimeter>,
}

/// Shrinkage from green to oven-dry condition.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Shrinkage {
    pub tangential: Option<Percentage>,
    pub radial: Option<Percentage>,
    pub volumetric: Option<Percentage>,
}

impl Shrinkage {
    /// Tangential/radial shrinkage ratio (anisotropy coefficient). The closer
    /// to 1, the more dimensionally stable the wood.
    pub fn tangential_radial_ratio(&self) -> Option<f64> {
        let tangential = self.tangential?.value();
        let radial = self.radial?.value();

        (radial > 0.0).then(|| tangential / radial)
    }
}
