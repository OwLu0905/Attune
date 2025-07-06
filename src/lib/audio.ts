import { invoke } from "@tauri-apps/api/core";
import type { AudioItem } from "./types/audio";

export async function getAudioList(token: string): Promise<AudioItem[]> {
    try {
        return await invoke("handle_get_audio_list", {
            token: token,
        });
    } catch (error) {
        console.error(error);
        return [];
    }
}
