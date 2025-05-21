<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import AsyncOperationList from "./AsyncOperationList.svelte";
    import "./operations.scss";

    const {
        title,
        operations,
    }: { title: string; operations: Promise<Operations> } = $props();
</script>

<div class="transactions-container">
    <div class="header">
        <h2>{title}</h2>
    </div>
    <div class="transactions">
        <AsyncOperationList operationsPromise={operations} />
    </div>
    <div class="total">
        {#await operations then operationsFulfilled}
            <h2>
                Total: {operationsFulfilled.reduce(
                    (acc, op) => acc + op.amount,
                    0,
                )}€
            </h2>
        {/await}
    </div>
</div>
