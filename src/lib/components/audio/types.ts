type SubtitleWord = {
    word: string;
    start: number;
    end: number;
    probability: number;
};

export type SubtitleSegment = {
    start: number;
    end: number;
    text: string;
    words: SubtitleWord[];
};
