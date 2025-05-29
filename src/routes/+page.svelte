<script lang="ts">
    import Header from "../components/Header.svelte";
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<main class="container">
    <Header />

    <div class="financial-entities">
        {#await data.accountSummary then accountSummary}
            <FinancialEntity title="Budget" summary={accountSummary} />

            <FinancialEntity title="Account" summary={accountSummary} />
        {/await}
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
