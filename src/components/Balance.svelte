<script lang="ts">
    let { balance }: { balance: number } = $props();

    const getCicleColorFromBalance = (balance: number) =>
        balance < 0 ? "red" : balance <= 100 ? "yellow" : "green";
</script>

<div class="balance-container">
    <div class="circle {getCicleColorFromBalance(balance)}">
        <div class="balance">
            <h2>Balance</h2>
            <p>{balance} €</p>
        </div>
    </div>
</div>

<style>
    .balance-container {
        display: flex;
        justify-content: center;

        .balance {
            text-align: center;
        }

        .circle {
            width: 150px;
            height: 150px;
            border-radius: 50%;
            border-style: solid;
            border-width: 15px;
            display: flex;
            justify-content: center;
            align-items: center;
        }

        .green {
            border-color: lightgreen;
        }
        .yellow {
            border-color: lightsalmon;
        }
        .red {
            border-color: lightcoral;
        }
    }
</style>
