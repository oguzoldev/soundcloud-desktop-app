use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::menu::{Menu, Submenu, PredefinedMenuItem, AboutMetadata, MenuItem};
                use tauri::tray::{TrayIconBuilder, TrayIconEvent};

                let app_menu = Submenu::with_items(
                    app,
                    "App",
                    true,
                    &[
                        &PredefinedMenuItem::about(app, None, Some(AboutMetadata::default()))?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::hide(app, None)?,
                        &PredefinedMenuItem::hide_others(app, None)?,
                        &PredefinedMenuItem::show_all(app, None)?,
                        &PredefinedMenuItem::quit(app, None)?,
                    ],
                )?;
                let edit_menu = Submenu::with_items(
                    app,
                    "Edit",
                    true,
                    &[
                        &PredefinedMenuItem::undo(app, None)?,
                        &PredefinedMenuItem::redo(app, None)?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::cut(app, None)?,
                        &PredefinedMenuItem::copy(app, None)?,
                        &PredefinedMenuItem::paste(app, None)?,
                        &PredefinedMenuItem::select_all(app, None)?,
                    ],
                )?;
                let window_menu = Submenu::with_items(
                    app,
                    "Window",
                    true,
                    &[
                        &PredefinedMenuItem::minimize(app, None)?,
                        &PredefinedMenuItem::close_window(app, None)?,
                    ],
                )?;
                let menu = Menu::with_items(app, &[&app_menu, &edit_menu, &window_menu])?;
                app.set_menu(menu)?;

                // System Tray
                let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
                let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .menu(&menu)
                    .menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.eval("
                                    const style = document.getElementById('low-power-mode-style');
                                    if (style) style.remove();
                                ");
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click { .. } = event {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.eval("
                                    const style = document.getElementById('low-power-mode-style');
                                    if (style) style.remove();
                                ");
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .icon(app.default_window_icon().unwrap().clone())
                    .build(app)?;
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if let Some(webview) = window.app_handle().get_webview_window(window.label()) {
                    let _ = webview.eval("
                        if (!document.getElementById('low-power-mode-style')) {
                            const style = document.createElement('style');
                            style.id = 'low-power-mode-style';
                            style.textContent = '* { animation-play-state: paused !important; transition: none !important; }';
                            document.head.appendChild(style);
                        }
                    ");
                }
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.eval("
                        const style = document.getElementById('low-power-mode-style');
                        if (style) style.remove();
                    ");
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
        });
}
