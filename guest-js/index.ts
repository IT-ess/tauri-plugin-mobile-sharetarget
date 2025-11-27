import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:mobile-sharetarget|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export function getLatestIntent(): Promise<string | null> {
  return invoke<string | null>(
    "plugin:mobile-sharetarget|get_latest_intent",
    {},
  );
}
