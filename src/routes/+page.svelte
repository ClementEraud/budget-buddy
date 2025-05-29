<script lang="ts">
    import Header from "../components/Header.svelte";
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<main class="container">
    <Header />
    <div class="financial-entities">
        <FinancialEntity title="Budget" summary={data.currentBudgetSummary} />
        <FinancialEntity title="Account" summary={data.accountSummary} />
    </div>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        height: 90vh;

        .financial-entities {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            height: 100%;
        }
    }
</style>
