export type AudioItem = {
    id: string;
    title: string;
    thumbnail?: string;
    description: string;
    transcribe: 0 | 1;
    lastUsedAt: string;
    exerciseType: string;
};
