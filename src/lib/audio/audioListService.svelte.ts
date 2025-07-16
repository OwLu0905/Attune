import { getContext, setContext } from "svelte";
import { getAudioList } from "@/audio";
import type { AudioListItem } from "@/tauri";

export const audioListKey = Symbol("audioList");

export type AudioListContext = {
    audioList: AudioListItem[];
    refreshAudioList: (token: string) => Promise<void>;
    addAudioItem: (item: AudioListItem) => void;
    removeAudioItem: (id: string) => void;
};

export function createAudioListContext(): AudioListContext {
    let audioList: AudioListItem[] = $state.raw([]);

    const refreshAudioList = async (token: string) => {
        try {
            audioList = await getAudioList(token);
            console.log("load");
        } catch (error) {
            console.error("Failed to refresh audio list:", error);
        }
    };

    const addAudioItem = (item: AudioListItem) => {
        audioList = [item, ...audioList];
    };

    const removeAudioItem = (id: string) => {
        audioList = audioList.filter((item) => item.id !== id);
    };

    return {
        get audioList() {
            return audioList;
        },
        refreshAudioList,
        addAudioItem,
        removeAudioItem,
    };
}

export function setAudioListContext(context: AudioListContext) {
    setContext(audioListKey, context);
}

export function getAudioListContext() {
    return getContext(audioListKey) as AudioListContext;
}
