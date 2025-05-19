<script lang="ts">
    import type { Operations } from "../../../domain/types/operation";
    import AsyncOperationList from "../AsyncOperationList.svelte";
    import "../operations.scss";

    const {
        actualExpenses,
    }: {
        actualExpenses: Promise<Operations>;
    } = $props();
</script>

<div class="transactions-container">
    <div class="header">
        <h2>Actual</h2>
    </div>
    <div class="transactions">
        <AsyncOperationList operationsPromise={actualExpenses} />
    </div>
    <div class="total">
        <h2>Total: 300€</h2>
    </div>
</div>
