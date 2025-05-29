<script lang="ts">
    import Header from "../components/Header.svelte";
    import FinancialEntity from "../components/FinancialEntity.svelte";
    import type { PageProps } from "./$types";
    import { Card } from "flowbite-svelte";

    const { data }: PageProps = $props();
</script>

<main class="container mx-auto px-4 flex flex-col h-[90vh]">
    <Card class="my-4">
        <Header />
    </Card>
    <div class="flex flex-row justify-between items-center h-full">
        <FinancialEntity title="Budget" summary={data.currentBudgetSummary} />
        <FinancialEntity title="Account" summary={data.accountSummary} />
    </div>
</main>
