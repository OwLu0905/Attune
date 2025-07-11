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
    import { Play, RotateCcw, Save } from "@lucide/svelte";
    import Badge from "../ui/badge/badge.svelte";
    import {
        createDir,
        decodeJSON,
        encodeJSON,
        getFile,
        saveFile,
    } from "@/file";
    import type { Content } from "@tiptap/core";

    interface Props {
        audioId: string;
        index: number;
    }

    let { audioId, index }: Props = $props();
    let editor = $state() as Readable<Editor>;

    async function load() {
        const dataPath = `${audioId}/${index}/answer`;
        const file = await getFile(dataPath, "json");

        if (file) {
            const data = decodeJSON<Content>(file);

            editor = createEditor({
                extensions: [StarterKit, Color, TextStyle],
                content: data,
                editorProps: {
                    attributes: {
                        class: "border-1 border-black p-3 overflow-auto h-40",
                    },
                },
            });
        } else {
            editor = createEditor({
                extensions: [StarterKit, Color, TextStyle],
                content: `<div>hi</div>`,
                editorProps: {
                    attributes: {
                        class: "border-1 border-black p-3 overflow-auto h-40",
                    },
                },
            });
        }
    }
    // onMount(() => {
    //     load();
    // });

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
    }
</script>

<h5 class="my-2 font-bold">Dictation</h5>

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

    <div class="text-xs">
        TODO: add playback to current selected segment, play replay save
    </div>

    <div class="flex items-center justify-evenly gap-1">
        <div>
            <Badge variant="secondary">{index} / 200</Badge>
        </div>

        <div class="ml-auto flex items-center gap-1">
            <Save
                class="h-4 w-4"
                onclick={() => {
                    storeAnswer(
                        $state.snapshot($editor.getJSON()),
                        audioId,
                        index,
                    );
                }}
            />
            <Play class="h-4 w-4" />
            <RotateCcw class="h-4 w-4" />
        </div>
    </div>
{/await}
