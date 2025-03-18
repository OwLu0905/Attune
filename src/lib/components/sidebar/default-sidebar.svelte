<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import NavMain from "$lib/components/nav-main.svelte";
	import NavProjects from "$lib/components/nav-projects.svelte";

	let { data } = $props();
</script>

<Sidebar.Content>
	<NavMain items={data.navMain} />
	<NavProjects projects={data.projects} />
</Sidebar.Content>
