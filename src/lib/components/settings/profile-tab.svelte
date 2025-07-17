<script lang="ts">
    import { Button } from "@/components/ui/button/";
    import { Card } from "@/components/ui/card/";
    import { Input } from "@/components/ui/input/";
    import { Label } from "@/components/ui/label/";
    import { commands } from "@/tauri";
    import { getUserContext } from "@/user/userService.svelte";

    // NOTE: we don't let user change the email
    // TODO: if userName is dirty then show the save button, please reference the language-tab.svelte to see how to make it

    let { getUser } = getUserContext();
    let user = getUser();

    let userName = $derived(user.name);
    let userEmail = $derived(user.email);

    let isDirty = $derived.by(() => {
        return userName !== (user.name || "");
    });

    async function handleSave(event: MouseEvent) {
        event?.preventDefault();

        if (!user.accessToken) return;
        if (!userName) return;

        const result = await commands.handleUpdateUserName(
            user.accessToken,
            userName,
        );
        if (result.status === "error") {
            throw new Error(result.error);
        }

        user.name = userName;
    }
</script>

<Card class="p-6">
    <div class="space-y-4">
        <div>
            <Label for="userName" class="text-sm font-medium">Name</Label>
            <Input
                id="userName"
                type="text"
                placeholder="Enter your name"
                bind:value={userName}
                class="mt-2"
            />
        </div>
        <div>
            <Label for="userEmail" class="text-sm font-medium">Email</Label>
            <Input
                id="userEmail"
                type="email"
                placeholder="Enter your email"
                bind:value={userEmail}
                class="mt-2"
            />
        </div>
        {#if isDirty}
            <div class="flex justify-end pt-4">
                <Button onclick={handleSave}>Save</Button>
            </div>
        {/if}
    </div>
</Card>
