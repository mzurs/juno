<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import { missionControlStore } from '$lib/stores/mission-control.store';
	import { authSignedInStore } from '$lib/stores/auth.store';
	import type { Satellite } from '$declarations/mission_control/mission_control.did';
	import { navigateToSatellite } from '$lib/utils/nav.utils';
	import { createSatellite, loadSatellites } from '$lib/services/satellites.services';
	import { toasts } from '$lib/stores/toasts.store';
	import { isNullish } from '$lib/utils/utils';
	import SpinnerModal from '$lib/components/ui/SpinnerModal.svelte';
	import { i18n } from '$lib/stores/i18n.store';
	import type { JunoModalDetail } from '$lib/types/modal';
	import { wizardBusy } from '$lib/stores/busy.store';
	import WhatNext from '$lib/components/onboarding/WhatNext.svelte';
	import InstallSDK from '$lib/components/onboarding/InstallSDK.svelte';
	import Deploy from '$lib/components/onboarding/Deploy.svelte';
	import CreditsGuard from '$lib/components/guards/CreditsGuard.svelte';
	import { createOrbiter, loadOrbiters } from '$lib/services/orbiters.services';
	import type { Orbiter } from '$declarations/mission_control/mission_control.did';

	export let detail: JunoModalDetail;

	let insufficientFunds = true;

	let steps: 'init' | 'in_progress' | 'ready' | 'error' = 'init';
	let orbiter: Orbiter | undefined = undefined;

	const onSubmit = async () => {
		wizardBusy.start();
		steps = 'in_progress';

		try {
			orbiter = await createOrbiter({
				missionControl: $missionControlStore
			});

			// Reload list of orbiters before navigation
			await loadOrbiters({ missionControl: $missionControlStore, reload: true });

			steps = 'ready';
		} catch (err) {
			toasts.error({
				text: $i18n.errors.satellite_unexpected_error,
				detail: err
			});

			steps = 'error';
		}

		wizardBusy.stop();
	};

	const dispatch = createEventDispatcher();
	const close = () => dispatch('junoClose');
</script>

<Modal on:junoClose>
	{#if steps === 'ready'}
		<div class="msg">
			<p>{$i18n.analytics.ready}</p>
			<button on:click={close}>{$i18n.core.close}</button>
		</div>
	{:else if steps === 'in_progress'}
		<SpinnerModal>
			<p>{$i18n.analytics.initializing}</p>
		</SpinnerModal>
	{:else}
		<h2>{$i18n.analytics.start}</h2>

		<p>
			{$i18n.analytics.description}
		</p>

		<CreditsGuard
			on:junoClose
			bind:insufficientFunds
			{detail}
			priceLabel={$i18n.analytics.create_orbiter_price}
		>
			<form on:submit|preventDefault={onSubmit}>
				<button
					type="submit"
					disabled={!$authSignedInStore || isNullish($missionControlStore) || insufficientFunds}
					>{$i18n.analytics.create}</button
				>
			</form>
		</CreditsGuard>
	{/if}
</Modal>

<style lang="scss">
	@use '../../styles/mixins/overlay';

	h2 {
		@include overlay.title;
	}

	.msg {
		@include overlay.message;
	}
</style>
