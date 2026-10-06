use std::sync::Arc;

use zylotek_application::woods::facade::WoodFacade;

#[derive(Clone)]
pub struct AppState {
    pub woods: Arc<WoodFacade>,
}
