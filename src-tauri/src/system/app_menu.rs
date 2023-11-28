/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-27 16:39:37
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-27 16:43:11
 * @Description: file content
 */
use tauri::utils::assets::EmbeddedAssets;
use tauri::{ AboutMetadata, Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent };

// 应用菜单项
pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    // 应用名称
    let name = &context.package_info().name;
    // tauri::Menu::os_default(name)
    // 应用主菜单
    let app_menu = Submenu::new(
        "",
        // MenuItem::About 为原生菜单
        Menu::new().add_native_item(MenuItem::About(name.into(), AboutMetadata::new()))
    );
    // 文件菜单（自定义菜单）
    let file_menu = Submenu::new(
        "File(TODO)",
        Menu::new()
            .add_item(CustomMenuItem::new("import_config".to_string(), "导入配置"))
            .add_item(CustomMenuItem::new("export_config_template".to_string(), "导出配置模板"))
    );
    // 编辑菜单（自定义菜单）
    let edit_menu = Submenu::new(
        "Edit(TODO)",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "Undo"))
            .add_item(CustomMenuItem::new("redo".to_string(), "Redo"))
    );

    Menu::new().add_submenu(app_menu).add_submenu(file_menu).add_submenu(edit_menu)
}

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let _win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "import_config" => {
            // debug 信息（终端输出）
            dbg!("new file");
        }
        "export_config_template" => {
            // 发送信息到菜单所属窗口（弹窗形式）

        }
        "undo" => {
            dbg!("undo");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => { print!("{}", event.menu_item_id()) }
    }
}