<script lang="ts">
    import * as Select from "@/components/ui/select/";
    import { Button } from "@/components/ui/button/";
    import { Card } from "@/components/ui/card/";
    import { Input } from "@/components/ui/input/";
    import { Label } from "@/components/ui/label/";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAppSettingsContext } from "../../../routes/setting/app-setting-context.svelte";
    import { commands } from "@/tauri";

    let { getUser } = getUserContext();
    let user = getUser();
    const appSettingsApi = getAppSettingsContext();

    const models = [
        { id: 0, label: "base", value: "base" },
        { id: 1, label: "base.en", value: "base.en" },
        { id: 2, label: "small", value: "small" },
        { id: 3, label: "small.en", value: "small.en" },
        { id: 4, label: "medium", value: "medium" },
        { id: 5, label: "medium.en", value: "medium.en" },
    ];

    const languages = [
        { id: 0, label: "English", value: "en" },
        { id: 1, label: "Spanish", value: "es" },
        { id: 2, label: "French", value: "fr" },
        { id: 3, label: "German", value: "de" },
        { id: 4, label: "Chinese", value: "zh" },
        { id: 5, label: "Japanese", value: "ja" },
    ];

    let selectedModel = $derived.by(() => {
        if (!appSettingsApi.appSettings) return "base.en";
        return appSettingsApi.appSettings.selectedModel;
    });

    let selectedLanguage = $derived.by(() => {
        if (!appSettingsApi.appSettings) return "en";
        return appSettingsApi.appSettings.language;
    });

    let modelProxy = $derived.by(() => {
        if (!appSettingsApi.appSettings) return "";
        return appSettingsApi.appSettings.modelProxy;
    });

    let isDirty = $derived.by(() => {
        if (!appSettingsApi.appSettings) return false;

        return (
            selectedModel !== appSettingsApi.appSettings.selectedModel ||
            selectedLanguage !== appSettingsApi.appSettings.language ||
            modelProxy !== (appSettingsApi.appSettings.modelProxy || "")
        );
    });

    const modelTriggerContent = $derived(
        models.find((m) => m.value === selectedModel)?.label ??
            "Select a model",
    );

    const languageTriggerContent = $derived(
        languages.find((l) => l.value === selectedLanguage)?.label ??
            "Select a language",
    );

    async function handleSave(
        event: SubmitEvent & {
            currentTarget: EventTarget & HTMLFormElement;
        },
    ) {
        event.preventDefault();

        if (!user.accessToken) return;

        const result = await commands.handleUpdateAppSettings(
            user.accessToken,
            {
                theme: null,
                selectedModel,
                language: selectedLanguage,
                modelProxy,
                autoLogin: null,
            },
        );
        if (result.status === "error") {
            throw new Error(result.error);
        }

        appSettingsApi.appSettings = result.data;
    }
</script>

<Card class="p-6">
    <form onsubmit={(e) => handleSave(e)} class="space-y-6">
        <div>
            <Label class="text-sm font-medium">Language Model</Label>
            <div class="mt-2">
                <Select.Root
                    type="single"
                    name="langModel"
                    bind:value={selectedModel}
                >
                    <Select.Trigger class="w-full">
                        {modelTriggerContent}
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Group>
                            <Select.GroupHeading
                                >Available Models</Select.GroupHeading
                            >
                            {#each models as model (model.value)}
                                <Select.Item
                                    value={model.value}
                                    label={model.label}
                                >
                                    {model.label}
                                </Select.Item>
                            {/each}
                        </Select.Group>
                    </Select.Content>
                </Select.Root>
            </div>
        </div>
        <div>
            <Label class="text-sm font-medium">Interface Language</Label>
            <div class="mt-2">
                <Select.Root
                    type="single"
                    name="language"
                    bind:value={selectedLanguage}
                >
                    <Select.Trigger class="w-full">
                        {languageTriggerContent}
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Group>
                            <Select.GroupHeading>Languages</Select.GroupHeading>
                            {#each languages as language (language.value)}
                                <Select.Item
                                    value={language.value}
                                    label={language.label}
                                >
                                    {language.label}
                                </Select.Item>
                            {/each}
                        </Select.Group>
                    </Select.Content>
                </Select.Root>
            </div>
        </div>
        <div>
            <Label class="text-sm font-medium">Model Proxy URL</Label>
            <div class="mt-2">
                <Input
                    type="url"
                    name="modelProxy"
                    placeholder="http://localhost:8000 or https://your-model-service.com"
                    bind:value={modelProxy}
                    class="w-full"
                />
            </div>
            <p class="text-muted-foreground mt-1 text-xs">
                Optional: URL for model service proxy (e.g., local or remote
                inference server)
            </p>
        </div>
        {#if isDirty}
            <div class="flex justify-end pt-4">
                <Button type="submit">Save</Button>
            </div>
        {/if}
    </form>
</Card>
