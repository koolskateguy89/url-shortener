<script lang="ts">
	import { goto } from '$app/navigation';
	import { z } from 'zod';

	import { type RegisterRequest, api } from 'api';
	import { Button, Input, LoadingSpinner } from 'ui';

	let username = '';
	let password = '';

	let isLoading = false;

	const formDataSchema = z.object({
		username: z.string(),
		password: z.string().min(4)
	}) satisfies z.ZodType<RegisterRequest>;

	async function handleSubmit() {
		const parsed = formDataSchema.safeParse({ username, password });

		if (!parsed.success) {
			alert('Invalid credentials');
			return;
		}

		isLoading = true;

		const registered = await api.register(parsed.data);
		alert(registered ? 'Registered' : 'Failed to register');

		if (registered) goto('/login');
	}
</script>

<main class="flex h-screen flex-col items-center justify-center">
	<form on:submit|preventDefault={handleSubmit} class="flex flex-col items-center gap-y-2">
		<Input
			bind:value={username}
			placeholder="Username"
			autocomplete="username"
			disabled={isLoading}
			required
		/>

		<Input
			bind:value={password}
			type="password"
			placeholder="Password"
			autocomplete="new-password"
			disabled={isLoading}
			required
		/>

		<div>
			<Button href="/login" variant="link">Login</Button>
			<Button type="submit" disabled={isLoading}>
				{#if isLoading}
					<LoadingSpinner class="mr-2" />
				{/if}
				Register
			</Button>
		</div>
	</form>
</main>
