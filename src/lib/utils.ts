import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { appLocalDataDir } from "@tauri-apps/api/path";

import {
    readFile,
    BaseDirectory,
    readDir,
    stat,
    remove,
} from "@tauri-apps/plugin-fs";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

// Audio playback constants
export const PLAYBACK_BUFFER = 0.25; // Buffer time in seconds for clearer audio playback

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
    ? Omit<T, "children">
    : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
    ref?: U | null;
};

export function simpleFormatSecondsToMMSS(totalSeconds: number) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

type FileType = "m4a" | "mp4";
export async function getAudioFile(
    id: string,
    file: FileType | undefined = "m4a",
): Promise<Uint8Array<ArrayBuffer> | undefined> {
    try {
        return await readFile(`data/${id}/audio.${file}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.error(error);
    }
}

export async function getSubtitleFile(id: string) {
    try {
        const contents = await readFile(`data/${id}/subtitle.json`, {
            baseDir: BaseDirectory.AppLocalData,
        });
        const decoder = new TextDecoder("utf-8");
        const jsonString = decoder.decode(contents);

        // Parse the JSON string
        const jsonData = JSON.parse(jsonString);
        return jsonData;
    } catch (error) {
        console.error(error);
    }
}

export async function getRecordHistory(audioId: string, index: number) {
    try {
        return await readDir(`data/${audioId}/${index}/record`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.error(error);
    }
}

export async function getRecordItem(
    audioId: string,
    index: number,
    filename: string,
) {
    try {
        return await readFile(`data/${audioId}/${index}/record/${filename}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.error(error);
    }
}
export async function getRecordItemMetadata(
    audioId: string,
    index: number,
    filename: string,
) {
    try {
        return await stat(`data/${audioId}/${index}/record/${filename}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.error(error);
    }
}

export async function deleteRecordItem(
    audioId: string,
    index: number,
    filename: string,
) {
    try {
        return await remove(`data/${audioId}/${index}/record/${filename}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.error(error);
    }
}
