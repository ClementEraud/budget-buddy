<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import OperationList from "./OperationList.svelte";
    import "./operations.scss";

    const {
        title,
        operations,
        total,
    }: { title: string; operations: Operations; total: number } = $props();
</script>

<div class="transactions-container">
    <div class="header">
        <h2>{title}</h2>
    </div>
    <div class="transactions">
        <OperationList {operations} />
    </div>
    <div class="total">
        <h2>
            Total: {total}€
        </h2>
    </div>
</div>
