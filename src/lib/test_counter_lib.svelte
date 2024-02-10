<script lang="ts">
    let count: number = 0;
    const increment = () => {
        count += 1;
    };
</script>

<button on:click={increment}>
    count is {count}
</button>

<!--
Usage of the library example:
<script lang="ts">
    import test_counter from './lib/test_counter_lib.svelte'
</script>

<main>
    <div class="card">
        <test_counter />
    </div>
</main>
-->
