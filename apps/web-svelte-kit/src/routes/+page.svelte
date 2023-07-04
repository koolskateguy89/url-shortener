<script lang="ts">
	import { Button, Input, LoadingSpinner } from 'ui';

	// TODO: will probably put this in +page.ts, in form action
	// import { api } from 'api';
</script>

<main class="flex h-screen flex-col items-center justify-center space-y-4">
	<Button>test default button</Button>
	<Button variant="secondary">test secondary button</Button>
	<Button variant="destructive">test destructive button</Button>

	<!-- TODO: status display -->

	<!-- TODO?: action -->
	<form class="flex flex-col items-center space-y-2">
		<Input type="url" name="url" placeholder="Url" />
		<Button type="submit">
			<LoadingSpinner class="mr-2" />
			Shorten
		</Button>
	</form>
</main>
