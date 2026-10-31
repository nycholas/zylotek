use crate::woods::entities::WoodId;

#[derive(Debug, Clone)]
pub enum WoodEvent {
    WoodCreated { wood_id: WoodId },
    WoodUpdated { wood_id: WoodId },
    WoodDeleted { wood_id: WoodId },
}
