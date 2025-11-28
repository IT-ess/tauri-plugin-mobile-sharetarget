use crossbeam_queue::SegQueue;

static INTENTS: SegQueue<String> = SegQueue::new();
static TEXT_INTENT_KEY: &str = "android.intent.extra.TEXT";

pub fn push_new_intent(raw_intent: String) {
    INTENTS.push(raw_intent);
}

pub(crate) fn pop_intent() -> Option<String> {
    INTENTS.pop()
}

pub(crate) fn pop_and_extract_text_intent() -> Option<String> {
    let raw_intent_opt = pop_intent();
    if cfg!(target_os = "ios") {
        // Extract the url from the deep-link
        raw_intent_opt.map(|url| {
            let prefix = format!("{}://share?url=", crate::IOS_DEEP_LINK_SCHEME.wait());
            url.strip_prefix(&prefix)
                .map(|remaining_part| remaining_part.to_string())
                .expect("Shared URLs should respect this prefix")
        })
    } else {
        // Extract the url from the android intent
        raw_intent_opt
            .map(|i| extract_android_intent_extra(&i, TEXT_INTENT_KEY))
            .flatten()
    }
}

fn extract_android_intent_extra(intent_uri: &str, key_name: &str) -> Option<String> {
    // The key in the intent string is usually prefixed with a type identifier
    // 'S' stands for String in Android Intent parsing.
    // We construct the full prefix we are looking for: "S.key_name="
    let search_prefix = format!("S.{}=", key_name);

    // Iterate over the semi-colon separated parts of the string
    for part in intent_uri.split(';') {
        // If this part starts with our target key, return the rest of the string (the value)
        if let Some(value) = part.strip_prefix(&search_prefix) {
            return Some(value.to_string());
        }
    }

    None
}

#[test]
fn test_android_extract() {
    let input = r"#Intent;action=android.intent.action.SEND;type=text\/plain;launchFlags=0x13400000;extendedLaunchFlags=0x4;component=com.tauri.dev\/.MainActivity;S.android.intent.extra.SUBJECT=The%20most%20serious%20conspiracy%20in%20French%20tech;S.android.intent.extra.TEXT=https%3A%2F%2Fyoutube.com%2Fwatch%3Fv%3D2wMxldl3Alk%26si%3Da-EfIBdz_xvHK5Bk;end";
    let result = extract_android_intent_extra(input, TEXT_INTENT_KEY);
    assert_eq!(
        result.unwrap(),
        "https%3A%2F%2Fyoutube.com%2Fwatch%3Fv%3D2wMxldl3Alk%26si%3Da-EfIBdz_xvHK5Bk"
    )
}
