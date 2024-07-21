use sea_orm::{ConnectOptions, Database};
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};
use std::time::Duration;
use std::{fs::File, path::Path};
use tracing::{debug, info};
use user::Entity as User;

use crate::error::BackendError;

pub mod auth;
pub mod user;

#[derive(Debug, Clone)]
pub struct DB {
    pub path: String,
    pub conn: DatabaseConnection,
}

impl DB {
    pub async fn new(path: &str) -> Result<Self, BackendError> {
        // if the db file does not exist, create it
        if !Path::new(path).exists() {
            File::create(path).expect("Could not create db file");
            debug!("Created db file");
        }

        // setup db options
        let mut opt = ConnectOptions::new(format!("sqlite://{}", path));
        opt.max_connections(100)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Trace);

        // connect to the db
        debug!("Connecting to the db...");
        let db = Database::connect(opt).await?;
        info!("Connected to the db");

        Ok(Self {
            conn: db,
            path: path.to_owned(),
        })
    }

    pub async fn create_tables(&self) -> Result<(), BackendError> {
        let builder = self.conn.get_database_backend();
        let schema = Schema::new(builder);

        // if the table does not exist, create it
        match self
            .conn
            .execute(builder.build(&schema.create_table_from_entity(User)))
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.to_string().contains("already exists") {
                    return Ok(());
                }

                Err(err)
            }
        }?;
        Ok(())
    }
}
