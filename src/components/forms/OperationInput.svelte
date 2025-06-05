<script lang="ts">
    import {
        Button,
        ButtonGroup,
        Heading,
        Input,
        InputAddon,
    } from "flowbite-svelte";
    import Separator from "../ui/Separator.svelte";
    import OperationsComponent from "../operations/Operations.svelte";
    import { PlusOutline } from "flowbite-svelte-icons";

    const { title, listTitle } = $props();

    const operations = [
        { label: "Salary", amount: 5000 },
        { label: "Freelance", amount: 2000 },
        { label: "Investments", amount: 1000 },
    ];
</script>

<Heading
    tag="h2"
    class="text-center text-primary-500 mb-20"
    title="Input your incomes"
>
    {title}
</Heading>

<div class="flex flex-col w-full h-full items-center">
    <ButtonGroup class="flex flex-row w-2/7 justify-center mb-5">
        <Input
            type="text"
            placeholder="Income Name"
            color="gray"
            class="input w-full max-w-xs "
        />
        <Input
            type="number"
            color="gray"
            placeholder="Amount"
            class="input w-full max-w-xs"
        />
        <InputAddon>€</InputAddon>
        <Button color="primary" class="btn max-w-xs bg-white text-primary-400">
            <PlusOutline />
        </Button>
    </ButtonGroup>
    <Separator />
    <OperationsComponent title={listTitle} {operations} total={8000} />
</div>
