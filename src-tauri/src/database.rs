use rusqlite::{Connection, Error};
use std::fs;
use tauri::AppHandle;

pub fn load_database(app_handle: &AppHandle) -> Result<Connection, Error> {
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("App data directory does not exist");
    fs::create_dir_all(&path).expect("App data directory creation failed");
    let sqlite = path.join("profile.sqlite");
    let mut db = Connection::open(sqlite)?;
    let user_version: f32 = db.pragma_query_value(None, "user_version", |row| Ok(row.get(0)?))?;
    upgrade_database(&mut db, user_version)?;
    Ok(db)
}

fn upgrade_database(connection: &mut Connection, version: f32) -> Result<(), Error> {
    if version < 1.0 {
        let transaction = connection.transaction()?;
        let sql = "CREATE TABLE PATHS (
        id Integer PRIMARY KEY,
        path TEXT NOT NULL
        )";
        transaction
            .execute(sql, [])
            .expect("Path table creation failed");
        transaction.pragma_update(None, "user_version", 1.0)?;
        transaction.commit()?;
    }
    Ok(())
}
