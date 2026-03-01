use sqlx::{ConnectOptions, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use directories::ProjectDirs;

pub async fn init_db() -> SqlitePool {
    // Rust detects the file system where the app is loaded
    let proj_dirs = ProjectDirs::from("com", "angiro22", "todo_app_dioxus").expect("Impossible to define file system folders");

    // Defines the data directory (es. AppData/Local/... on Windows)
    let data_dir = proj_dirs.data_dir();

    // Creates the data directory (Needed by Android)
    std::fs::create_dir_all(data_dir) .expect("Impossible to create data directory"); // works only if the directory isn't created yet

    // Sets the path to the database (with join it handles the '/' and '\' symbols for Windows/Android)
    let db_path = data_dir.join("task.db");  
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap()); // Trasforma il percorso in un url sqlite

    let mut opts = SqliteConnectOptions::from_str(&db_url)
        .expect("Invalid URL") 
        .create_if_missing(true) // If task.db doesn't exsists it is created
        .disable_statement_logging(); // Disable the logs in console

    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("DB connection error");

    sqlx::query("
        CREATE TABLE IF NOT EXSISTS tasks (
            id INTEGER AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            done BOOLEAN NOT NULL DEFAULT false
        );
    ")
    .execute(&pool) // Executes query on the db pool 
    .await
    .expect("Table creation error");    

    pool
}