use toasty_cli::ToastyCli;

use crate::errors::InfrastructureError;

pub mod errors;
pub mod woods;

pub async fn build_db(database_uri: &str) -> Result<toasty::Db, InfrastructureError> {
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(database_uri)
        .await?;

    db.push_schema().await?;

    Ok(db)
}

pub fn cli(args: Vec<&str>) -> Result<(), InfrastructureError> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| InfrastructureError::Fatal(e.to_string()))?;

    let mut args: Vec<String> = args.into_iter().map(|str| str.to_string()).collect();
    let _ = args.insert_mut(0, String::from("--"));

    let handle: tokio::task::JoinHandle<Result<_, InfrastructureError>> = rt.spawn(async move {
        let config = toasty_cli::Config::load()?;

        let db = build_db("sqlite::memory:").await?;

        let cli = ToastyCli::with_config(db, config);
        cli.parse_from(args).await?;

        Ok(())
    });

    let _ = rt
        .block_on(handle)
        .map_err(|e| InfrastructureError::Fatal(e.to_string()))?;

    Ok(())
}

impl From<toasty::Error> for InfrastructureError {
    fn from(err: toasty::Error) -> Self {
        Self::Database(err.into())
    }
}
