<script lang="ts">
	import { Button, Input, LoadingSpinner } from 'ui';
	import { z } from 'zod';

	import { api } from 'api';

	const formSchema = z.object({
		url: z.string().url()
	});

	let promise: ReturnType<typeof api.shorten> | undefined;

	function handleSubmit(
		e: SubmitEvent & {
			currentTarget: HTMLFormElement;
		}
	) {
		const formData = new FormData(e.currentTarget);
		const parsed = formSchema.safeParse(Object.fromEntries(formData));

		if (!parsed.success) {
			alert('Invalid url');
			return;
		}

		const { url } = parsed.data;

		promise = api.shorten(url);
	}
</script>

<main class="flex h-screen flex-col items-center justify-center space-y-4">
	{#if promise}
		<p>
			{#await promise}
				Loading...
			{:then data}
				{@const { id } = data}
				{JSON.stringify(data, null, 2)}
				<a href="/{id}" class="underline">
					BASE_URL/{id}
				</a>
			{:catch error}
				Error = {JSON.stringify(error, null, 2)}
			{/await}
		</p>
	{/if}

	<form class="flex flex-col items-center space-y-2" on:submit|preventDefault={handleSubmit}>
		<Input type="url" name="url" placeholder="Url" />
		<Button type="button">
			{#await promise}
				<LoadingSpinner class="mr-2" />
			{/await}
			Shorten
		</Button>
	</form>
</main>
