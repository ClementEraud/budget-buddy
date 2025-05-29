<script lang="ts">
    import { Heading } from "flowbite-svelte";

    let { balance }: { balance: number } = $props();

    const getColorClassFromBalance = (balance: number) =>
        balance < 0
            ? "border-red-400 dark:border-red-500"
            : balance <= 100
              ? "border-amber-400 dark:border-amber-500"
              : "border-primary-400 dark:border-primary-500";
</script>

<div class="flex justify-center">
    <div
        class="flex justify-center items-center w-40 h-40 rounded-full border-[15px] {getColorClassFromBalance(
            balance,
        )}"
    >
        <div class="text-center">
            <Heading tag="h3" class="text-primary-700 dark:text-primary-300"
                >Balance</Heading
            >
            <p
                class="text-lg font-semibold text-secondary-600 dark:text-secondary-400"
            >
                {balance} €
            </p>
        </div>
    </div>
</div>
