<script lang="ts">
    import type { Operation } from "../../domain/types/operation";

    const { operation }: { operation: Operation } = $props();
</script>

<li>{`${operation.label}: ${operation.value}€`}</li>
