use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MobileSharetarget;
#[cfg(mobile)]
use mobile::MobileSharetarget;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mobile-sharetarget APIs.
pub trait MobileSharetargetExt<R: Runtime> {
  fn mobile_sharetarget(&self) -> &MobileSharetarget<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MobileSharetargetExt<R> for T {
  fn mobile_sharetarget(&self) -> &MobileSharetarget<R> {
    self.state::<MobileSharetarget<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("mobile-sharetarget")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let mobile_sharetarget = mobile::init(app, api)?;
      #[cfg(desktop)]
      let mobile_sharetarget = desktop::init(app, api)?;
      app.manage(mobile_sharetarget);
      Ok(())
    })
    .build()
}
