export type TSLIDER_VALUES = [left: number, right: number];
export type YouTubePlayerContext = {
    getPlayer: () => YT.Player | null;
    getReady: () => boolean;
    getError: () => string | null;
};

export type YouTubeSliderContext = {
    getSliderValues: () => TSLIDER_VALUES;
};
