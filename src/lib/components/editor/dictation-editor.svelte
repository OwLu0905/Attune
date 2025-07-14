<script lang="ts">
    import { onMount } from "svelte";
    import type { Readable } from "svelte/store";
    import StarterKit from "@tiptap/starter-kit";
    import { cn } from "@/utils";
    import {
        Editor,
        EditorContent,
        BubbleMenu,
        createEditor,
    } from "svelte-tiptap";
    import Color from "@tiptap/extension-color";
    import TextStyle from "@tiptap/extension-text-style";
    import {
        ChevronLeft,
        ChevronRight,
        Eye,
        Pause,
        Play,
        RotateCcw,
        Save,
    } from "@lucide/svelte";
    import Badge from "../ui/badge/badge.svelte";
    import {
        createDir,
        decodeJSON,
        encodeJSON,
        getFile,
        saveFile,
    } from "@/file";
    import type { Content } from "@tiptap/core";
    import { fade } from "svelte/transition";

    import type { SubtitleSegment } from "../audio/types";
    import type { AudioPlayer } from "../audio/audio-player.svelte";
    import Button from "../ui/button/button.svelte";

    interface Props {
        audioId: string;
        index: number;
        length: number;
        dictationItem: SubtitleSegment;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
    }

    let {
        audioId,
        length,
        index = $bindable(),
        audioPlayer,
        dictationItem,
        onPause,
        onPlaySection,
    }: Props = $props();

    let currentTime = $derived(audioPlayer.currentTime);

    let isInbound = $derived.by(() => {
        if (dictationItem) {
            return (
                currentTime > dictationItem.start &&
                currentTime <= dictationItem.end
            );
        }
        return false;
    });

    let editor = $state() as Readable<Editor>;

    async function load() {
        const dataPath = `${audioId}/${index}/answer`;
        const file = await getFile(dataPath, "json");

        let data: Content | string = "";
        if (file) {
            data = decodeJSON<Content>(file);
        }

        editor = createEditor({
            extensions: [StarterKit, Color, TextStyle],
            content: data,
            editorProps: {
                attributes: {
                    class: "transitino-all duration-150 rounded-md focus:outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 border-1 border-input p-3 overflow-auto h-40",
                },
            },
        });
    }

    const toggleBold = () => {
        $editor.chain().focus().toggleBold().run();
    };
    const toggleStrike = () => {
        $editor.chain().focus().toggleStrike().run();
    };

    const toggleItalic = () => {
        $editor.chain().focus().toggleItalic().run();
    };
    const toggleRed = () => {
        if (isColor()) {
            $editor.chain().focus().unsetColor().run();
            return;
        }

        $editor.chain().focus().setColor("red").run();
    };
    const isColor = () => {
        return $editor.isActive("textStyle", { color: "red" });
    };

    const isActive = (name: string, attrs = {}) =>
        $editor.isActive(name, attrs);

    async function storeAnswer(data: {}, id: string, index: number) {
        const dirPath = `${id}/${index}`;

        await createDir(dirPath);
        const file = await encodeJSON(data);
        const filename = `${id}/${index}/answer`;
        await saveFile(file, filename, "json");

        // TODO: invoke create dictation_item
    }

    function onPlay() {
        if (!dictationItem) return;

        // Handle floating-point precision: if currentTime is very close to item.end, start from beginning
        const epsilon = 0.01; // 100ms tolerance
        const start =
            Math.abs(currentTime - dictationItem.end) < epsilon ||
            currentTime >= dictationItem.end
                ? dictationItem.start
                : currentTime;

        onPlaySection(start, dictationItem.end);
    }
</script>

<div class="flex items-center justify-between">
    <h5 class="my-2 font-bold">Dictation</h5>

    <Badge variant="secondary" class="text-primary tabular-nums">
        <div in:fade>{index} / {length - 1}</div>
    </Badge>
</div>

{#await load() then _}
    {#if editor}
        <BubbleMenu
            editor={$editor}
            class="outline-foreground/20 rounded-lg bg-white px-2 py-1.5 text-sm shadow-lg outline"
        >
            <div class="flex gap-1">
                <button
                    class={cn(
                        "rounded-sm px-2",
                        isActive("bold") &&
                            "bg-primary text-primary-foreground",
                    )}
                    type="button"
                    onclick={toggleBold}
                >
                    bold
                </button>
                <button
                    class={cn(
                        "rounded-sm px-2",
                        isActive("strike") &&
                            "bg-primary text-primary-foreground",
                    )}
                    type="button"
                    onclick={toggleStrike}
                >
                    S
                </button>
                <button
                    class={cn(
                        "rounded-sm px-2",
                        isActive("italic") &&
                            "bg-primary text-primary-foreground",
                    )}
                    type="button"
                    onclick={toggleItalic}
                >
                    italic
                </button>
                <button
                    class={cn(
                        "rounded-sm px-2",
                        isColor() && "bg-primary text-primary-foreground",
                    )}
                    type="button"
                    onclick={toggleRed}
                >
                    red
                </button>
            </div>
        </BubbleMenu>
    {/if}

    <EditorContent editor={$editor} class="border-none" />

    <div class="mt-2 flex items-center justify-evenly gap-1">
        <div class="flex w-full items-center gap-1">
            <Button
                class="mr-auto"
                variant="ghost"
                size="sm"
                onclick={() => {
                    storeAnswer(
                        $state.snapshot($editor.getJSON()),
                        audioId,
                        index,
                    );
                }}
            >
                <Eye class="h-6 w-6" />
            </Button>
            <div class="mx-auto flex items-center gap-2.5">
                <Button
                    size="sm"
                    variant="secondary"
                    onclick={() => {
                        if (index === 0) {
                            index = length - 1;
                        } else {
                            index--;
                        }
                    }}
                >
                    <ChevronLeft />
                </Button>
                <Button
                    size="sm"
                    tabindex={0}
                    onclick={() => {
                        if (audioPlayer?.isPlaying && isInbound) {
                            onPause();
                        } else {
                            onPlay();
                        }
                    }}
                >
                    {#if audioPlayer?.isPlaying && isInbound}
                        <Pause class="h-6 w-6" />
                    {:else}
                        <Play class="h-6 w-6" />
                    {/if}
                </Button>
                <Button
                    size="sm"
                    variant="outline"
                    onclick={() => {
                        onPlay();
                    }}
                >
                    <RotateCcw class="h-6 w-6" />
                </Button>
                <Button
                    size="sm"
                    variant="secondary"
                    onclick={() => {
                        if (index === length - 1) {
                            index = 0;
                        } else {
                            index++;
                        }
                    }}
                >
                    <ChevronRight />
                </Button>
            </div>
            <Button
                class="ml-auto"
                variant="outline"
                size="sm"
                onclick={() => {
                    storeAnswer(
                        $state.snapshot($editor.getJSON()),
                        audioId,
                        index,
                    );
                }}
            >
                <Save class="stroke-primary h-6 w-6" />
            </Button>
        </div>
    </div>
{/await}
