<script lang="ts">
    import { toggleMode } from "mode-watcher";
    import Button from "./ui/button/button.svelte";
    import { Sun, Moon } from "@lucide/svelte";
</script>

<Button onclick={toggleMode} variant="outline" size="icon">
    <Sun
        class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 !transition-all dark:scale-0 dark:-rotate-90"
    />
    <Moon
        class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 !transition-all dark:scale-100 dark:rotate-0"
    />
    <span class="sr-only">Toggle theme</span>
</Button>
