use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use std::time::Duration;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let mut opt =
        ConnectOptions::new("postgres://personal:secret@localhost:5432/hello_rocket".to_string());

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;

    Ok(db)
}
