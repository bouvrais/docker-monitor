<script>
    import { onMount } from 'svelte';

    let greeting = "";

    onMount(() => {
        greeting = "Hello World from subpage";
    });
</script>

<h1>{greeting}</h1>