<script>
    let { children } = $props();
</script>

<div class="bg-background relative flex min-h-screen flex-col">
    {@render children()}
</div>
