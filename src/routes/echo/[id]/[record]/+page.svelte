<script lang="ts">
    import { page } from "$app/state";

    let recordId = $derived(page.params.record);
</script>

<div>
    {recordId}
</div>
