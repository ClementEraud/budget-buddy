<script lang="ts">
    import type { Operations } from "../domain/types/operation";
    import OperationsComponent from "./operations/Operations.svelte";

    const {
        incomes,
        expenses,
        title,
    }: {
        incomes: Promise<Operations>;
        expenses: Promise<Operations>;
        title: string;
    } = $props();
</script>

<div class="account-container">
    <h2>{title}</h2>
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
        margin-top: 10%;
        height: 100%;
        display: flex;
        flex-direction: column;
        width: 50%;
        justify-content: center;
        align-items: center;

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
