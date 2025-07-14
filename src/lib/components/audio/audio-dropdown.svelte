<script lang="ts">
    import * as DropdownMenu from "@/components/ui/dropdown-menu";
    import { Languages, Bot, LoaderCircle, Settings } from "@lucide/svelte";
    import Button from "$lib/components/ui/button/button.svelte";

    interface Props {
        isTranscribed: boolean;
        isTranscribing: boolean;
        getSubtitle: () => Promise<void>;
    }

    let { isTranscribed, isTranscribing, getSubtitle }: Props = $props();
</script>

{#if !isTranscribed}
    <Button
        disabled={isTranscribing}
        onclick={() => {
            getSubtitle();
        }}
    >
        {#if isTranscribing}
            <LoaderCircle class="mx-auto animate-spin" />
        {:else}
            <Bot />
        {/if}
        Get Transcribe
    </Button>
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
                    onclick={() => {
                        // TODO: update subtitle => add alert modal
                        getSubtitle();
                    }}
                    disabled={isTranscribing}
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
{/if}
