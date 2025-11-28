use tauri::{command, AppHandle, Runtime};

use crate::{MobileSharetargetExt, Result};

#[command]
pub(crate) fn get_latest_intent<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    app.mobile_sharetarget().get_latest_intent()
}

#[command]
pub(crate) fn get_latest_intent_and_extract_text<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<String>> {
    app.mobile_sharetarget()
        .get_latest_intent_and_extract_text()
}
