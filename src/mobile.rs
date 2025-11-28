use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::intents::{pop_and_extract_text_intent, pop_intent};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_mobile_sharetarget);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<MobileSharetarget<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("com.plugin.mobilesharetarget", "SharetargetPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_mobile_sharetarget)?;
    Ok(MobileSharetarget(handle))
}

/// Access to the mobile-sharetarget APIs.
pub struct MobileSharetarget<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> MobileSharetarget<R> {
    pub fn get_latest_intent(&self) -> crate::Result<Option<String>> {
        Ok(pop_intent())
    }

    pub fn get_latest_intent_and_extract_text(&self) -> crate::Result<Option<String>> {
        Ok(pop_and_extract_text_intent())
    }
}
