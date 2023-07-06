<script lang="ts">
	import { z } from 'zod';

	import { type LoginRequest, api } from 'api';
	import { Button, Input, LoadingSpinner } from 'ui';

	import WhoAmI from './WhoAmI.svelte';

	let username = '';
	let password = '';

	let isLoading = false;

	const formDataSchema = z.object({
		username: z.string(),
		password: z.string().min(4)
	}) satisfies z.ZodType<LoginRequest>;

	async function handleSubmit() {
		const parsed = formDataSchema.safeParse({ username, password });

		if (!parsed.success) {
			alert('Invalid credentials');
			return;
		}

		isLoading = true;

		const loggedIn = await api.login(parsed.data);

		isLoading = false;
		alert(loggedIn ? 'Logged in' : 'Failed to log in');
	}

	function handleLogout() {
		api.logout();
	}
</script>

<main class="flex h-screen flex-col items-center justify-center">
	<div class="mb-12 flex flex-col gap-y-4">
		<WhoAmI />

		<Button on:click={handleLogout} variant="destructive">LOG out</Button>
	</div>

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
			name="password"
			placeholder="Password"
			autocomplete="current-password"
			disabled={isLoading}
			required
		/>

		<div>
			<Button type="submit" disabled={isLoading}>
				{#if isLoading}
					<LoadingSpinner class="mr-2" />
				{/if}
				Login
			</Button>
			<Button href="/register" variant="link">Register</Button>
		</div>
	</form>
</main>
