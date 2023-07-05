<script lang="ts">
	import { page } from '$app/stores';

	let searchParams = $page.url.searchParams;

	const id = searchParams.get('id');
	const cause = searchParams.get('cause');
</script>

<main>
	<h1>Error</h1>
	<pre>ID = {id}</pre>
	<pre>searchParams = {JSON.stringify(Object.fromEntries(searchParams.entries()), null, 2)}</pre>
</main>
