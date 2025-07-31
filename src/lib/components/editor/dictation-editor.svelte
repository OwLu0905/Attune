<script lang="ts">
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
        SquareCheckBig,
    } from "@lucide/svelte";
    import Badge from "../ui/badge/badge.svelte";
    import {
        createDir,
        decodeJSON,
        encodeJSON,
        getFile,
        saveFile,
    } from "@/file";
    import type { Content, JSONContent } from "@tiptap/core";
    import { fade } from "svelte/transition";

    import type { SubtitleSegment } from "../audio/types";
    import type { AudioPlayer } from "../audio/audio-player.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { getUserContext } from "@/user/userService.svelte";
    import { commands, type BookmarkDictationView } from "$lib/tauri";

    interface Props {
        audioId: string;
        dictationId: number;
        combinedList: BookmarkDictationView[];
        length: number;
        dictationItem: SubtitleSegment;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
    }

    let {
        audioId,
        dictationId = $bindable(),
        combinedList = $bindable(),
        length,
        dictationItem,
        audioPlayer,
        onPause,
        onPlaySection,
    }: Props = $props();

    const { getUser } = getUserContext();
    const user = getUser();

    let currentTime = $derived(audioPlayer.currentTime);

    let editor = $state() as Readable<Editor>;

    let dictationState = $derived.by(() => {
        return (
            combinedList.find((i) => i.dictationPosition === dictationId) ??
            undefined
        );
    });
    let saveTimeoutId: number | undefined = undefined;
    let isSaved = $state(false);

    const debouncedSave = (cb: () => void, delay = 600) => {
        if (saveTimeoutId) {
            clearTimeout(saveTimeoutId);
        }

        saveTimeoutId = setTimeout(() => cb(), delay);
    };

    async function load() {
        const dataPath = `${audioId}/${dictationId}/answer`;
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
                    class: "relative transitino-all duration-150 rounded-md focus:outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 border-1 border-input p-3 overflow-auto h-40",
                },
            },
            onUpdate(props) {
                const data = props.editor.getJSON();
                debouncedSave(() => storeAnswer(data, audioId, dictationId));
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

    async function storeAnswer(
        data: JSONContent,
        id: string,
        dictationId: number,
    ) {
        const dirPath = `${id}/${dictationId}`;

        await createDir(dirPath);
        const file = await encodeJSON(data);
        const filename = `${id}/${dictationId}/answer`;
        await saveFile(file, filename, "json");
        isSaved = true;

        setTimeout(() => {
            isSaved = false;
        }, 1000);
    }

    async function saveAsCompleted(id: string, dictationId: number) {
        // Create dictation item
        if (user?.accessToken) {
            try {
                const result = await commands.handleCreateDictationItem(
                    user.accessToken,
                    id,
                    dictationId,
                );

                if (result.status === "error") {
                    throw new Error(result.error);
                }
                combinedList = result.data;

                // Success - dictation item created
                console.log("Dictation item created successfully");
            } catch (error) {
                console.error("Failed to create dictation item:", error);
            }
        }
    }

    function onPlay() {
        if (!dictationItem) return;

        // Handle floating-point precision: if currentTime is very close to item.end, start from beginning
        const epsilon = 0.01; // 10ms tolerance

        let start = dictationItem.start;
        let end = dictationItem.end - epsilon;
        if (currentTime >= start && currentTime <= end) {
            start = currentTime;
        } else {
            start = Math.max(dictationItem.start - epsilon, 0);
        }

        onPlaySection(start, dictationItem.end);
    }
</script>

<div class="flex items-center justify-between">
    <h5 class="my-2 font-bold">Dictation</h5>

    <Badge variant="secondary" class="text-primary tabular-nums">
        <span in:fade>{dictationId} </span>/ {length - 1}
    </Badge>
</div>

{#await load() then _}
    {#if editor}
        <BubbleMenu
            editor={$editor}
            class="outline-foreground/20 bg-background rounded-lg px-2 py-1.5 text-sm shadow-lg outline"
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

    <div class="relative">
        <EditorContent editor={$editor} class="" />

        <!-- {#if isSaved} -->
        <!--     <div in:fade out:fade class="absolute right-2 bottom-2"> -->
        <!--         <Badge variant="outline" class="">Save</Badge> -->
        <!--     </div> -->
        <!-- {/if} -->
    </div>

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
                        dictationId,
                    );
                }}
            >
                <Eye class="h-6 w-6" />
            </Button>
            <div class="mx-auto flex items-center gap-2.5">
                <Button
                    size="sm"
                    variant="outline"
                    onclick={() => {
                        onPause();
                        if (dictationId === 0) {
                            dictationId = length - 1;
                        } else {
                            dictationId--;
                        }
                    }}
                >
                    <ChevronLeft />
                </Button>
                <Button
                    size="sm"
                    tabindex={0}
                    onclick={() => {
                        if (audioPlayer?.isPlaying) {
                            onPause();
                        } else {
                            onPlay();
                        }
                    }}
                >
                    {#if audioPlayer?.isPlaying}
                        <Pause class="h-6 w-6" />
                    {:else}
                        <Play class="h-6 w-6" />
                    {/if}
                </Button>
                <Button
                    size="sm"
                    variant="outline"
                    onclick={() => {
                        const epsilon = 0.1;
                        const start = Math.max(
                            dictationItem.start - epsilon,
                            0,
                        );
                        onPlaySection(start, dictationItem.end);
                    }}
                >
                    <RotateCcw class="h-6 w-6" />
                </Button>
                <Button
                    size="sm"
                    variant="outline"
                    onclick={() => {
                        onPause();
                        if (dictationId === length - 1) {
                            dictationId = 0;
                        } else {
                            dictationId++;
                        }
                    }}
                >
                    <ChevronRight />
                </Button>
            </div>
            {#if !dictationState}
                <Button
                    class="ml-auto border-1"
                    variant="default"
                    size="sm"
                    onclick={async () => {
                        await storeAnswer(
                            $state.snapshot($editor.getJSON()),
                            audioId,
                            dictationId,
                        );
                        await saveAsCompleted(audioId, dictationId);
                    }}
                >
                    <SquareCheckBig class="h-6 w-6" />
                </Button>
            {:else}
                <Button
                    class="ml-auto"
                    variant="outline"
                    size="sm"
                    onclick={async () => {
                        await storeAnswer(
                            $state.snapshot($editor.getJSON()),
                            audioId,
                            dictationId,
                        );
                    }}
                >
                    <Save class="h-6 w-6" />
                </Button>
            {/if}
        </div>
    </div>
{/await}
