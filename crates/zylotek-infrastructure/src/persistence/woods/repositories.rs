use zylotek_domain::{
    errors::RepositoryError,
    woods::{entities as domain, repositories::WoodRepository},
};

use crate::persistence::woods::{mappers::WoodRecord, models::Wood};

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
                .map_err(database_error)?;

        Ok(exists.is_some())
    }

    async fn find_by_id(
        &self,
        id: domain::WoodId,
    ) -> Result<Option<domain::Wood>, RepositoryError> {
        let mut db = self.db.clone();

        let model: Option<Wood> = Wood::filter_by_id(id.as_uuid())
            .first()
            .exec(&mut db)
            .await
            .map_err(database_error)?;

        model.map(to_domain).transpose()
    }

    async fn list(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<domain::Wood>, RepositoryError> {
        let mut db = self.db.clone();

        let models: Vec<Wood> = Wood::all()
            .order_by(Wood::fields().scientific_name().asc())
            .limit(limit)
            .offset(offset)
            .exec(&mut db)
            .await
            .map_err(database_error)?;

        models.into_iter().map(to_domain).collect()
    }

    async fn count(&self) -> Result<u64, RepositoryError> {
        let mut db = self.db.clone();

        Wood::all()
            .count()
            .exec(&mut db)
            .await
            .map_err(database_error)
    }

    async fn save(&self, wood: &domain::Wood) -> Result<domain::Wood, RepositoryError> {
        let mut db = self.db.clone();
        let record = WoodRecord::from(wood);

        let model = Wood::create()
            .id(record.id)
            .name(record.name)
            .scientific_name(record.scientific_name)
            .family(record.family)
            .collection_location(record.collection_location)
            .common_names(record.common_names)
            .source_species_id(record.source_species_id)
            .source_page_url(record.source_page_url)
            .tree(record.tree)
            .general_characteristics(record.general_characteristics)
            .description(record.description)
            .kiln_drying_schedule(record.kiln_drying_schedule)
            .kiln_drying_days(record.kiln_drying_days)
            .images(record.images)
            .physical_properties(record.physical_properties)
            .mechanical_properties(record.mechanical_properties)
            .workabilities(record.workabilities)
            .exec(&mut db)
            .await
            .map_err(database_error)?;

        to_domain(model)
    }

    async fn update(&self, wood: &domain::Wood) -> Result<domain::Wood, RepositoryError> {
        let mut db = self.db.clone();
        let record = WoodRecord::from(wood);

        Wood::filter_by_id(record.id)
            .update()
            .name(record.name)
            .scientific_name(record.scientific_name)
            .family(record.family)
            .collection_location(record.collection_location)
            .common_names(record.common_names)
            .source_species_id(record.source_species_id)
            .source_page_url(record.source_page_url)
            .tree(record.tree)
            .general_characteristics(record.general_characteristics)
            .description(record.description)
            .kiln_drying_schedule(record.kiln_drying_schedule)
            .kiln_drying_days(record.kiln_drying_days)
            .images(record.images)
            .physical_properties(record.physical_properties)
            .mechanical_properties(record.mechanical_properties)
            .workabilities(record.workabilities)
            .exec(&mut db)
            .await
            .map_err(database_error)?;

        self.find_by_id(wood.id())
            .await?
            .ok_or(RepositoryError::OperationFailed)
    }

    async fn delete(&self, id: domain::WoodId) -> Result<(), RepositoryError> {
        let mut db = self.db.clone();

        Wood::delete_by_id(&mut db, id.as_uuid())
            .await
            .map_err(database_error)
    }
}

fn to_domain(model: Wood) -> Result<domain::Wood, RepositoryError> {
    domain::Wood::try_from(model).map_err(|err| RepositoryError::InvalidData(err.to_string()))
}

fn database_error(err: toasty::Error) -> RepositoryError {
    tracing::error!(error = %err, "wood repository query failed");
    RepositoryError::Database
}
