use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::env;
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
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

        let mut opt = ConnectOptions::new(database_url);

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
