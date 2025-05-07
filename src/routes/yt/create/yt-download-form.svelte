<script lang="ts">
    import { onMount, untrack } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import * as Form from "$lib/components/ui/form/index.js";
    import { Input } from "$lib/components/ui/input/index.js";

    import LoaderCircle from "@lucide/svelte/icons/loader-circle";
    import { CircleHelp, Youtube } from "@lucide/svelte";

    import { ytDlpSchema } from "./schema";

    import { superForm, defaults } from "sveltekit-superforms";
    import { zod, zodClient } from "sveltekit-superforms/adapters";

    import { YtDownloadManager } from "@/components/youtue/yt-download-manager.svelte";
    import { DOWNLOAD_YT_EVENT } from "@/types/yt-download";

    import type { TSLIDER_VALUES } from "@/components/youtue/types";
    import type { YtOembUrlInfo } from "./types";

    interface Props {
        sliderValues: TSLIDER_VALUES;
        urlInfo: YtOembUrlInfo | null;
    }

    let { sliderValues = $bindable(), urlInfo = $bindable() }: Props = $props();

    let yt_download_manager = new YtDownloadManager();
    const form = superForm(defaults(zod(ytDlpSchema)), {
        SPA: true,
        validators: zodClient(ytDlpSchema),
        async onUpdate({ form }) {
            if (form.valid) {
                // TODO: db handle
                await invoke(DOWNLOAD_YT_EVENT.download_section, {
                    start: form.data.startTime,
                    end: form.data.endTime,
                    url: form.data.url,
                });
            }
        },
    });

    const { form: formData, enhance, delayed } = form;

    onMount(() => {
        yt_download_manager.initialize();

        return () => {
            yt_download_manager.cleanup();
        };
    });

    $effect(() => {
        let start = sliderValues[0];
        let end = sliderValues[1];
        untrack(() => ($formData.startTime = start));
        untrack(() => ($formData.endTime = end));
    });

    $effect(() => {
        if (!urlInfo) {
            form.reset();
            return;
        }

        // NOTE: find better way to bind these values
        untrack(() => ($formData.url = urlInfo.url));
        if (urlInfo?.embedInfo?.provider_name) {
            untrack(() => ($formData.privoder = "YouTube"));
        } else {
            untrack(() => ($formData.privoder = "Custom"));
        }
        if (
            urlInfo?.embedInfo?.title &&
            untrack(() => $formData.title === "")
        ) {
            untrack(() => ($formData.title = urlInfo?.embedInfo?.title));
        }
    });
</script>

<form method="POST" use:enhance class="h-full space-y-4">
    <Form.Field {form} name="title">
        <Form.Control>
            {#snippet children({ props })}
                <Form.Label>Title</Form.Label>
                <Input {...props} bind:value={$formData.title} />
            {/snippet}
        </Form.Control>
        <Form.Description>
            video title : {urlInfo?.embedInfo?.title}
        </Form.Description>
        <Form.FieldErrors />
    </Form.Field>

    <Form.Field {form} name="description">
        <Form.Control>
            {#snippet children({ props })}
                <Form.Label>Description</Form.Label>
                <Input {...props} bind:value={$formData.description} />
            {/snippet}
        </Form.Control>
        <Form.FieldErrors />
    </Form.Field>

    <div class="flex w-full gap-2">
        <Form.Field {form} name="url" class="flex-1">
            <Form.Control>
                {#snippet children({ props })}
                    <Form.Label>Source</Form.Label>
                    <div class="relative">
                        <Input
                            {...props}
                            bind:value={$formData.url}
                            class="truncate pr-10"
                            disabled={true}
                        />

                        <div
                            class="absolute bottom-0 right-0 top-0 mx-2 flex items-center text-primary"
                        >
                            {#if $formData.privoder === "YouTube"}
                                <Youtube />
                            {:else}
                                <CircleHelp />
                            {/if}
                        </div>
                    </div>
                {/snippet}
            </Form.Control>

            <Form.FieldErrors />
        </Form.Field>
    </div>

    <div class="flex gap-2">
        <Form.Field {form} name="startTime" class="flex-1">
            <Form.Control>
                {#snippet children({ props })}
                    <Form.Label>Start Time (sec)</Form.Label>
                    <Input
                        {...props}
                        bind:value={$formData.startTime}
                        disabled={true}
                    />
                {/snippet}
            </Form.Control>

            <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="endTime" class="flex-1">
            <Form.Control>
                {#snippet children({ props })}
                    <Form.Label>End Time (sec)</Form.Label>
                    <Input
                        {...props}
                        bind:value={$formData.endTime}
                        disabled={true}
                    />
                {/snippet}
            </Form.Control>

            <Form.FieldErrors />
        </Form.Field>
    </div>
    <Form.Button disabled={$delayed} class="mt-auto">
        {#if $delayed}
            <LoaderCircle class="animate-spin" />
        {/if}

        Submit
    </Form.Button>
    <div class="truncate">{yt_download_manager.getMessage}</div>
</form>
