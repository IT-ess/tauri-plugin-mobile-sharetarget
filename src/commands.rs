use tauri::{command, AppHandle, Runtime};

// use crate::models::*;
// use crate::MobileSharetargetExt;
use crate::{MobileSharetargetExt, Result};

// #[command]
// pub(crate) async fn ping<R: Runtime>(
//     app: AppHandle<R>,
//     payload: PingRequest,
// ) -> Result<PingResponse> {
//     app.mobile_sharetarget().ping(payload)
// }

#[command]
pub(crate) fn get_latest_intent<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    app.mobile_sharetarget().get_latest_intent()
}
