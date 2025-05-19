<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import Actual from "./expenses/Actual.svelte";
    import Planned from "./expenses/Planned.svelte";
    import "./operations.scss";

    const {
        plannedExpenses,
        actualExpenses,
    }: {
        plannedExpenses: Promise<Operations>;
        actualExpenses: Promise<Operations>;
    } = $props();
</script>

<div class="header">
    <h2>Expenses</h2>
</div>
<div class="expense-container">
    <div class="sub-container">
        <Planned {plannedExpenses} />
    </div>
    <div class="sub-container">
        <Actual {actualExpenses} />
    </div>
</div>

<style>
    .expense-container {
        display: flex;
        flex-direction: row;
        flex-grow: 1;

        .sub-container {
            flex-grow: 1;
        }

        .sub-container:first-of-type {
            border-right: 1px solid #514c4c;
        }
    }
</style>
