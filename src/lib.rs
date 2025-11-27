#[cfg(target_os = "android")]
use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};
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
        .invoke_handler(tauri::generate_handler![])
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
#[no_mangle]
pub extern "system" fn Java_com_plugin_mobilesharetarget_Sharetarget_helloWorld(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
) -> jstring {
    println!("Calling JNI Hello World!");

    let input: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();

    let result = format!("Hello, {}!", input);

    match env.new_string(result) {
        Ok(jstr) => jstr.into_raw(),
        Err(e) => {
            eprintln!("Failed to create JString: {}", e);
            std::ptr::null_mut()
        }
    }
}
