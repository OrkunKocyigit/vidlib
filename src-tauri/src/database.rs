use std::collections::HashMap;
use std::fs;

use rusqlite::{named_params, Connection, Error};
use tauri::AppHandle;

use crate::state::VideoCacheItem;
use crate::video::VideoEntry;

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
    let transaction = connection.transaction()?;
    if version < 1 {
        let sql = "CREATE TABLE PATHS (
            id Integer PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL
        )";
        transaction.execute(sql, [])?;
        transaction.pragma_update(None, "user_version", 1)?;
    }
    if version < 2 {
        let sql = "CREATE TABLE VIDEOS (
            id TEXT PRIMARY KEY,
            name TEXT,
            rating INTEGER,
            notes TEXT,
            watched INTEGER,
            category INTEGER
        )";
        transaction.execute(sql, [])?;
        transaction.pragma_update(None, "user_version", 2)?;
    }
    if version < 3 {
        let sql = "CREATE TABLE VIDEO_CACHE (
            path TEXT PRIMARY KEY,
            size NUMBER,
            id TEXT
        )";
        transaction.execute(sql, [])?;
        transaction.pragma_update(None, "user_version", 3)?;
    }
    transaction.commit()?;
    Ok(())
}

pub fn get_paths(connection: &Connection) -> Result<Vec<String>, Error> {
    let mut query = connection.prepare("SELECT path FROM PATHS ORDER BY id")?;
    let rows = query.query_map([], |row| row.get("path"))?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn add_path(connection: &Connection, path: &String) -> Result<(), Error> {
    connection
        .prepare("INSERT INTO PATHS(path) VALUES (@path)")?
        .execute(named_params! {"@path":path})?;
    Ok(())
}

pub fn get_videos(connection: &Connection) -> Result<HashMap<String, VideoEntry>, Error> {
    let mut query = connection.prepare("SELECT * FROM VIDEOS")?;
    let rows = query.query_map([], |row| {
        Ok((
            row.get("id")?,
            VideoEntry::new(
                row.get("name")?,
                row.get("rating")?,
                row.get("notes")?,
                row.get("watched")?,
            ),
        ))
    })?;
    Ok(rows.collect::<Result<HashMap<_, _>, _>>()?)
}

pub fn add_video(
    connection: &Connection,
    id: &String,
    video_entry: &VideoEntry,
) -> Result<(), Error> {
    connection.prepare("INSERT INTO VIDEOS(id, name, rating, notes, watched) VALUES (@id, @name, @rating, @notes, @watched)")?.execute(named_params! {
        "@id": id,
        "@name": video_entry.name(),
        "@rating": video_entry.rating(),
        "@notes": video_entry.notes(),
        "@watched": video_entry.watched(),
    })?;
    Ok(())
}

pub(crate) fn update_rating(connection: &Connection, id: &String, new_rating: usize) -> Option<()> {
    connection
        .prepare("UPDATE VIDEOS SET RATING = @rating WHERE ID = @id")
        .expect("Query failed")
        .execute(named_params! {
            "@rating": new_rating,
            "@id": id
        })
        .expect("Execute failed");
    Some(())
}

pub(crate) fn update_watched(
    connection: &Connection,
    id: &String,
    new_watched: bool,
) -> Option<()> {
    connection
        .prepare("UPDATE VIDEOS SET WATCHED = @watched WHERE ID = @id")
        .expect("Query Failed")
        .execute(named_params! {
            "@watched": new_watched,
            "@id": id
        })
        .expect("Execute failed");
    Some(())
}

pub(crate) fn add_video_cache(
    connection: &Connection,
    path: &String,
    size: &u64,
    id: &String,
) -> () {
    connection
        .prepare("INSERT INTO VIDEO_CACHE(path, size, id) VALUES(@path, @size, @id)")
        .expect("Query Failed")
        .execute(named_params! {
            "@path": path,
            "@size": size,
            "@id": id
        })
        .expect("Execute failed");
}

pub(crate) fn delete_video_cache(connection: &Connection, path: &String) -> () {
    connection
        .prepare("DELETE FROM VIDEO_CACHE WHERE path = @path")
        .expect("Query Failed")
        .execute(named_params! {
            "@path": path,
        })
        .expect("Execute failed");
}

pub(crate) fn get_video_cache_items(
    connection: &Connection,
) -> Result<HashMap<String, VideoCacheItem>, Error> {
    let mut query = connection.prepare("SELECT * FROM VIDEO_CACHE")?;
    let rows = query.query_map([], |row| {
        Ok((
            row.get("path")?,
            VideoCacheItem::new(row.get("size")?, row.get("id")?),
        ))
    })?;
    Ok(rows.collect::<Result<HashMap<_, _>, _>>()?)
}

pub(crate) fn update_name(c: &Connection, i: &String, n: &String) -> Option<()> {
    c.prepare("UPDATE VIDEOS SET NAME = @name WHERE ID = @id")
        .expect("Query Failed")
        .execute(named_params! {
            "@name": n,
            "@id": i
        })
        .expect("Execute failed");
    Some(())
}

pub(crate) fn update_notes(c: &Connection, i: &String, n: &String) -> Option<()> {
    c.prepare("UPDATE VIDEOS SET NOTES = @notes WHERE ID = @id")
        .expect("Query Failed")
        .execute(named_params! {
            "@notes": n,
            "@id": i
        })
        .expect("Execute failed");
    Some(())
}

pub(crate) fn delete_path(db: &mut Connection, path: &str) -> Result<(), Error> {
    let transaction = db.transaction()?;
    transaction
        .prepare("DELETE FROM PATHS WHERE PATH = @path")?
        .execute(named_params! {"@path": path})?;
    transaction.commit()?;
    Ok(())
}

pub(crate) fn get_cache_items_with_path(
    db: &mut Connection,
    path: &str,
) -> Result<Vec<String>, Error> {
    let mut query = db.prepare("SELECT path FROM VIDEO_CACHE WHERE path LIKE @path")?;
    let rows = query.query_map(
        named_params! {
            "@path": format!("{}%", path)
        },
        |row| row.get("path"),
    )?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}
