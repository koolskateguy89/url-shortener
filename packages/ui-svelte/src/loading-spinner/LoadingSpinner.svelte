<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { Loader2 } from 'lucide-svelte';

	import { cn } from 'ui-core/utils';

	let className: string | undefined | null = undefined;
	export { className as class };

	type $$Props = ComponentProps<Loader2>;
</script>

<Loader2 {...$$restProps} class={cn('h-4 w-4 animate-spin', className)} />
