use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};
use sea_orm_rocket::{rocket::figment::Figment, Database};
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

        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not found");
        let database_user = env::var("DATABASE_USER").expect("DATABASE_USER not found");
        let database_password = env::var("DATABASE_PASS").expect("DATABASE_PASS not found");
        let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST not found");
        let database_port = env::var("DATABASE_PORT").expect("DATABASE_PORT not found");
        let database_url = format!("postgres://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}");

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
