use std::fmt;

use crate::woods::errors::WoodError;

fn normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

macro_rules! text_value_object {
    ($(#[$meta:meta])* $name:ident, $field:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }

            fn non_empty(value: &str) -> Result<String, WoodError> {
                let value = normalize(value);
                if value.is_empty() {
                    return Err(WoodError::EmptyText { field: $field });
                }

                Ok(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

text_value_object!(
    /// Popular (trade) name of a wood, e.g. "Muiracatiara-rajada".
    WoodName,
    "name"
);

text_value_object!(
    /// Binomial name of the species, e.g. "Astronium lecointei".
    ScientificName,
    "scientific name"
);

text_value_object!(
    /// Botanical family, e.g. "Anacardiaceae".
    BotanicalFamily,
    "family"
);

impl WoodName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, WoodError> {
        Self::non_empty(value.as_ref()).map(Self)
    }
}

impl ScientificName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, WoodError> {
        let value = Self::non_empty(value.as_ref())?;

        // A species name has at least a genus and a specific epithet ("sp." included).
        if value.split(' ').count() < 2 {
            return Err(WoodError::InvalidScientificName(value));
        }

        Ok(Self(value))
    }

    pub fn genus(&self) -> &str {
        self.0.split(' ').next().unwrap_or_default()
    }
}

impl BotanicalFamily {
    pub fn new(value: impl AsRef<str>) -> Result<Self, WoodError> {
        Self::non_empty(value.as_ref()).map(Self)
    }
}

/// Regional common names of a wood, trimmed and without duplicates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommonNames(Vec<String>);

impl CommonNames {
    pub fn new<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut values: Vec<String> = Vec::new();
        for name in names {
            let name = normalize(name.as_ref());
            if name.is_empty() || values.iter().any(|v| v.eq_ignore_ascii_case(&name)) {
                continue;
            }
            values.push(name);
        }

        Self(values)
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
