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
    import Button from "@/components/ui/button/button.svelte";
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

<div class="group flex w-full items-center gap-2 select-none">
    <div class="flex grow gap-2 px-2 py-0.5">
        <Button
            variant="ghost"
            class="play"
            aria-label={paused ? "play" : "pause"}
            onclick={() => (paused = !paused)}
        >
            {#if paused}
                <PlayIcon size={14} class="text-foreground" />
            {:else}
                <PauseIcon size={14} class="text-foreground" />
            {/if}
        </Button>
        <div class="flex flex-1 items-center justify-center gap-1 text-xs">
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
    <Button
        variant="ghost"
        onclick={() => {
            onDelete();
        }}
        class={cn(
            "bg-background flex shrink-0 items-center justify-center rounded-r-lg",
            "transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]",
        )}
    >
        <X
            class="text-foreground transition-all duration-1000 ease-in"
            size={12}
        />
    </Button>
</div>
