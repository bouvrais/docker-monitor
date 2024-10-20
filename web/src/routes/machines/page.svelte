<script>
	import { onMount } from 'svelte';
	import Container from './container.svelte';
	let containers = [];
	onMount(async () => {
		console.log('Mounting App and fetching containers');
		let server_name = "Server%201";	//window.location.hostname;
		const url = import.meta.env.PROD ? `/api/machines/${server_name}/containers` : `http://localhost:6780/api/machines/${server_name}/containers`;
		console.log(url);
		const response = await fetch(url);
		console.log(response);
		const data = await response.json();
		console.log(data);
		console.log("This is a test");
		containers = data.containers;
		containers.forEach(container => {
			console.log(container);
		});
	});
</script>

<main>
	<h1>Container Monitor</h1>
	{#each containers as container}
		<Container {container} />
	{/each}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
