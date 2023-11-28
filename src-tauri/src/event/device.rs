/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-27 14:46:10
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-28 10:17:14
 * @Description: file content
 */
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::{
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};
use tauri::Manager;

pub fn execute<Callback: Fn() + Send + Sync + 'static>(
    code: Vec<Keycode>,
    callback: Callback,
) -> JoinHandle<()> {
    if code.len() == 0 {
        println!("code not null")
    }
    spawn(move || {
        let device_state = DeviceState::new();
        loop {
            let keys = device_state.get_keys();
            // 捕获 LC + C
            if code.eq(&keys) {
                println!("keys is {:?}", keys);
                callback()
            }

            // 一秒查10次
            sleep(Duration::from_millis(100));
        }
    })
}

pub fn window_enent(app: &mut tauri::App) {
    let app_handle = app.app_handle();
    let show = Arc::new(Mutex::new(true));
    let weak_up = vec![Keycode::Space, Keycode::LShift];
    
    let _res: JoinHandle<()> = execute(weak_up, move || {
        let window = app_handle.get_window("main").unwrap();
        let mut visible = show.lock().unwrap();

        if *visible {
            let _ = window.hide();
            *visible = false;
        } else {
            let _ = window.show();
            *visible = true
        }
    });
}

pub fn run() {
    let copy = vec![Keycode::C, Keycode::LControl];
    let _copy: JoinHandle<()> = execute(copy, || println!("回调"));
}
