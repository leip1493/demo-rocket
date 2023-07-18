use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;

#[derive(Database, Debug)]
#[database("hello_rocket")]
pub struct DB(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub connection: DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        // let config = figment.extract::<Config>().unwrap();
        // let mut options: ConnectOptions = config.url.into();
        // options
        //     .max_connections(config.max_connections as u32)
        //     .min_connections(config.min_connections.unwrap_or_default())
        //     .connect_timeout(Duration::from_secs(config.connect_timeout))
        //     .sqlx_logging(config.sqlx_logging);
        // if let Some(idle_timeout) = config.idle_timeout {
        //     options.idle_timeout(Duration::from_secs(idle_timeout));
        // }
        // let conn = sea_orm::Database::connect(options).await?;

        let mut opt = ConnectOptions::new(
            "postgres://personal:secret@localhost:5432/hello_rocket".to_string(),
        );

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));

        let connection: DatabaseConnection = SeaOrmDatabase::connect(opt).await?;

        Ok(SeaOrmPool { connection })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.connection
    }
}