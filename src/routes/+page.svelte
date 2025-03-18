<script lang="ts">
	import Button from "$lib/components/ui/button/button.svelte";
	import { Input } from "$lib/components/ui/input";
	import { invoke } from "@tauri-apps/api/core";
	import { toast } from "svelte-sonner";

	let name = $state("");
	let greetMsg = $state("");

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke("greet", { name });
	}
</script>

<main class="container">
	<h2 class="text-xl text-primary font-bold py-4">Welcome to Tauri + Svelte</h2>

	<form class="row" onsubmit={greet}>
		<div class="flex gap-4">
			<Input id="greet-input" placeholder="Enter a name..." bind:value={name} />
			<Button type="submit">Greet</Button>
		</div>
	</form>
	<p>{greetMsg}</p>

	<Button
		class="my-4"
		variant="secondary"
		onclick={() =>
			toast.success("Event has been created", {
				description: "Sunday, December 03, 2023 at 9:00 AM",
				action: {
					label: "Undo",
					onClick: () => console.info("Undo"),
				},
			})}
	>
		Show Toast
	</Button>
</main>
