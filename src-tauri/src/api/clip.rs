/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-28 10:33:38
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-29 14:21:23
 * @Description: file content
 */
extern crate clipboard;

use rusqlite::Connection;
use serde::Deserialize;
use serde::Serialize;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

const DB: &str = "plankton.db";

pub fn run() {
    spawn(move || {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let conn = Connection::open(DB).unwrap();
        let _ = conn.execute(
            "CREATE TABLE history (id INTEGER primary key,  data  TEXT)",
            (),
        );
        let mut last_conent = String::new();

        let mut stmt = conn
            .prepare("SELECT data FROM history order by id desc limit 1")
            .unwrap();
        let mut row = stmt.query([]).unwrap();
        while let Some(row) = row.next().unwrap() {
            last_conent = row.get(0).unwrap();
        }

        loop {
            let content = ctx.get_contents();
            match content {
                Ok(value) => {
                    if value.is_empty() || value.eq(&last_conent) {
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                    last_conent = value.clone();
                    let _ = conn.execute("INSERT INTO history (data) VALUES (?1)", [&last_conent]);
                }
                Err(err) => {
                    println!("{}", err);
                }
            };
            sleep(Duration::from_millis(1000));
        }
    });
}

#[tauri::command]
pub fn get_one() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let content = ctx.get_contents();
    match content {
        Ok(value) => value,
        Err(_) => String::new(),
    }
}

#[tauri::command]
pub fn get_all() -> Vec<History> {
    let conn = Connection::open(DB).unwrap();
    let mut res = Vec::new();
    let mut stmt = conn
        .prepare("SELECT id, data FROM history order by id desc ")
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(History {
                id: row.get(0).unwrap(),
                data: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        res.push(row.unwrap())
    }
    res
}

#[tauri::command]
pub fn copy(value: &str) -> &str {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let r = ctx.set_contents(value.to_string());
    match r {
        Ok(_) => "success",
        Err(_) => "error",
    }
}

#[tauri::command]
pub fn delete(id: usize) -> usize {
    let conn = Connection::open(DB).unwrap();
    conn.execute("DELETE FROM history WHERE id = ?1", [id])
        .unwrap()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct History {
    id: i32,
    data: String,
}

#[cfg(test)]
mod tests {
    use snowflake::ProcessUniqueId;

    #[test]
    fn test() {
        println!("{}", ProcessUniqueId::new().to_string());
    }
}
