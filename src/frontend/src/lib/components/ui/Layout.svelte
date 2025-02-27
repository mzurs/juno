<script lang="ts">
	import SplitPane from '$lib/components/ui/SplitPane.svelte';
	import { layoutTitle } from '$lib/stores/layout.store';
	import { nonNullish } from '$lib/utils/utils';

	export let centered = false;
	export let title = true;
</script>

<SplitPane>
	<slot name="menu" slot="menu" />

	<div class="content">
		<slot name="navbar" />

		<div class="page">
			<main class:centered>
				{#if title}
					<h1>
						<span class={`title ${nonNullish($layoutTitle) ? 'visible' : ''}`}
							>{$layoutTitle ?? ''}</span
						>
					</h1>
				{/if}

				<slot />
			</main>

			<slot name="footer" />
		</div>
	</div>
</SplitPane>

<style lang="scss">
	@use '../../styles/mixins/media';
	@use '../../styles/mixins/text';

	.content {
		position: relative;
		width: 100%;
		min-width: 100%;

		@include media.min-width(large) {
			min-width: auto;
		}
	}

	.page {
		height: 100%;
		overflow-y: auto;

		padding: var(--header-height) 0 0;
	}

	main {
		max-width: media.$breakpoint-extra-large;
		overflow-x: hidden;

		padding: 0 var(--padding-4x) var(--padding-4x) var(--padding-2x);

		position: relative;
		min-height: calc(100% - var(--footer-height));

		@include media.min-width(xlarge) {
			padding: 0 var(--padding-10x) var(--padding-2x);
		}
	}

	.centered {
		margin: 0 auto;
	}

	h1 {
		line-height: var(--line-height-standard);
		margin: 0 0 var(--padding-2x);

		@include text.truncate;

		&:before {
			content: '';
			display: inline-block;
		}
	}

	.title {
		visibility: hidden;
		opacity: 0;

		transition: opacity 0.15s ease-out;

		&.visible {
			visibility: visible;
			opacity: 1;
		}
	}
</style>
