<script lang="ts">
    import moment from "moment";
    import { Heading } from "flowbite-svelte";

    const monthToDisplay = moment().format("MMMM YYYY");
</script>

<div class="flex justify-between items-center p-4 bg-primary-100 dark:bg-primary-800 rounded-lg shadow-sm mb-6">
    <Heading tag="h1" class="text-primary-700 dark:text-primary-300 font-lobster">Budget Buddy</Heading>
    <div></div>
    <Heading tag="h2" class="text-secondary-600 dark:text-secondary-400">{monthToDisplay}</Heading>
</div>
