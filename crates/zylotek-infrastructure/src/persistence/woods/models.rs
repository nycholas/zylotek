use toasty::Model;
use uuid::Uuid;

#[derive(Debug, Model)]
pub struct Wood {
    #[key]
    pub id: Uuid,

    pub name: String,

    pub scientific_name: String,
}
