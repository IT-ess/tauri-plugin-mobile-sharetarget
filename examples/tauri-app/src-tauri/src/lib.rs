use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_mobile_sharetarget::{push_new_intent, IOS_DEEP_LINK_SCHEME};

// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_mobile_sharetarget::init())
        .setup(|app| {
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
                println!("deep link URLs: {:?}", urls);
                if let Some(url) = urls.first() {
                    if url.scheme().eq(IOS_DEEP_LINK_SCHEME.wait()) {
                        push_new_intent(url.to_string());
                    }
                }
            }

            app.deep_link().on_open_url(move |event| {
                println!("got new url");
                if let Some(url) = event.urls().first() {
                    if url.scheme().eq(IOS_DEEP_LINK_SCHEME.wait()) {
                        push_new_intent(url.to_string());
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
