use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;
use dotenvy::dotenv;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // Load environment variables from `.env` file
    dotenv().ok();

    // Get the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection to the database
    Database::connect(&database_url).await
}
