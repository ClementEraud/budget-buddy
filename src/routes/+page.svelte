<script lang="ts">
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<div class="mt-10 flex flex-row justify-between items-center h-full">
    <div class="w-1/2 h-full">
        <FinancialEntity title="Budget" summary={data.currentBudgetSummary} />
    </div>

    <div class="w-1/2 h-full">
        <FinancialEntity title="Account" summary={data.accountSummary} />
    </div>
</div>
