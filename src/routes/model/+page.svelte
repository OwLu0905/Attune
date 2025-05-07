<script lang="ts">
    import Button from "@/components/ui/button/button.svelte";
    import Input from "@/components/ui/input/input.svelte";
    import Label from "@/components/ui/label/label.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "@/components/ui/select/";
    import { invoke } from "@tauri-apps/api/core";

    let value = $state("");
    let ws: WebSocket | null = $state(null);

    let answer = $state("");

    let load = $state(false);

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

    async function sendMessage() {
        try {
            await invoke("start_transcribe", {
                file_name: "c27ad6af-05aa-4012-8026-1361f9dfebd1",
                model: "base.en",
            });
        } catch (err) {
            console.log(err);
        }
    }
</script>

<Card.Root class="col-span-12 @5xl:col-span-6">
    <Card.Content class="w-full">
        <div class="flex flex-col gap-2">
            <Label>Send Message</Label>
            <div class="flex gap-2">
                <Input bind:value class="w-[180px]" />
                <Button onclick={sendMessage}>Send</Button>
            </div>
            <div>
                <Select.Root
                    type="single"
                    name="langModel"
                    bind:value={selectedValue}
                >
                    <Select.Trigger class="w-[180px]">
                        {triggerContent}
                    </Select.Trigger>
                    <Select.Content>
                        <Select.Group>
                            <Select.GroupHeading>Models</Select.GroupHeading>
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
    </Card.Content>
    <Card.Footer>
        <!-- <Button -->
        <!--     type="button" -->
        <!--     variant="ghost" -->
        <!--     onclick={async () => { -->
        <!--         const res = await invoke("start_model"); -->
        <!--         console.log(res); -->
        <!--     }}>Startup</Button -->
        <!-- > -->
        <!-- <Button onclick={() => (load = true)}>connect</Button> -->
        <!-- <div>{answer}</div> -->
    </Card.Footer>
</Card.Root>
