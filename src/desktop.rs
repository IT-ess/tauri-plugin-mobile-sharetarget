use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<MobileSharetarget<R>> {
  Ok(MobileSharetarget(app.clone()))
}

/// Access to the mobile-sharetarget APIs.
pub struct MobileSharetarget<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MobileSharetarget<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
