<script lang="ts">
	import { onMount } from 'svelte';

	import { api } from 'api';
	import { Button, LoadingSpinner } from 'ui';

	let me = '';
	let promise: ReturnType<typeof api.whoami> | undefined;

	$: promise?.then((data) => {
		me = data;
	});

	onMount(() => {
		promise = api.whoami();
	});

	function handleRefetch() {
		promise = api.whoami();
	}
</script>

<code>
	me ={' '}
	{#await promise}
		<LoadingSpinner class="mr-2 inline" />
	{/await}
	{JSON.stringify(me, null, 2)}
</code>

<Button on:click={handleRefetch} variant="destructive">Refetch</Button>
