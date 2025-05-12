<script>
    let { children } = $props();
</script>

<div class="relative flex min-h-screen flex-col bg-background">
    {@render children()}
</div>
