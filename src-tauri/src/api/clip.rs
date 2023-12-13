/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-28 10:33:38
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 10:46:54
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
        let get_data_sql = "SELECT data FROM history order by time desc limit 1";
        let _ = conn.execute(
            "CREATE TABLE history (
                    id INTEGER primary key, 
                    data TEXT, 
                    time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime'))
                )",
            (),
        );
        let mut stmt = conn.prepare(get_data_sql).unwrap();
        let mut last_content = String::new();
        loop {
            let res = stmt.query_row([], |row| Ok(row.get(0).expect("没数据")));
            match res {
                Ok(val) => last_content = val,
                Err(err) => println!("get data error:{}", err),
            }
            let content = ctx.get_contents();
            match content {
                Ok(value) => {
                    if value.is_empty() || value.eq(&last_content) {
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                    let _ = conn.execute("INSERT INTO history (data) VALUES (?1)", [value]);
                }
                Err(err) => {
                    println!("insert data error:{}", err);
                }
            };
            sleep(Duration::from_millis(1000));
        }
    });
}

#[tauri::command]
pub fn get_all() -> Vec<History> {
    let conn = Connection::open(DB).unwrap();
    let mut res = Vec::new();
    let mut stmt = conn
        .prepare("SELECT id, data, time FROM history order by time desc ")
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(History {
                id: row.get(0).unwrap(),
                data: row.get(1).unwrap(),
                time: row.get(2).unwrap(),
            })
        })
        .unwrap();

    for row in rows {
        res.push(row.unwrap())
    }
    res
}

#[tauri::command]
pub fn copy(id: usize, data: &str) -> &str {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let r = ctx.set_contents(data.to_string());
    match r {
        Ok(_) => {
            let conn = Connection::open(DB).unwrap();
            let _ = conn.execute(
                "UPDATE history SET time = (datetime(CURRENT_TIMESTAMP,'localtime')) WHERE id = ?1",
                [id],
            );
            "success"
        }
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
    time: String,
}
