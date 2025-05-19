<script lang="ts">
    import Balance from "../components/Balance.svelte";
    import Header from "../components/Header.svelte";
    import Operations from "../components/Operations.svelte";
    import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<main class="container">
    <Header />

    {#await data.balance then balance}
        <Balance {balance} />
    {/await}

    <Operations
        incomes={data.incomes}
        actualExpenses={data.actualExpenses}
        plannedExpenses={data.plannedExpenses}
    />
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }
</style>
