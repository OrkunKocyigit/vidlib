use std::fs;

use rusqlite::{named_params, Connection, Error};
use tauri::AppHandle;

pub fn load_database(app_handle: &AppHandle) -> Result<Connection, Error> {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist");
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let sqlite = path.join("profile.sqlite");
    let mut db = Connection::open(sqlite)?;
    let user_version: u32 = db.pragma_query_value(None, "user_version", |row| Ok(row.get(0)?))?;
    upgrade_database(&mut db, user_version)?;
    Ok(db)
}

fn upgrade_database(connection: &mut Connection, version: u32) -> Result<(), Error> {
    if version < 1 {
        let transaction = connection.transaction()?;
        let sql = "CREATE TABLE PATHS (
        id Integer PRIMARY KEY AUTOINCREMENT,
        path TEXT NOT NULL
        )";
        transaction
            .execute(sql, [])
            .expect("Path table creation failed");
        transaction.pragma_update(None, "user_version", 1)?;
        transaction.commit()?;
    }
    if version < 2 {
        let transaction = connection.transaction()?;
        let sql = "CREATE TABLE VIDEOS (
        id TEXT PRIMARY KEY,
        name TEXT,
        rating INTEGER,
        notes TEXT,
        watched INTEGER,
        category INTEGER
        )";
        transaction
            .execute(sql, [])
            .expect("Path table creation failed");
        transaction.pragma_update(None, "user_version", 2)?;
        transaction.commit()?;
    }
    Ok(())
}

pub fn get_paths(connection: &Connection) -> Result<Vec<String>, Error> {
    let mut query = connection.prepare("SELECT path FROM PATHS ORDER BY id")?;
    let mut rows = query.query([])?;
    let mut paths: Vec<String> = Vec::new();
    while let Some(row) = rows.next()? {
        let path = row.get("path")?;
        paths.push(path);
    }
    Ok(paths)
}

pub fn add_path(connection: &Connection, path: &String) -> Result<(), Error> {
    let mut query = connection.prepare("INSERT INTO PATHS(path) VALUES (@path)")?;
    query.execute(named_params! {"@path":path})?;
    Ok(())
}