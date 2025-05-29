<script>
    import { Alert, Button, Card, Toast } from "flowbite-svelte";
    import { FireOutline } from "flowbite-svelte-icons";
</script>

<Button color="primary">Primary</Button>
<Alert color="secondary">Secondary</Alert>

<Card href="/cards" color="secondary" class="p-4 sm:p-6">
    <h5
        class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-red-800"
    >
        Noteworthy technology acquisitions 2021
    </h5>
    <p class="leading-tight font-normal text-gray-700 dark:text-gray-700">
        Here are the biggest enterprise technology acquisitions of 2021 so far,
        in reverse chronological order.
    </p>
</Card>
<Toast color="primary">
    {#snippet icon()}
        <FireOutline class="h-6 w-6" />
    {/snippet}
    Dismissable user notification.
</Toast>
