<script>
    import "@fontsource/lobster";
    import "../app.css";
    import { Card } from "flowbite-svelte";
    import Header from "../components/Header.svelte";

    let { children } = $props();
</script>

<main class="container mx-auto px-4 flex flex-col h-[90vh]">
    <Card class="my-4">
        <Header />
    </Card>
    {@render children()}
</main>
