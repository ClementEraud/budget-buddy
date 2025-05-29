<script lang="ts">
    import type { Summary } from "../domain/types/summary";
    import Balance from "./Balance.svelte";
    import OperationsComponent from "./operations/Operations.svelte";

    const {
        summary,
        title,
    }: {
        summary: Summary;
        title: string;
    } = $props();
</script>

<div class="account-container">
    <h2>{title}</h2>

    <div class="balance-container">
        <Balance balance={summary.balance} />
    </div>

    <div class="operations">
        <div class="operation-container incomes">
            <OperationsComponent
                title="Incomes"
                operations={summary.incomes}
                total={summary.totalIncome}
            />
        </div>
        <div class="operation-container expenses">
            <OperationsComponent
                title="Expenses"
                operations={summary.expenses}
                total={summary.totalExpense}
            />
        </div>
    </div>
</div>

<style>
    .account-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        width: 50%;
        justify-content: center;
        align-items: center;

        h2 {
            margin-bottom: 30px;
        }

        .balance-container {
            margin-bottom: 30px;
        }

        .operations {
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: space-evenly;
            flex-direction: row;
            flex-grow: 1;

            .operation-container {
                border: 1px solid #514c4c;
                border-radius: 5px;
                height: 100%;
                display: flex;
                flex-direction: column;
            }

            .incomes {
                width: 40%;
            }

            .expenses {
                width: 40%;
            }
        }
    }
</style>
