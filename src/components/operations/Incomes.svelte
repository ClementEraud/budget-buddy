<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import AsyncOperationList from "./AsyncOperationList.svelte";
    import "./operations.scss";

    const { incomes }: { incomes: Promise<Operations> } = $props();
</script>

<div class="transactions-container">
    <div class="header">
        <h2>Incomes</h2>
    </div>
    <div class="transactions">
        <AsyncOperationList operationsPromise={incomes} />
    </div>
    <div class="total">
        <h2>Total: 2000€</h2>
    </div>
</div>
