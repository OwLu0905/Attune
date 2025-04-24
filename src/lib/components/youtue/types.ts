export type YouTubePlayerContext = {
    getPlayer: () => YT.Player | null;
    getReady: () => boolean;
    getError: () => string | null;
};
