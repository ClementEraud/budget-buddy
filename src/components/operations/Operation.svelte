<script lang="ts">
    import type { Operation } from "../../domain/types/operation";
    import { Li } from "flowbite-svelte";

    const { operation }: { operation: Operation } = $props();
</script>

<Li class="py-2 px-3 flex justify-between items-center">
    <span class="text-gray-700 dark:text-gray-300">{operation.label}</span>
    <span class="font-semibold text-secondary-600 dark:text-secondary-400"
        >{operation.amount}€</span
    >
</Li>
