<script>
    import Incomes from "./operations/Incomes.svelte";
    import Expenses from "./operations/Expenses.svelte";
</script>

<div class="operations">
    <div class="operation-container incomes">
        <Incomes />
    </div>
    <div class="operation-container expenses">
        <Expenses />
    </div>
</div>

<style>
    .operations {
        margin-top: 10%;
        flex-grow: 1;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding-left: 2%;
        padding-right: 2%;

        .operation-container {
            border: 1px solid #514c4c;
            border-radius: 5px;
            height: 100%;
            display: flex;
            flex-direction: column;
        }

        .incomes {
            width: 30%;
        }

        .expenses {
            width: 50%;
        }
    }
</style>
