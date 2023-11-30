/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-24 14:57:58
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-29 15:07:09
 * @Description: 骗自己可以，骗兄弟也可以，但是不能骗爷爷。爷爷年纪大了，记性也不好了，记不得那么多东西，前面忘了，中间忘了，后面也忘了，但是还是不能骗爷爷
 */
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;
mod event;
mod system;
use api::clip;
use event::device;
use system::{tray_menu, app_menu};
use tauri::Manager;
use tokio::sync::mpsc;

fn main() {
    let context = tauri::generate_context!();
    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    let _app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            clip::copy,
            clip::delete,
            clip::get_all,
            clip::get_one
        ])
        .setup(|app: &mut tauri::App| {
            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        rs2js(output, &app_handle);
                    }
                }
            });

            device::window_enent(app);
            clip::run();

            Ok(())
        })
        .system_tray(tray_menu::menu())
        .on_system_tray_event(tray_menu::handler)
        .menu(app_menu::init(&context))
        .on_menu_event(app_menu::handler)
        .run(context)
        .expect("error while running tauri application");
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        while let Some(input) = input_rx.recv().await {
            let output = input;
            output_tx.send(output).await?;
        }
    }
}

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
}
