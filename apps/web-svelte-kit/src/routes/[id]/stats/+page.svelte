<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>{data.id} Stats</title>
</svelte:head>

<main>
	<h1>Stats</h1>
	<p>URL: {data.url}</p>
	<p>Num hits: {data.num_hits}</p>
</main>
