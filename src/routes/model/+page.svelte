<script lang="ts">
    import Button from "@/components/ui/button/button.svelte";
    import Input from "@/components/ui/input/input.svelte";
    import Label from "@/components/ui/label/label.svelte";
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "@/components/ui/select/";

    let value = $state("");
    let ws: WebSocket | null = $state(null);

    let answer = $state("");

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

    onMount(() => {
        ws = new WebSocket("ws://localhost:8017/ws/some_token");

        ws.onmessage = (event) => {
            console.log(event.data);
            if (typeof event.data === "string") {
                answer = event.data;
            }
        };
    });

    function sendMessage() {
        if (!ws) return;
        ws.send(
            JSON.stringify({
                message_type: "transcribe",
                file_name: value,
                message: "hello",
            }),
        );
        answer = "";
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
</Card.Root>
