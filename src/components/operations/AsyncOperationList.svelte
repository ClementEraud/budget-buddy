<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import OperationList from "./OperationList.svelte";

    const {
        operationsPromise,
    }: {
        operationsPromise: Promise<Operations>;
    } = $props();
</script>

{#await operationsPromise then operations}
    <OperationList {operations} />
{/await}
