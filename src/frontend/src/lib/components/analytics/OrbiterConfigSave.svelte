<script lang="ts">
	import { busy, isBusy } from '$lib/stores/busy.store';
	import { i18n } from '$lib/stores/i18n.store';
	import { toasts } from '$lib/stores/toasts.store';
	import { setOrbiterSatelliteConfigs } from '$lib/api/orbiter.api';
	import { Principal } from '@dfinity/principal';
	import type { OrbiterSatelliteConfigEntry } from '$lib/types/ortbiter';
	import type { SatelliteIdText } from '$lib/types/satellite';
	import { nonNullish } from '$lib/utils/utils';
	import { createEventDispatcher } from 'svelte';

	export let orbiterId: Principal;
	export let config: Record<SatelliteIdText, OrbiterSatelliteConfigEntry>;

	let validConfirm = false;
	$: validConfirm = Object.keys(config).length > 0;

	const dispatch = createEventDispatcher();

	const handleSubmit = async () => {
		if (!validConfirm) {
			// Submit is disabled if not valid
			toasts.error({
				text: $i18n.errors.orbiter_configuration_missing
			});
			return;
		}

		busy.start();

		try {
			const results = await setOrbiterSatelliteConfigs({
				orbiterId,
				config: Object.entries(config).map(([satelliteId, value]) => [
					Principal.fromText(satelliteId),
					{
						enabled: value.enabled,
						updated_at: nonNullish(value.config) ? [value.config.updated_at] : []
					}
				])
			});

			dispatch('junoConfigUpdate', results);
		} catch (err: unknown) {
			toasts.error({
				text: $i18n.errors.orbiter_configuration_unexpected,
				detail: err
			});
		}

		busy.stop();
	};
</script>

<form class="container" on:submit|preventDefault={handleSubmit}>
	<button type="submit" class="submit" disabled={$isBusy || !validConfirm}>
		{$i18n.core.submit}
	</button>
</form>
