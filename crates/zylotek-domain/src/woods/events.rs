use crate::woods::entities::WoodId;

#[derive(Debug, Clone)]
pub enum WoodEvent {
    WoodCreated { wood_id: WoodId },
}
