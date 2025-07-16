<script>
    import * as Select from "@/components/ui/select/";
    import ThemeSwitch from "@/components/theme-switch.svelte";

    const models = [
        { id: 0, label: "base", value: "base" },
        { id: 1, label: "base.en", value: "base.en" },
        { id: 2, label: "small", value: "small" },
        { id: 2, label: "small.en", value: "small.en" },
        { id: 3, label: "medium", value: "medium" },
        { id: 3, label: "medium.en", value: "medium.en" },
    ];

    let selectedValue = $state("base.en");

    const triggerContent = $derived(
        models.find((m) => m.value === selectedValue)?.label ??
            "Select a model",
    );
</script>

<Select.Root type="single" name="langModel" bind:value={selectedValue}>
    <Select.Trigger class="w-[180px]">
        {triggerContent}
    </Select.Trigger>
    <Select.Content>
        <Select.Group>
            <Select.GroupHeading>Models</Select.GroupHeading>
            {#each models as model (model.value)}
                <Select.Item value={model.value} label={model.label}>
                    {model.label}
                </Select.Item>
            {/each}
        </Select.Group>
    </Select.Content>
</Select.Root>

<div class="p-4">
    <ThemeSwitch />
</div>
