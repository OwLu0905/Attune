<script lang="ts">
    import { MediaQuery } from "svelte/reactivity";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Drawer from "$lib/components/ui/drawer/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import type { Snippet } from "svelte";
    import Textarea from "../ui/textarea/textarea.svelte";

    interface Props {
        open: boolean;
        initialPrompt: string;
        trigger?: Snippet;
        onSubmit: () => Promise<void>;
        disabled: boolean;
    }

    let {
        open = $bindable(),
        initialPrompt = $bindable(),
        trigger,
        onSubmit,
        disabled,
    }: Props = $props();

    const isDesktop = new MediaQuery("(min-width: 768px)");

    const title = "Transcribe";
    const description =
        "Tell AI more about your video to get better transcription";

    const id = $props.id();
</script>

{#snippet content()}
    <form
        class="grid items-start gap-4"
        onsubmit={async () => {
            await onSubmit();
        }}
    >
        <div class="grid gap-2">
            <Textarea
                id="prompt-{id}"
                bind:value={initialPrompt}
                placeholder="Type initial prompt here"
                class="h-40"
            />
        </div>
        <Button type="submit">Get Transcribe</Button>
    </form>
{/snippet}

{#if isDesktop.current}
    <Dialog.Root bind:open>
        {#if trigger}
            <Dialog.Trigger {disabled}>
                {@render trigger()}
            </Dialog.Trigger>
        {/if}
        <Dialog.Content class="sm:max-w-xl">
            <Dialog.Header>
                <Dialog.Title>{title}</Dialog.Title>
                <Dialog.Description>{description}</Dialog.Description>
            </Dialog.Header>
            {@render content()}
        </Dialog.Content>
    </Dialog.Root>
{:else}
    <Drawer.Root bind:open>
        {#if trigger}
            <Drawer.Trigger
                class={buttonVariants({ variant: "outline" })}
                {disabled}
            >
                {@render trigger()}
            </Drawer.Trigger>
        {/if}
        <Drawer.Content>
            <Drawer.Header class="text-left">
                <Drawer.Title>{title}</Drawer.Title>
                <Drawer.Description>
                    {description}
                </Drawer.Description>
            </Drawer.Header>
            <div class="px-4">
                {@render content()}
            </div>
            <Drawer.Footer class="pt-2">
                <Drawer.Close class={buttonVariants({ variant: "outline" })}
                    >Cancel</Drawer.Close
                >
            </Drawer.Footer>
        </Drawer.Content>
    </Drawer.Root>
{/if}
