use tauri::{command, AppHandle, Runtime};

use crate::{MobileSharetargetExt, Result};

#[command]
/// Pops the first item in the intents queue if there is one, and return it (raw).
pub(crate) fn pop_intent_queue<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    app.mobile_sharetarget().get_latest_intent()
}

#[command]
/// Pops the first item in the intents queue if there is one, and return its parsed payload.
pub(crate) fn pop_intent_queue_and_extract_text<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<String>> {
    app.mobile_sharetarget()
        .get_latest_intent_and_extract_text()
}
