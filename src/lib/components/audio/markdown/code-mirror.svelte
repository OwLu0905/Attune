<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { basicSetup } from "codemirror";
    import { EditorView } from "@codemirror/view";
    import { EditorState } from "@codemirror/state";
    import { markdown } from "@codemirror/lang-markdown";
    import { vim } from "@replit/codemirror-vim";
    import { marked } from "marked";
    import * as Tabs from "$lib/components/ui/tabs";
    import { createDir, getFile, saveFile } from "@/file";

    interface Props {
        value?: string;
        audioId?: string;
    }

    let {
        value = $bindable(
            "# Start Document\n\nWrite your markdown here...\n\n## Section 2\n\nThis is a **paragraph** with some formatting.\n\n### Subsection\n\n- List item 1\n- List item 2\n\n`inline code` and [links](https://example.com)",
        ),
        audioId,
    }: Props = $props();

    let ref: HTMLElement;
    let view = $state<EditorView>();
    let activeTab = $state("code");
    let isSaved = $state(true);
    let saveTimeout: number;
    let dirCreated = $state(false);

    // Custom CodeMirror theme using shadcn colors
    // const customTheme = EditorView.theme(
    // {
    //     "&": {
    //         color: "var(--foreground)",
    //         backgroundColor: "var(--background)",
    //     },
    //     ".cm-content": {
    //         padding: "16px",
    //         caretColor: "var(--foreground)",
    //         fontFamily: "var(--font-mono)",
    //         fontSize: "14px",
    //         lineHeight: "1.6",
    //     },
    //     ".cm-focused": {
    //         outline: "2px solid var(--ring)",
    //         outlineOffset: "2px",
    //     },
    //     ".cm-editor": {
    //         borderRadius: "calc(var(--radius) - 2px)",
    //         border: "1px solid var(--border)",
    //     },
    //     ".cm-gutters": {
    //         backgroundColor: "var(--muted)",
    //         color: "var(--muted-foreground)",
    //         border: "none",
    //         borderTopLeftRadius: "calc(var(--radius) - 2px)",
    //         borderBottomLeftRadius: "calc(var(--radius) - 2px)",
    //         opacity: "1",
    //     },
    //     ".cm-activeLineGutter": {
    //         backgroundColor: "var(--accent)",
    //     },
    //     ".cm-lineNumbers .cm-gutterElement": {
    //         color: "var(--muted-foreground)",
    //         fontSize: "13px",
    //         backgroundColor: "var(--muted)",
    //         opacity: "1",
    //     },
    //     ".cm-activeLine": {
    //         backgroundColor: "oklch(from var(--accent) l c h / 0.1)",
    //     },
    //     ".cm-selectionBackground, ::selection": {
    //         backgroundColor: "oklch(from var(--primary) l c h / 0.2)",
    //     },
    //     ".cm-searchMatch": {
    //         backgroundColor: "oklch(from var(--primary) l c h / 0.3)",
    //     },
    //     ".cm-cursor, .cm-dropCursor": {
    //         borderLeftColor: "var(--foreground)",
    //     },
    // },
    //     {},
    //     { dark: true },
    // );

    // Debounced save function
    const debouncedSave = (cb: () => void, delay = 600) => {
        if (saveTimeout) {
            clearTimeout(saveTimeout);
        }
        isSaved = false;
        saveTimeout = setTimeout(cb, delay);
    };

    // Load markdown from file
    async function loadMarkdown() {
        if (!audioId) return;

        try {
            const dataPath = `${audioId}/note`;
            const file = await getFile(dataPath, "md");

            if (file) {
                const decoder = new TextDecoder("utf-8");
                const content = decoder.decode(file);
                value = content;
            }
        } catch (error) {
            console.error("Error loading markdown:", error);
        }
    }

    // Store markdown to file
    async function storeMarkdown(content: string, audioId: string) {
        try {
            // Only create directory once per session
            if (!dirCreated) {
                await createDir(audioId);
                dirCreated = true;
            }

            const encoder = new TextEncoder();
            const file = encoder.encode(content);
            const filename = `${audioId}/note`;
            await saveFile(file, filename, "md");
            isSaved = true;

            setTimeout(() => {
                isSaved = true;
            }, 1000);
        } catch (error) {
            console.error("Error saving markdown:", error);
        }
    }

    onMount(async () => {
        // Load existing markdown content
        await loadMarkdown();

        if (ref) {
            const updateListener = EditorView.updateListener.of((update) => {
                if (update.docChanged) {
                    const content = update.state.doc.toString();
                    value = content;

                    // Auto-save with debounce
                    if (audioId) {
                        debouncedSave(() => storeMarkdown(content, audioId));
                    }
                }
            });

            view = new EditorView({
                doc: value,
                parent: ref,
                extensions: [
                    basicSetup,
                    markdown(),
                    vim(),
                    // customTheme,
                    updateListener,
                    EditorView.lineWrapping,
                ],
            });
        }
    });

    onDestroy(() => {
        if (view) {
            view.destroy();
        }
    });
</script>

<Tabs.Root bind:value={activeTab} class="w-full">
    <Tabs.List class="grid w-full grid-cols-2">
        <Tabs.Trigger value="code" class="relative">
            Code
            {#if audioId && !isSaved}
                <div
                    class="bg-primary absolute -top-1 -right-1 h-2 w-2 rounded-full"
                ></div>
            {/if}
        </Tabs.Trigger>
        <Tabs.Trigger value="markdown" class="relative">
            Markdown
            {#if audioId && !isSaved}
                <div
                    class="bg-primary absolute -top-1 -right-1 h-2 w-2 rounded-full"
                ></div>
            {/if}
        </Tabs.Trigger>
    </Tabs.List>

    <Tabs.Content value="code" class="mt-2">
        <div
            class="ring-border max-h-[600px] min-h-[600px] overflow-auto rounded-lg ring"
        >
            <div bind:this={ref}></div>
        </div>
    </Tabs.Content>

    <Tabs.Content value="markdown" class="mt-2">
        <iframe
            title="Markdown Preview"
            class="markdown-preview bg-background ring-border max-h-[600px] min-h-[600px] w-full overflow-auto rounded-lg px-6 ring"
            srcdoc={marked(value) as string || ""}
        >
            <!-- <style> -->
            <!--     /* Isolated markdown preview styles - using @layer components to override Tailwind */ -->
            <!--     .markdown-preview h1 { -->
            <!--         font-size: 2.25rem; -->
            <!--         line-height: 2.5rem; -->
            <!--         font-weight: 800; -->
            <!--         margin-bottom: 1rem; -->
            <!--         margin-top: 1.5rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--         border-bottom: 1px solid hsl(var(--border)); -->
            <!--         padding-bottom: 0.5rem; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview h2 { -->
            <!--         font-size: 1.875rem; -->
            <!--         line-height: 2.25rem; -->
            <!--         font-weight: 700; -->
            <!--         margin-bottom: 0.875rem; -->
            <!--         margin-top: 1.25rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--         border-bottom: 1px solid hsl(var(--border)); -->
            <!--         padding-bottom: 0.25rem; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview h3 { -->
            <!--         font-size: 1.5rem; -->
            <!--         line-height: 2rem; -->
            <!--         font-weight: 600; -->
            <!--         margin-bottom: 0.75rem; -->
            <!--         margin-top: 1rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview h4 { -->
            <!--         font-size: 1.25rem; -->
            <!--         line-height: 1.75rem; -->
            <!--         font-weight: 600; -->
            <!--         margin-bottom: 0.625rem; -->
            <!--         margin-top: 0.875rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview h5 { -->
            <!--         font-size: 1.125rem; -->
            <!--         line-height: 1.75rem; -->
            <!--         font-weight: 600; -->
            <!--         margin-bottom: 0.5rem; -->
            <!--         margin-top: 0.75rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview h6 { -->
            <!--         font-size: 1rem; -->
            <!--         line-height: 1.5rem; -->
            <!--         font-weight: 600; -->
            <!--         margin-bottom: 0.5rem; -->
            <!--         margin-top: 0.625rem; -->
            <!--         color: hsl(var(--muted-foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview p { -->
            <!--         margin-bottom: 1rem; -->
            <!--         line-height: 1.7; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--         white-space: pre-line; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview ul, -->
            <!--     .markdown-preview ol { -->
            <!--         margin-bottom: 1rem; -->
            <!--         padding-left: 1.5rem; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview li { -->
            <!--         margin-bottom: 0.25rem; -->
            <!--         line-height: 1.6; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview blockquote { -->
            <!--         border-left: 4px solid hsl(var(--primary)); -->
            <!--         padding-left: 1rem; -->
            <!--         margin: 1rem 0; -->
            <!--         font-style: italic; -->
            <!--         color: hsl(var(--muted-foreground)); -->
            <!--         background-color: hsl(var(--muted) / 0.5); -->
            <!--         padding: 0.75rem 1rem; -->
            <!--         border-radius: calc(var(--radius) - 2px); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview code { -->
            <!--         background-color: hsl(var(--muted)); -->
            <!--         color: hsl(var(--foreground)); -->
            <!--         padding: 0.125rem 0.25rem; -->
            <!--         border-radius: 0.25rem; -->
            <!--         font-family: var(--font-mono); -->
            <!--         font-size: 0.875rem; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview pre { -->
            <!--         background-color: hsl(var(--muted)); -->
            <!--         color: hsl(var(--foreground)); -->
            <!--         padding: 1rem; -->
            <!--         border-radius: calc(var(--radius) - 2px); -->
            <!--         overflow-x: auto; -->
            <!--         margin: 1rem 0; -->
            <!--         border: 1px solid hsl(var(--border)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview pre code { -->
            <!--         background: none; -->
            <!--         padding: 0; -->
            <!--         font-size: 0.875rem; -->
            <!--         line-height: 1.5; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview a { -->
            <!--         color: hsl(var(--primary)); -->
            <!--         text-decoration: underline; -->
            <!--         text-underline-offset: 2px; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview a:hover { -->
            <!--         color: hsl(var(--primary) / 0.8); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview table { -->
            <!--         width: 100%; -->
            <!--         border-collapse: collapse; -->
            <!--         margin: 1rem 0; -->
            <!--         border: 1px solid hsl(var(--border)); -->
            <!--         border-radius: calc(var(--radius) - 2px); -->
            <!--         overflow: hidden; -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview th, -->
            <!--     .markdown-preview td { -->
            <!--         padding: 0.75rem; -->
            <!--         text-align: left; -->
            <!--         border-bottom: 1px solid hsl(var(--border)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview th { -->
            <!--         background-color: hsl(var(--muted)); -->
            <!--         font-weight: 600; -->
            <!--         color: hsl(var(--foreground)); -->
            <!--     } -->
            <!---->
            <!--     .markdown-preview hr { -->
            <!--         border: none; -->
            <!--         height: 1px; -->
            <!--         background-color: hsl(var(--border)); -->
            <!--         margin: 2rem 0; -->
            <!--     } -->
            <!-- </style> -->
            {@html marked(value) as string}
        </iframe>
    </Tabs.Content>
</Tabs.Root>
