use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::intents::{pop_and_extract_text_intent, pop_intent};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MobileSharetarget<R>> {
    Ok(MobileSharetarget(app.clone()))
}

/// Access to the mobile-sharetarget APIs.
pub struct MobileSharetarget<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MobileSharetarget<R> {
    pub fn get_latest_intent(&self) -> crate::Result<Option<String>> {
        Ok(pop_intent())
    }

    pub fn get_latest_intent_and_extract_text(&self) -> crate::Result<Option<String>> {
        Ok(pop_and_extract_text_intent())
    }
}
