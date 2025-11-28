use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod intents;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MobileSharetarget;
#[cfg(mobile)]
use mobile::MobileSharetarget;

pub use crate::intents::push_new_intent;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mobile-sharetarget APIs.
pub trait MobileSharetargetExt<R: Runtime> {
    fn mobile_sharetarget(&self) -> &MobileSharetarget<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MobileSharetargetExt<R> for T {
    fn mobile_sharetarget(&self) -> &MobileSharetarget<R> {
        self.state::<MobileSharetarget<R>>().inner()
    }
}

use std::sync::OnceLock;
pub static IOS_DEEP_LINK_SCHEME: OnceLock<String> = OnceLock::new();

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mobile-sharetarget")
        .invoke_handler(tauri::generate_handler![
            commands::pop_intent_queue,
            commands::pop_intent_queue_and_extract_text
        ])
        .setup(|app, api| {
            #[cfg(target_os = "ios")]
            IOS_DEEP_LINK_SCHEME
                .set(
                    extract_deep_link_scheme(&app.config().plugins.0)
                        .expect("Deeplink plugin hasn't been installed or properly configured"),
                )
                .expect("Once Lock already set");
            #[cfg(mobile)]
            let mobile_sharetarget = mobile::init(app, api)?;
            #[cfg(desktop)]
            let mobile_sharetarget = desktop::init(app, api)?;
            app.manage(mobile_sharetarget);
            Ok(())
        })
        .build()
}

#[cfg(target_os = "android")]
use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_plugin_mobilesharetarget_Sharetarget_pushIntent(
    mut env: JNIEnv,
    _class: JClass,
    intent: JString,
) {
    println!("Calling JNI Hello World!");

    let input: String = env
        .get_string(&intent)
        .expect("Couldn't get java string!")
        .into();

    push_new_intent(input);
}

#[cfg(target_os = "ios")]
use anyhow::anyhow;
#[cfg(target_os = "ios")]
use serde_json::Value;
#[cfg(target_os = "ios")]
use std::collections::HashMap;
#[cfg(target_os = "ios")]
fn extract_deep_link_scheme(plugins_value: &HashMap<String, Value>) -> Result<String> {
    // 1. Get the "deep-link" object
    let deep_link = plugins_value
        .get("deep-link")
        .ok_or(anyhow!("Error: Missing 'deep-link' configuration block."))?;

    // 2. Get the "mobile" array
    let mobile_array = deep_link
        .get("mobile")
        .ok_or(anyhow!("Error: Missing 'mobile' key within 'deep-link'."))?
        .as_array()
        .ok_or(anyhow!("Error: 'mobile' value is not a JSON array."))?;

    // 3. Get the first element [0] of the mobile array (the object)
    let first_mobile_obj = mobile_array
        .get(0)
        .ok_or(anyhow!("Error: 'mobile' array is empty."))?;

    // 4. Get the "scheme" array from the first mobile object
    let scheme_array = first_mobile_obj
        .get("scheme")
        .ok_or(anyhow!(
            "Error: Missing 'scheme' key in mobile configuration."
        ))?
        .as_array()
        .ok_or(anyhow!("Error: 'scheme' value is not a JSON array."))?;

    // 5. Get the first element [0] of the scheme array (the string value)
    let first_scheme_value = scheme_array
        .get(0)
        .ok_or(anyhow!("Error: 'scheme' array is empty."))?
        .as_str()
        .ok_or(anyhow!("Error: Scheme value is not a string."))?;

    // Return the extracted string value
    Ok(first_scheme_value.to_string())
}
