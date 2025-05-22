<script lang="ts">
    import moment from "moment";

    const monthToDisplay = moment().format("MMMM YYYY");
</script>

<div class="top-header">
    <h1>Budget Buddy</h1>
    <div></div>
    <h2>{monthToDisplay}</h2>
</div>

<style>
    .top-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;

        h1 {
            text-align: left;
        }
    }
</style>
