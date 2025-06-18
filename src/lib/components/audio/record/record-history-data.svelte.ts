import { SvelteMap } from "svelte/reactivity";

import {
    deleteRecordItem,
    getRecordHistory,
    getRecordItem,
    getRecordItemMetadata,
} from "@/utils";

export type TRecordItem = {
    name: string;
    data: Uint8Array;
    modifiedTime: Date | null;
};

export type RecordMap = SvelteMap<string, TRecordItem[]>;

export class RecordHistoryData {
    data: RecordMap = new SvelteMap();
    constructor() {}

    async fetchData(audioId: string, questionId: string) {
        try {
            const data = await getRecordHistory(audioId, questionId);
            if (!data) return [];

            const filesPromise = data.map((i) => {
                return getRecordItem(audioId, questionId, i.name);
            });
            const filesMetadataPromise = data.map((i) => {
                return getRecordItemMetadata(audioId, questionId, i.name);
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
            this.data.set(questionId, files);

            return files;
        } catch (error) {
            console.error(error);
            return [];
        }
    }
    async getData(audioId: string, questionId: string): Promise<TRecordItem[]> {
        try {
            if (this.data.get(questionId)) {
                return this.data.get(questionId) ?? [];
            } else {
                return this.fetchData(audioId, questionId);
            }
        } catch (error) {
            console.error(error);
            return [];
        }
    }
    async updateData(audioId: string, questionId: string) {
        return this.fetchData(audioId, questionId);
    }
    async deleteData(audioId: string, questionId: string, filename: string) {
        await deleteRecordItem(audioId, questionId, filename);

        return this.fetchData(audioId, questionId);
    }
}
