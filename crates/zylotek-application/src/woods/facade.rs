use std::sync::Arc;

use zylotek_domain::woods::entities::{Wood, WoodId};

use crate::woods::{
    create_wood::{CreateWood, CreateWoodCommand},
    delete_wood::DeleteWood,
    errors::WoodUseCaseError,
    get_wood::GetWood,
    list_woods::{ListWoods, ListWoodsQuery, WoodPage},
    update_wood::{UpdateWood, UpdateWoodCommand},
};

#[derive(Clone)]
pub struct WoodFacade {
    create_wood: Arc<CreateWood>,
    get_wood: Arc<GetWood>,
    list_woods: Arc<ListWoods>,
    update_wood: Arc<UpdateWood>,
    delete_wood: Arc<DeleteWood>,
}

impl WoodFacade {
    pub fn new(
        create_wood: CreateWood,
        get_wood: GetWood,
        list_woods: ListWoods,
        update_wood: UpdateWood,
        delete_wood: DeleteWood,
    ) -> Self {
        Self {
            create_wood: Arc::new(create_wood),
            get_wood: Arc::new(get_wood),
            list_woods: Arc::new(list_woods),
            update_wood: Arc::new(update_wood),
            delete_wood: Arc::new(delete_wood),
        }
    }

    pub async fn create_wood(&self, command: CreateWoodCommand) -> Result<Wood, WoodUseCaseError> {
        self.create_wood.execute(command).await
    }

    pub async fn get_wood(&self, id: WoodId) -> Result<Wood, WoodUseCaseError> {
        self.get_wood.execute(id).await
    }

    pub async fn list_woods(&self, query: ListWoodsQuery) -> Result<WoodPage, WoodUseCaseError> {
        self.list_woods.execute(query).await
    }

    pub async fn update_wood(&self, command: UpdateWoodCommand) -> Result<Wood, WoodUseCaseError> {
        self.update_wood.execute(command).await
    }

    pub async fn delete_wood(&self, id: WoodId) -> Result<(), WoodUseCaseError> {
        self.delete_wood.execute(id).await
    }
}
