import {
    readFile,
    stat,
    remove,
    writeFile,
    mkdir,
    BaseDirectory,
} from "@tauri-apps/plugin-fs";

type TFileType = "json";

export function decodeJSON<T>(contents: Uint8Array): T {
    const decoder = new TextDecoder("utf-8");
    const jsonString = decoder.decode(contents);

    // Parse the JSON string
    const jsonData = JSON.parse(jsonString);
    return jsonData;
}

export async function encodeJSON(data: {}) {
    const jsonString = JSON.stringify(data, null, 2); // null, 2 for pretty formatting

    // Encode string to bytes
    const encoder = new TextEncoder();
    const bytes = encoder.encode(jsonString);

    return bytes;
}
/**
 * @description: path will be : `data/${path}`
 */
export async function createDir(path: string) {
    try {
        await mkdir(`data/${path}`, {
            baseDir: BaseDirectory.AppLocalData,
            recursive: true, // This creates parent directories if they don't exist
        });
    } catch (error) {
        console.log("Directory creation info:", error);
    }
}

/**
 * @description: path will be : `data/${path}.${type}`
 */
export async function saveFile(
    data: Uint8Array | ReadableStream<Uint8Array>,
    path: string,
    type: TFileType,
) {
    try {
        await writeFile(`data/${path}.${type}`, data, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.log("File creation info", error);
    }
}

export async function getFile(path: string, type: TFileType) {
    try {
        return await readFile(`data/${path}.${type}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        // console.log(error);
        // console.log("File query info", error);
        return undefined;
    }
}

export async function deleteFile(path: string, type: TFileType) {
    try {
        await remove(`data/${path}.${type}`, {
            baseDir: BaseDirectory.AppLocalData,
        });
    } catch (error) {
        console.log("File deletion info", error);
    }
}
