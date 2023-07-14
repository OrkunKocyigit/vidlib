use rusqlite::Connection;
use std::fs;
use tauri::AppHandle;

const DATABASE_VERSION: f32 = 1.0;

pub fn load_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist");
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let sqlite = path.join("profile.sqlite");
    let db = Connection::open(sqlite)?;
    let user_version: f32 = db.pragma_query_value(None, "user_version", |row| Ok(row.get(0)?))?;
    Ok(db)
}
