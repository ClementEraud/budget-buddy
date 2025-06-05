<script lang="ts">
    import { Button, Heading, StepIndicator } from "flowbite-svelte";
    import OperationInput from "../../../components/forms/OperationInput.svelte";

    let currentStep = 1;
    let steps = ["incomes", "expenses", "review"];

    const handleNext = () => {
        if (currentStep === 3) {
            // Handle final step logic here
        } else {
            currentStep += 1;
        }
    };

    const handlePrevious = () => {
        currentStep -= 1;
    };
</script>

<form class="flex flex-col w-full h-full justify-start items-center">
    <Heading
        tag="h1"
        class="text-center text-primary-500 mb-5"
        title="New Budget"
    >
        Create a new budget
    </Heading>

    <div class="flex flex-row justify-around items-center w-2/4 mt-8 mb-8">
        <Button
            color="secondary"
            class="btn w-30"
            disabled={currentStep === 1}
            onclick={handlePrevious}>Previous</Button
        >
        <StepIndicator
            class="w-2/5"
            size="h-3"
            {steps}
            {currentStep}
            glow
            hideLabel
        />
        <Button color="primary" class="btn w-30" onclick={handleNext}>
            {currentStep === steps.length ? "Finish" : "Next"}
        </Button>
    </div>

    {#if currentStep === 1}
        <OperationInput title="Input your incomes" listTitle="Incomes" />
    {:else if currentStep === 2}
        <OperationInput title="Input your expenses" listTitle="Expenses" />
    {:else}
        <Heading
            tag="h2"
            class="text-center text-primary-500 mb-20"
            title="Review your budget"
        >
            Review your budget
        </Heading>
    {/if}
</form>
