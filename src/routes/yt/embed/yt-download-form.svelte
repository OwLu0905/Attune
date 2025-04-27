<script lang="ts">
    import * as Form from "$lib/components/ui/form/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import LoaderCircle from "@lucide/svelte/icons/loader-circle";

    import { ytDltSchema } from "./schema";

    import { superForm, defaults } from "sveltekit-superforms";
    import { zod, zodClient } from "sveltekit-superforms/adapters";

    import type { TSLIDER_VALUES } from "@/components/youtue/types";

    interface Props {
        sliderValues: TSLIDER_VALUES;
    }

    let { sliderValues = $bindable() }: Props = $props();

    async function sleep() {
        return new Promise((res) => setTimeout(res, 5000));
    }
    const form = superForm(defaults(zod(ytDltSchema)), {
        SPA: true,
        validators: zodClient(ytDltSchema),
        async onUpdate({ form }) {
            const _currentSliderValues = $state.snapshot(sliderValues);
            if (form.valid) {
                // TODO: Call an external API with form.data, await the result and update form
                console.log(form.data, _currentSliderValues);
                await sleep();
            }
        },
    });

    const { form: formData, enhance, delayed } = form;

    $effect(() => {
        // NOTE: find better way to bind these values
        $formData.startTime = sliderValues[0];
        $formData.endTime = sliderValues[1];
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
        <Form.Description>This is your public display name.</Form.Description>
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
</form>
