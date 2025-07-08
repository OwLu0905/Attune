export type AudioItem = {
    id: string;
    url: string;
    title: string;
    thumbnail?: string;
    description: string;
    transcribe: 0 | 1;
    lastUsedAt: string;
    exerciseType: string;
};
