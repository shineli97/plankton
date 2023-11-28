use tauri::{
    SystemTray,
    SystemTrayMenu,
    CustomMenuItem,
    SystemTrayMenuItem,
    AppHandle,
    SystemTrayEvent,
    Manager,
};

// 构建托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    let _parent_window = Some(&window);

    match event {
        SystemTrayEvent::LeftClick { tray_id:_, position: _, size: _, .. } => {
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { tray_id:_, id, .. } =>
            match id.as_str() {
                "hide" => {
                    window.hide().unwrap();
                }
                "show" => {
                    window.show().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }

        _ => { println!("暂不支持") }
    }
}