<script lang="ts">
    import type { Summary } from "../domain/types/summary";
    import Balance from "./Balance.svelte";
    import OperationsComponent from "./operations/Operations.svelte";
    import { Heading } from "flowbite-svelte";

    const {
        summary,
        title,
    }: {
        summary: Summary;
        title: string;
    } = $props();
</script>

<div class="h-full flex flex-col justify-center items-center">
    <Heading tag="h2" class="text-primary-300 dark:text-primary-300 mb-8"
        >{title}</Heading
    >

    <div class="mb-8">
        <Balance balance={summary.balance} />
    </div>

    <div class="w-full h-full flex justify-evenly flex-row flex-grow">
        <div class="w-2/5 flex flex-row justify-center">
            <OperationsComponent
                title="Incomes"
                operations={summary.incomes}
                total={summary.totalIncome}
            />
        </div>
        <div class="w-2/5 flex flex-row justify-center">
            <OperationsComponent
                title="Expenses"
                operations={summary.expenses}
                total={summary.totalExpense}
            />
        </div>
    </div>
</div>
