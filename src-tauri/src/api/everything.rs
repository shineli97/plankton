/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-04 12:14:24
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:06:09
 * @Description: file content
 */
use chrono::{DateTime, Local, NaiveDateTime, Offset, Utc};
use everything_rs::{Everything, EverythingRequestFlags, EverythingSort};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[tauri::command]
pub fn search(name: &str) -> Vec<FileItem> {
    let mut res: Vec<FileItem> = Vec::new();
    let everything = Everything::new();
    everything.set_search(name);
    everything.set_max_results(10);
    everything.set_result_offset(10);
    everything.set_request_flags(
        EverythingRequestFlags::FileName
            | EverythingRequestFlags::Path
            | EverythingRequestFlags::FullPathAndFileName
            | EverythingRequestFlags::DateCreated
            | EverythingRequestFlags::DateModified
            | EverythingRequestFlags::Size
            | EverythingRequestFlags::Extension,
    );
    everything.set_sort(EverythingSort::TypeNameDescending);
    everything.query().unwrap();

    for (i, path) in everything.full_path_iter().enumerate() {
        let index = i.try_into().unwrap();
        let name = everything.get_result_file_name(index).unwrap();
        let size: u64 = everything.get_result_size(index).unwrap();
        let created_date = everything.get_result_created_date(index).unwrap();
        let time =
            NaiveDateTime::from_timestamp_millis((created_date / 100_000).try_into().unwrap())
                .unwrap();
        let temp = FileItem {
            index,
            name,
            path: path.expect("msg"),
            size: format_size(size),
            created_date: time.and_utc().format("%Y/%m/%d %H:%M:%S").to_string(),
        };
        res.push(temp);
    }
    res
}

#[tauri::command]
pub fn open(path: &str) {
    Command::new("explorer").arg(path).spawn().unwrap();
}

pub fn format_size(mut size: u64) -> String {
    let unit;
    if size < 1024 {
        unit = " B";
    } else if size < 1024 * 1024 {
        size = size / 1024;
        unit = " KB";
    } else if size < 1024 * 1024 * 1024 {
        size = size / (1024 * 1024);
        unit = " MB";
    } else if size < 1024 * 1024 * 1024 * 1024 {
        size = size / (1024 * 1024 * 1024);
        unit = " GB";
    } else if size < 1024 * 1024 * 1024 * 1024 * 1024 {
        size = size / (1024 * 1024 * 1024 * 1024);
        unit = " TB";
    } else {
        unit = "...";
    }
    let mut res = String::from(size.to_string());
    res.push_str(unit);
    res
}

pub fn start() {
    Command::new("./Everything.exe")
        .arg("-first-instance")
        .arg("-startup")
        .spawn()
        .expect("后台运行Everything实例出错");
}

pub fn close() {
    Command::new("./Everything.exe")
        .arg("-exit")
        .spawn()
        .expect("退出Everything实例出错");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileItem {
    index: u32,
    name: String,
    path: String,
    size: String,
    created_date: String,
}

#[cfg(test)]
mod test {
    use crate::api::everything::search;
    use std::time::SystemTime;

    #[test]
    fn test() {
        let res = search("test");
        println!("res:{:?}", res);
    }
}
