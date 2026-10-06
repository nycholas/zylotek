use uuid::Uuid;

use crate::woods::{errors::WoodError, events::WoodEvent};

#[derive(Debug, Clone)]
pub struct Wood {
    pub id: WoodId,
    pub name: String,
    // image_url: String,
    pub scientific_name: String,
    // family: String,
    // collection_location: String,
    // source_page_link: String,
    // common_names: Option<Vec<String>>,
    // tree: Option<HashMap<String, String>>,
    // general_characteristics: Option<HashMap<String, String>>,
    // end_uses: Option<String>,
    // klin_drying: Option<HashMap<String, String>>,
    // preservation: Option<String>,
    // workability: Option<String>,
    // klin_drying_days: Option<f64>,
    // heartwood_color_classification: Option<String>,
    // tree_image_url: Option<String>,
    // log_image_url: Option<String>,
    // bark_image_url: Option<String>,
    // tangencial_cut_image_url: Option<String>,
    // cross_section_image_url: Option<String>,
    // radial_cut_image_url: Option<String>,
    // wood_image_url: Option<String>,
    // workabilities: Workabilities,
    // physical_properties: PhysicalProperties,
    // mechanical_properties: MechanicalProperties,
}

impl Wood {
    pub fn create(name: String, scientific_name: String) -> Result<(Self, WoodEvent), WoodError> {
        let wood = Self {
            id: WoodId::new(),
            name,
            // image_url: todo!(),
            scientific_name,
            // family: todo!(),
            // collection_location: todo!(),
            // source_page_link: todo!(),
            // common_names: todo!(),
            // tree: todo!(),
            // general_characteristics: todo!(),
            // end_uses: todo!(),
            // klin_drying: todo!(),
            // preservation: todo!(),
            // workability: todo!(),
            // klin_drying_days: todo!(),
            // heartwood_color_classification: todo!(),
            // tree_image_url: todo!(),
            // log_image_url: todo!(),
            // bark_image_url: todo!(),
            // tangencial_cut_image_url: todo!(),
            // cross_section_image_url: todo!(),
            // radial_cut_image_url: todo!(),
            // wood_image_url: todo!(),
        };

        let event = WoodEvent::WoodCreated { wood_id: wood.id() };

        Ok((wood, event))
    }

    pub fn id(&self) -> WoodId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scientific_name(&self) -> &str {
        &self.scientific_name
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
