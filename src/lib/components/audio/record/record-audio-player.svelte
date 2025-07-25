<script module>
    let current: HTMLAudioElement | undefined = undefined;

    export function _stopAll() {
        current?.pause();
    }
</script>

<script lang="ts">
    import { PauseIcon, PlayIcon, Trash, X } from "@lucide/svelte";
    import { Progress } from "@/components/ui/progress";
    import { cn } from "@/utils";
    let { src, title, onDelete } = $props();
    let time = $state(0);
    let duration = $state(0);
    let paused = $state(true);

    function format(time: number) {
        if (isNaN(time)) return "...";

        const minutes = Math.floor(time / 60);
        const seconds = Math.floor(time % 60);

        return `${minutes}:${seconds < 10 ? `0${seconds}` : seconds}`;
    }
</script>

<audio
    {src}
    bind:currentTime={time}
    bind:duration
    bind:paused
    onplay={(e) => {
        const audio = e.currentTarget;

        if (audio !== current) {
            current?.pause();
            current = audio;
        }
    }}
    onended={() => {
        time = 0;
    }}
></audio>

<div class="group flex gap-2 select-none">
    <div class="flex grow gap-2 px-4 py-2.5">
        <button
            class="play"
            aria-label={paused ? "play" : "pause"}
            onclick={() => (paused = !paused)}
        >
            {#if paused}
                <PlayIcon size={14} class="text-primary" />
            {:else}
                <PauseIcon size={14} class="text-primary" />
            {/if}
        </button>
        <div class="flex flex-1 items-center justify-center gap-1">
            <span>{format(time)}</span>
            <div
                class="flex-1"
                onpointerdown={(e) => {
                    const div = e.currentTarget;

                    function seek(e: PointerEvent) {
                        const { left, width } = div.getBoundingClientRect();

                        let p = (e.clientX - left) / width;
                        if (p < 0) p = 0;
                        if (p > 1) p = 1;

                        time = p * duration;
                    }

                    seek(e);

                    window.addEventListener("pointermove", seek);

                    window.addEventListener(
                        "pointerup",
                        () => {
                            window.removeEventListener("pointermove", seek);
                        },
                        {
                            once: true,
                        },
                    );
                }}
            >
                <Progress class="progress" value={(time * 100) / duration} />
            </div>

            <span>{duration ? format(duration) : "--:--"}</span>
        </div>
    </div>
    <button
        onclick={() => {
            onDelete();
        }}
        class={cn(
            "bg-secondary flex shrink-0 items-center justify-center rounded-r-lg",
            "transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]",
            "w-0 group-hover:w-3.5 hover:w-8",
            "hover:*:block",
            "opacity-0 group-hover:opacity-100",
        )}
    >
        <X
            class="text-primary hidden transition-all duration-1000 ease-in"
            size={12}
        />
    </button>
</div>
