use std::{fmt, str::FromStr};

use uuid::Uuid;

use crate::woods::{
    errors::WoodError,
    events::WoodEvent,
    value_objects::{ScientificName, WoodDetails, WoodName},
};

/// Aggregate root: a Brazilian wood species and its technological properties.
#[derive(Debug, Clone)]
pub struct Wood {
    id: WoodId,
    name: WoodName,
    scientific_name: ScientificName,
    details: WoodDetails,
}

impl Wood {
    pub fn create(
        name: WoodName,
        scientific_name: ScientificName,
        details: WoodDetails,
    ) -> (Self, WoodEvent) {
        let wood = Self {
            id: WoodId::new(),
            name,
            scientific_name,
            details,
        };

        let event = WoodEvent::WoodCreated { wood_id: wood.id };

        (wood, event)
    }

    /// Rebuilds a wood from storage without emitting events.
    pub fn restore(
        id: WoodId,
        name: WoodName,
        scientific_name: ScientificName,
        details: WoodDetails,
    ) -> Self {
        Self {
            id,
            name,
            scientific_name,
            details,
        }
    }

    pub fn update(
        &mut self,
        name: WoodName,
        scientific_name: ScientificName,
        details: WoodDetails,
    ) -> WoodEvent {
        self.name = name;
        self.scientific_name = scientific_name;
        self.details = details;

        WoodEvent::WoodUpdated { wood_id: self.id }
    }

    pub fn delete(self) -> WoodEvent {
        WoodEvent::WoodDeleted { wood_id: self.id }
    }

    pub fn id(&self) -> WoodId {
        self.id
    }

    pub fn name(&self) -> &WoodName {
        &self.name
    }

    pub fn scientific_name(&self) -> &ScientificName {
        &self.scientific_name
    }

    pub fn details(&self) -> &WoodDetails {
        &self.details
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WoodId(Uuid);

impl WoodId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for WoodId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WoodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for WoodId {
    type Err = WoodError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| WoodError::InvalidId(value.to_owned()))
    }
}
