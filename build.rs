const COMMANDS: &[&str] = &["get_latest_intent", "get_latest_intent_and_extract_text"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
