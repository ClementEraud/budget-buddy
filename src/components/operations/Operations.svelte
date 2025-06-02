<script lang="ts">
    import type { Operations } from "../../domain/types/operation";
    import OperationList from "./OperationList.svelte";
    import { Card, Heading } from "flowbite-svelte";

    const {
        title,
        operations,
        total,
    }: { title: string; operations: Operations; total: number } = $props();
</script>

<Card class="h-full flex flex-col bg-white dark:bg-gray-800">
    <div class="p-3 bg-primary-100 dark:bg-primary-800 rounded-t-lg">
        <Heading
            tag="h3"
            class="text-primary-700 dark:text-primary-300 text-center"
            >{title}</Heading
        >
    </div>
    <div class="flex-grow overflow-auto p-2">
        <OperationList {operations} />
    </div>
    <div class="p-3 bg-secondary-100 dark:bg-secondary-800 rounded-b-lg">
        <Heading
            tag="h4"
            class="text-secondary-700 dark:text-secondary-300 text-center"
        >
            Total: {total}€
        </Heading>
    </div>
</Card>
