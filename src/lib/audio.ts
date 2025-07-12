import { commands } from "./tauri";
import type { AudioListItem } from "./tauri";

export async function getAudioList(token: string): Promise<AudioListItem[]> {
    try {
        const result = await commands.handleGetAudioList(token);
        
        if (result.status === "error") {
            throw new Error(result.error);
        }
        
        return result.data;
    } catch (error) {
        console.error(error);
        return [];
    }
}
