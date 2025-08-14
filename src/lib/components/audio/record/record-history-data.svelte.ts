import { SvelteMap } from "svelte/reactivity";

import {
    deleteRecordItem,
    getRecordHistory,
    getRecordItem,
    getRecordItemMetadata,
} from "@/utils";

export type TRecordItem = {
    name: string;
    data: Uint8Array<ArrayBuffer>;
    modifiedTime: Date | null;
};

export type RecordMap = SvelteMap<number, TRecordItem[]>;

export class RecordHistoryData {
    data: RecordMap = new SvelteMap();
    constructor() {}

    async fetchData(audioId: string, dictationId: number) {
        try {
            const data = await getRecordHistory(audioId, dictationId);
            if (!data) return [];

            const filesPromise = data.map((i) => {
                return getRecordItem(audioId, dictationId, i.name);
            });
            const filesMetadataPromise = data.map((i) => {
                return getRecordItemMetadata(audioId, dictationId, i.name);
            });

            const resolveFiles = await Promise.all(filesPromise);
            const resolveFilesMetadata =
                await Promise.all(filesMetadataPromise);

            const files = data.map((i, index) => {
                if (!resolveFiles[index] || !resolveFilesMetadata[index]) {
                    throw new Error("failed");
                }
                return {
                    ...i,
                    data: resolveFiles[index],
                    modifiedTime: resolveFilesMetadata[index].mtime,
                };
            });

            files.sort((a, b) => {
                if (!b.modifiedTime && !a.modifiedTime) {
                    return 0; // Both null, maintain relative order
                }
                if (!b.modifiedTime) {
                    return -1; // b is null, a comes first
                }
                if (!a.modifiedTime) {
                    return 1; // a is null, b comes first
                }

                const timeA =
                    a.modifiedTime instanceof Date
                        ? a.modifiedTime.getTime()
                        : a.modifiedTime;
                const timeB =
                    b.modifiedTime instanceof Date
                        ? b.modifiedTime.getTime()
                        : b.modifiedTime;

                return timeB - timeA; // Sort most recent first
            });
            this.data.set(dictationId, files);

            return files;
        } catch (error) {
            console.error(error);
            return [];
        }
    }
    async getData(
        audioId: string,
        dictationId: number,
    ): Promise<TRecordItem[]> {
        try {
            if (this.data.get(dictationId)) {
                return this.data.get(dictationId) ?? [];
            } else {
                return this.fetchData(audioId, dictationId);
            }
        } catch (error) {
            console.error(error);
            return [];
        }
    }
    async updateData(audioId: string, dictationId: number) {
        return this.fetchData(audioId, dictationId);
    }
    async deleteData(audioId: string, dictationId: number, filename: string) {
        await deleteRecordItem(audioId, dictationId, filename);

        return this.fetchData(audioId, dictationId);
    }
}
