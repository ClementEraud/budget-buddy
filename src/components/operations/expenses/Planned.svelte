<script lang="ts">
    import type { Operations } from "../../../domain/types/operation";
    import AsyncOperationList from "../AsyncOperationList.svelte";
    import "../operations.scss";

    const {
        plannedExpenses,
    }: {
        plannedExpenses: Promise<Operations>;
    } = $props();
</script>

<div class="transactions-container">
    <div class="header">
        <h2>Planned</h2>
    </div>
    <div class="transactions">
        <AsyncOperationList operationsPromise={plannedExpenses} />
    </div>
    <div class="total">
        <h2>Total: 1200€</h2>
    </div>
</div>
