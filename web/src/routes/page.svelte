<script lang="ts">
	import { onMount } from 'svelte';
	import Machine from './machine.svelte';

	type ContainerSummary = {
		name: string;
		image: string;
		status: string;
	};

	type MachineSummary = {
		name: string;
		address: string;
		containers: ContainerSummary[];
	};

	let machines: MachineSummary[] = [];

	onMount(async () => {
		console.log('Mounting App and fetching machines');
		const response = await fetch('/api/machines');
		const data = await response.json();
		machines = data.machines;
	});
</script>

<main>
	<h1>Container Monitor</h1>
	{#each machines as machine}
		<Machine {machine} />
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
