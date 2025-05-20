import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { appLocalDataDir } from "@tauri-apps/api/path";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function simpleFormatSecondsToMMSS(totalSeconds: number) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

// Get the app's local data directory
export async function getAppLocalDataDir() {
    try {
        const path = await appLocalDataDir();
        console.log("App local data directory:", path);
        return path;
    } catch (error) {
        console.error("Error getting app local data directory:", error);
        return null;
    }
}

export async function getAudioSubtitlePath(id: string) {
    try {
        const dir = await appLocalDataDir();
        const audioPath = `${dir}/data/${id}/audio.mp3`;
        const subtitlePath = `${dir}/data/${id}/subtitle.json`;
        return [audioPath, subtitlePath];
    } catch (error) {
        console.error("Error getting app local data directory:", error);
        return null;
    }
}
