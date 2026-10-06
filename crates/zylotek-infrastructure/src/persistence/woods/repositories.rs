use zylotek_domain::{
    errors::RepositoryError,
    woods::{entities as domain, repositories::WoodRepository},
};

use crate::persistence::woods::models::Wood;

pub struct PostgresWoodRepository {
    db: toasty::Db,
}

impl PostgresWoodRepository {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl WoodRepository for PostgresWoodRepository {
    async fn exists_by_scientific_name(
        &self,
        scientific_name: &str,
    ) -> Result<bool, RepositoryError> {
        let mut db = self.db.clone();

        let exists: Option<Wood> =
            Wood::filter(Wood::fields().scientific_name().eq(scientific_name))
                .first()
                .exec(&mut db)
                .await
                .map_err(|_e| RepositoryError::Database)?;

        Ok(exists.is_some())
    }

    async fn save(&self, wood: &domain::Wood) -> Result<domain::Wood, RepositoryError> {
        let mut db = self.db.clone();

        let model = toasty::create!(Wood {
            id: wood.id().as_uuid(),
            name: wood.name(),
            scientific_name: wood.scientific_name()
        })
        .exec(&mut db)
        .await
        .map_err(|_err| RepositoryError::Database)?;

        Ok(model.into())
    }
}
