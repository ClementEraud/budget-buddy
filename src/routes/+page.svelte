<script lang="ts">
    import Balance from "../components/Balance.svelte";
    import Header from "../components/Header.svelte";
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<main class="container">
    <Header />

    <div class="financial-entities">
        <FinancialEntity
            title="Budget"
            balance={data.account_balance}
            incomes={data.account_incomes}
            expenses={data.account_expenses}
        />
        <FinancialEntity
            title="Account"
            balance={data.account_balance}
            incomes={data.account_incomes}
            expenses={data.account_expenses}
        />
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
