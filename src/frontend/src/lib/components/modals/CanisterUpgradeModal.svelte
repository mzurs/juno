<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { i18n } from '$lib/stores/i18n.store';
	import Modal from '$lib/components/ui/Modal.svelte';
	import SpinnerModal from '$lib/components/ui/SpinnerModal.svelte';
	import SelectUpgradeVersion from '$lib/components/upgrade/SelectUpgradeVersion.svelte';
	import type { Wasm } from '$lib/types/upgrade';
	import ReviewUpgradeVersion from '$lib/components/upgrade/ReviewUpgradeVersion.svelte';
	import { i18nFormat } from '$lib/utils/i18n.utils';

	export let currentVersion: string;
	export let newerReleases: string[];
	export let segment: 'satellite' | 'mission_control';
	export let upgrade: ({ wasm_module }: { wasm_module: Uint8Array }) => Promise<void>;

	let steps: 'init' | 'download' | 'review' | 'in_progress' | 'ready' | 'error' = 'init';

	const dispatch = createEventDispatcher();
	const close = () => dispatch('junoClose');

	let wasm: Wasm | undefined;

	const onSelect = ({
		detail: { steps: s, wasm: w }
	}: CustomEvent<{
		steps: 'review' | 'error' | 'download';
		wasm: Wasm;
	}>) => {
		wasm = w;
		steps = s;
	};
</script>

<Modal on:junoClose>
	{#if steps === 'ready'}
		<div class="msg">
			<p>
				{@html i18nFormat($i18n.canisters.upgrade_done, [
					{
						placeholder: '{0}',
						value: segment.replace('_', ' ')
					},
					{
						placeholder: '{1}',
						value: `v${wasm?.version ?? ''}`
					}
				])}
			</p>
			<button on:click={close}>{$i18n.core.close}</button>
		</div>
	{:else if steps === 'download'}
		<SpinnerModal>
			<p>{$i18n.canisters.download_in_progress}</p>
		</SpinnerModal>
	{:else if steps === 'in_progress'}
		<SpinnerModal>
			<p>{$i18n.canisters.upgrade_in_progress}</p>
		</SpinnerModal>
	{:else if steps === 'review'}
		<ReviewUpgradeVersion
			{wasm}
			{segment}
			{upgrade}
			on:junoNext={({ detail }) => (steps = detail)}
			on:junoClose
		/>
	{:else}
		<SelectUpgradeVersion {newerReleases} {segment} {currentVersion} on:junoNext={onSelect}>
			<slot name="intro" slot="intro" />
		</SelectUpgradeVersion>
	{/if}
</Modal>

<style lang="scss">
	@use '../../styles/mixins/overlay';

	.msg {
		@include overlay.message;
	}
</style>
