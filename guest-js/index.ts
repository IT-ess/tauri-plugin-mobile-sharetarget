import { invoke } from "@tauri-apps/api/core";

export function getLatestIntent(): Promise<string | null> {
  return invoke<string | null>(
    "plugin:mobile-sharetarget|get_latest_intent",
    {},
  );
}

export function getLatestIntentAndExtractText(): Promise<string | null> {
  return invoke<string | null>(
    "plugin:mobile-sharetarget|get_latest_intent_and_extract_text",
    {},
  );
}
