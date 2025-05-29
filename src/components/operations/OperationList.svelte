<script lang="ts">
    import { List } from "flowbite-svelte";
    import type { Operations } from "../../domain/types/operation";
    import Operation from "./Operation.svelte";

    const { operations }: { operations: Operations } = $props();
</script>

<List class="divide-y divide-gray-200 dark:divide-gray-700">
    {#each operations as operation}
        <Operation {operation} />
    {/each}
</List>
