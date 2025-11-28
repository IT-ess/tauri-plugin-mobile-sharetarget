const COMMANDS: &[&str] = &["pop_intent_queue", "pop_intent_queue_and_extract_text"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
