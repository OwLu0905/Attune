<script lang="ts">
    import { onMount, untrack } from "svelte";

    import * as Form from "$lib/components/ui/form/index.js";
    import { Input } from "$lib/components/ui/input/index.js";

    import LoaderCircle from "@lucide/svelte/icons/loader-circle";
    import { CircleHelp, Youtube } from "@lucide/svelte";

    import { ytDlpSchema } from "./schema";

    import { superForm, defaults } from "sveltekit-superforms";
    import { zod, zodClient } from "sveltekit-superforms/adapters";

    import { YtDownloadManager } from "@/components/youtue/yt-download-manager.svelte";

    import type { TSLIDER_VALUES } from "@/components/youtue/types";
    import type { YtOembUrlInfo } from "./types";
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import { toast } from "svelte-sonner";
    interface Props {
        sliderValues: TSLIDER_VALUES;
        urlInfo: YtOembUrlInfo | null;
    }

    let { sliderValues = $bindable(), urlInfo = $bindable() }: Props = $props();

    let { getUser } = getUserContext();

    const user = getUser();

    let yt_download_manager = new YtDownloadManager();
    const form = superForm(defaults(zod(ytDlpSchema)), {
        SPA: true,
        validators: zodClient(ytDlpSchema),
        async onUpdate({ form }) {
            try {
                const urlInfoSnapshot = $state.snapshot(urlInfo);
                if (form.valid) {
                    const data = form.data;
                    await yt_download_manager.handleDownload({
                        start: data.startTime,
                        end: data.endTime,
                        url: data.url,
                    });

                    let audioId = await invoke("handle_create_audio", {
                        token: user.accessToken,
                        title: data.title,
                        description: data.description,
                        url: data.url,
                        thumbnail: urlInfoSnapshot?.embedInfo.thumbnail_url,
                        start_time: data.startTime,
                        end_time: data.endTime,
                        provider: urlInfoSnapshot?.embedInfo.provider_name,
                    });

                    toast.success("Download completed!!", {
                        description: "check",
                        action: {
                            label: "Undo",
                            onClick: () => console.info("Undo"),
                        },
                    });
                }
            } catch (error) {
                console.error(error);
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
