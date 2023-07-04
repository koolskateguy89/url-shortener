<script>
	import { dev } from '$app/environment';
	import { TailwindIndicator } from '$components';

	import '../app.postcss';
</script>

<svelte:head>
	<title>URL Shortener</title>
</svelte:head>

<slot />

{#if dev}
	<TailwindIndicator />
{/if}
