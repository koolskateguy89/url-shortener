<script lang="ts">
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';

	import { type ButtonVariantProps, buttonVariants } from 'ui-core/button';
	import { cn } from 'ui-core/utils';

	let className: string | undefined | null = undefined;
	export { className as class };
	export let href: HTMLAnchorAttributes['href'] = undefined;
	export let type: HTMLButtonAttributes['type'] = undefined;
	export let variant: ButtonVariantProps['variant'] = 'default';
	export let size: ButtonVariantProps['size'] = 'default';

	type Props = {
		class?: string | null;
		variant?: ButtonVariantProps['variant'];
		size?: ButtonVariantProps['size'];
	};

	interface AnchorElement extends Props, HTMLAnchorAttributes {
		href?: HTMLAnchorAttributes['href'];
		type?: never;
	}

	interface ButtonElement extends Props, HTMLButtonAttributes {
		type?: HTMLButtonAttributes['type'];
		href?: never;
	}

	type $$Props = AnchorElement | ButtonElement;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<svelte:element
	this={href ? 'a' : 'button'}
	type={href ? undefined : type}
	{href}
	class={cn(buttonVariants({ variant, size, className }))}
	{...$$restProps}
	on:click
	on:change
	on:keydown
	on:keyup
	on:mouseenter
	on:mouseleave
>
	<slot />
</svelte:element>
