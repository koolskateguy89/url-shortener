<script lang="ts">
	// idk if the `lang="ts"` is necessary, becuase of svelte kit config settings,
	// it seems it does type checking by default, need to read more docs

	import { Button, LoadingSpinner } from 'ui';

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
		<input type="url" name="url" placeholder="Url" />
		<Button type="submit">
			<LoadingSpinner class="mr-2" />
			Shorten
		</Button>
	</form>
</main>
