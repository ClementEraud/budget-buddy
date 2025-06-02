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

    <!-- TODO: Handle when there is no account, a simple button should propose to create an account -->
    <div class="mt-10 flex flex-row justify-between items-center h-full">
        <div class={data.accountSummary ? "w-1/2 h-full" : "w-full h-full"}>
            <FinancialEntity
                title="Budget"
                summary={data.currentBudgetSummary}
            />
        </div>

        {#if data.accountSummary}
            <div class="w-1/2 h-full">
                <FinancialEntity
                    title="Account"
                    summary={data.accountSummary}
                />
            </div>
        {/if}
    </div>
</main>
