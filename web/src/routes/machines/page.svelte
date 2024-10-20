<script lang="ts">
	import { onMount } from 'svelte';
	import Container from './container.svelte';
	let containers: any[] = [];

	onMount(async () => {
		let server_name = window.location.pathname.split('/').pop();
		const url = import.meta.env.PROD ? `/api/machines/${server_name}/containers` : `http://localhost:6780/api/machines/${server_name}/containers`;
		const response = await fetch(url);
		const data = await response.json();
		containers = data.containers;
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
