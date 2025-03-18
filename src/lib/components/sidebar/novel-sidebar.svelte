<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
</script>

<Sidebar.Content>
	<Sidebar.Group>
		<Sidebar.GroupLabel>Novel</Sidebar.GroupLabel>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton>
					<a href={"/novel/challote"}>
						<span>Challote's Web</span>
					</a>
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton>
					<a href={"/novel/walk-two-moons"}>
						<span> Walk Two Moons </span>
					</a>
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton>
					<a href={"/novel/the-giver"}>
						<span>The GIver</span>
					</a>
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Group>
</Sidebar.Content>
