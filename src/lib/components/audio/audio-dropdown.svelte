<script lang="ts">
    import * as DropdownMenu from "@/components/ui/dropdown-menu";
    import { Languages, Bot, LoaderCircle, Settings } from "@lucide/svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import TranscribeDialog from "./transcribe-dialog.svelte";

    interface Props {
        isTranscribed: boolean;
        isTranscribing: boolean;
        getSubtitle: () => Promise<void>;
        open: boolean;
        initialPrompt: string;
    }

    let {
        isTranscribed,
        isTranscribing,
        getSubtitle,
        open = $bindable(),
        initialPrompt = $bindable(),
    }: Props = $props();
</script>

{#if isTranscribing}
    <Button disabled={true}>
        <LoaderCircle class="mx-auto animate-spin" />
        Get Transcribe
    </Button>
{:else if !isTranscribed}
    <TranscribeDialog
        bind:open
        bind:initialPrompt
        onSubmit={getSubtitle}
        disabled={isTranscribing}
    >
        {#snippet trigger()}
            <Button disabled={isTranscribing}>
                {#if isTranscribing}
                    <LoaderCircle class="mx-auto animate-spin" />
                {:else}
                    <Bot />
                {/if}
                Get Transcribe
            </Button>
        {/snippet}
    </TranscribeDialog>
{:else}
    <DropdownMenu.Root>
        <DropdownMenu.Trigger
            class="transition-transform duration-200 hover:rotate-30 data-[state=open]:rotate-30"
        >
            <Settings size={20} />
        </DropdownMenu.Trigger>
        <DropdownMenu.Content side="left" align="start">
            <DropdownMenu.Group>
                <DropdownMenu.Item
                    disabled={isTranscribing}
                    onclick={() => {
                        open = true;
                    }}
                >
                    {#snippet child({ props })}
                        <span {...props}>
                            <Languages />
                            Transcribe
                        </span>
                    {/snippet}
                </DropdownMenu.Item>
            </DropdownMenu.Group>
        </DropdownMenu.Content>
    </DropdownMenu.Root>

    <TranscribeDialog
        bind:open
        bind:initialPrompt
        onSubmit={getSubtitle}
        disabled={isTranscribing}
    />
{/if}
