<script lang="ts">
    import { cn } from "$lib/utils";

    interface Props {
        isCurrentLine: boolean;
        isPlaying: boolean;
        className?: string;
    }
    let { isCurrentLine, isPlaying, className }: Props = $props();
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={cn(
        "text-primary h-4 w-4 stroke-2 leading-8 opacity-0",
        "audio-lines",
        isCurrentLine ? "opacity-100" : "",
        isPlaying && "playing",
        "transition-all duration-300 ease-in",
        className,
    )}
>
    <path d="M2 10v3" />
    <path d="M6 6v11" />
    <path d="M10 3v18" />
    <path d="M14 8v7" />
    <path d="M18 5v13" />
    <path d="M22 10v3" />
</svg>

<style>
    .audio-lines {
        transition: all 0.3s ease;
    }

    .audio-lines.playing path {
        animation: audioWave 1s ease-in-out infinite alternate;
        transform-origin: center bottom;
    }

    .audio-lines.playing path:nth-child(1) {
        animation-delay: 0s;
    }
    .audio-lines.playing path:nth-child(2) {
        animation-delay: 0.1;
    }
    .audio-lines.playing path:nth-child(3) {
        animation-delay: 0.2s;
    }
    .audio-lines.playing path:nth-child(4) {
        animation-delay: 0.1s;
    }
    .audio-lines.playing path:nth-child(5) {
        animation-delay: 0.4s;
    }
    .audio-lines.playing path:nth-child(6) {
        animation-delay: 0.2s;
    }

    @keyframes audioWave {
        0% {
            opacity: 0.6;
            transform: scaleY(0.6);
            transform-origin: center;
        }
        50% {
            opacity: 0.8;
            transform: scaleY(1.1);

            transform-origin: center;
        }
        100% {
            opacity: 1;
            transform: scaleY(0.8);

            transform-origin: center;
        }
    }
</style>
