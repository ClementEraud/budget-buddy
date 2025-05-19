<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import Operation from "./Operation.svelte";

    const { operations }: { operations: Operations } = $props();
</script>

<ul>
    {#each operations as operation}
        <Operation {operation} />
    {/each}
</ul>
