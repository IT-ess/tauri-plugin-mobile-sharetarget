import { invoke } from "@tauri-apps/api/core";

/**
 *
 * Pops the first item in the intents queue if there is one, and return it (raw).
 */
export function popIntentQueue(): Promise<string | null> {
  return invoke<string | null>(
    "plugin:mobile-sharetarget|pop_intent_queue",
    {},
  );
}

/**
 *
 * Pops the first item in the intents queue if there is one, and return its parsed payload.
 */
export function popIntentQueueAndExtractText(): Promise<string | null> {
  return invoke<string | null>(
    "plugin:mobile-sharetarget|pop_intent_queue_and_extract_text",
    {},
  );
}
