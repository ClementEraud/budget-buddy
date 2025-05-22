<script lang="ts">
    import type { Operations } from "../domain/types/operation";
    import Balance from "./Balance.svelte";
    import OperationsComponent from "./operations/Operations.svelte";

    const {
        incomes,
        expenses,
        title,
        balance,
    }: {
        incomes: Promise<Operations>;
        expenses: Promise<Operations>;
        title: string;
        balance: Promise<number>;
    } = $props();
</script>

<div class="account-container">
    <h2>{title}</h2>

    <div class="balance-container">
        {#await balance then balanceFulfilled}
            <Balance balance={balanceFulfilled} />
        {/await}
    </div>

    <div class="operations">
        <div class="operation-container incomes">
            <OperationsComponent title="Incomes" operations={incomes} />
        </div>
        <div class="operation-container expenses">
            <OperationsComponent title="Expenses" operations={expenses} />
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
