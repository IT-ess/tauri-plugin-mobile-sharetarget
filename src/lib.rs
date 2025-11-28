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
mod intents;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MobileSharetarget;
#[cfg(mobile)]
use mobile::MobileSharetarget;

#[cfg(mobile)]
use crate::intents::push_new_intent;

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
        .invoke_handler(tauri::generate_handler![
            commands::get_latest_intent,
            commands::get_latest_intent_and_extract_text
        ])
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
use std::ffi::{c_char, CStr, CString};

#[cfg(target_os = "ios")]
#[no_mangle]
pub unsafe extern "C" fn push_intent_ffi(c_name: *const c_char) {
    println!("Called hello world !");
    let intent = match CStr::from_ptr(c_name).to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[iOS FFI] Failed to convert C string: {}", e);
            return ();
        }
    };

    push_new_intent(intent.to_string());
}
