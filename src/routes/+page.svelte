<script lang="ts">
    import Balance from "../components/Balance.svelte";
    import Header from "../components/Header.svelte";
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<main class="container">
    <Header />

    {#await data.balance then balance}
        <Balance {balance} />
    {/await}

    <div class="financial-entities">
        <FinancialEntity
            title="Budget"
            incomes={data.incomes}
            expenses={data.expenses}
        />
        <FinancialEntity
            title="Account"
            incomes={data.incomes}
            expenses={data.expenses}
        />
    </div>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        height: 100vh;

        .financial-entities {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            padding: 1rem;
        }
    }
</style>
