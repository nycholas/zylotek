use std::fmt;

use crate::woods::errors::WoodError;

/// A non-negative, finite physical quantity expressed in a fixed unit.
pub trait Measurement: Sized + Copy {
    const UNIT: &'static str;

    fn new(value: f64) -> Result<Self, WoodError>;

    fn value(&self) -> f64;

    fn from_optional(value: Option<f64>) -> Result<Option<Self>, WoodError> {
        value.map(Self::new).transpose()
    }
}

macro_rules! measurement {
    ($(#[$meta:meta])* $name:ident, $unit:literal, max = $max:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl Measurement for $name {
            const UNIT: &'static str = $unit;

            fn new(value: f64) -> Result<Self, WoodError> {
                // NaN and infinities fall outside the range too.
                if !(0.0..=$max).contains(&value) {
                    return Err(WoodError::InvalidMeasurement {
                        unit: Self::UNIT,
                        value,
                    });
                }

                Ok(Self(value))
            }

            fn value(&self) -> f64 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.0, Self::UNIT)
            }
        }
    };
    ($(#[$meta:meta])* $name:ident, $unit:literal) => {
        measurement!($(#[$meta])* $name, $unit, max = f64::MAX);
    };
}

measurement!(
    /// Density, in g/cm³.
    GramsPerCubicCentimeter,
    "g/cm³"
);

measurement!(
    /// A percentage between 0 and 100.
    Percentage,
    "%",
    max = 100.0
);

measurement!(
    /// Stress (bending, compression, tension, shear), in kgf/cm².
    KgfPerSquareCentimeter,
    "kgf/cm²"
);

measurement!(
    /// Cleavage strength, in kgf/cm.
    KgfPerCentimeter,
    "kgf/cm"
);

measurement!(
    /// Force (Janka hardness, nail extraction), in kgf.
    Kgf,
    "kgf"
);

measurement!(
    /// A duration in days.
    Days,
    "days"
);
