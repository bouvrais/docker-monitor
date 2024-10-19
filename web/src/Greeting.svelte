<script>
    import { onMount } from 'svelte';

    const url = import.meta.env.PROD ? 'api/machines' : "http://localhost:6780/api/machines";

    let greeting = "";

    async function getHello() {
        const opts = {
            method: "get"
        };
        const response = await fetch(url, opts);
        const result = await response.text();
        console.log("Coucou");
        console.log("Result:", result);

        greeting = "Docker Monitor";
    }

    onMount(() => getHello())
</script>

<h1>{greeting}</h1>