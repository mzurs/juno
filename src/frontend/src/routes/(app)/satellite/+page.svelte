<script lang="ts">
	import { satelliteStore } from '$lib/stores/satellite.store';
	import IdentityGuard from '$lib/components/guards/IdentityGuard.svelte';
	import SatelliteGuard from '$lib/components/guards/SatelliteGuard.svelte';
	import {
		type Tab,
		TABS_CONTEXT_KEY,
		type TabsContext,
		type TabsStore
	} from '$lib/types/tabs.context';
	import { writable } from 'svelte/store';
	import { setContext } from 'svelte';
	import Tabs from '$lib/components/ui/Tabs.svelte';
	import SatelliteOverview from '$lib/components/satellites/SatelliteOverview.svelte';
	import SatelliteControllers from '$lib/components/satellites/SatelliteControllers.svelte';
	import Warnings from '$lib/components/warning/Warnings.svelte';
	import MissionControlGuard from '$lib/components/guards/MissionControlGuard.svelte';
	import { nonNullish } from '$lib/utils/utils';
	import Examples from '$lib/components/examples/Examples.svelte';

	const tabs: Tab[] = [
		{
			id: Symbol('1'),
			labelKey: 'satellites.overview'
		},
		{
			id: Symbol('2'),
			labelKey: 'core.controllers'
		}
	];

	const store = writable<TabsStore>({
		tabId: tabs[0].id,
		tabs
	});

	setContext<TabsContext>(TABS_CONTEXT_KEY, {
		store
	});
</script>

<IdentityGuard>
	<Tabs
		help={$store.tabId === $store.tabs[0].id
			? 'https://juno.build/docs/add-juno-to-an-app/install-the-sdk-and-initialize-juno'
			: 'https://juno.build/docs/miscellaneous/controllers'}
	>
		<svelte:fragment slot="info">
			{#if nonNullish($satelliteStore)}
				<Warnings satellite={$satelliteStore} />
			{/if}
		</svelte:fragment>

		<SatelliteGuard>
			<MissionControlGuard>
				{#if nonNullish($satelliteStore)}
					{#if $store.tabId === $store.tabs[0].id}
						<SatelliteOverview satellite={$satelliteStore} />

						<Examples />
					{:else if $store.tabId === $store.tabs[1].id}
						<SatelliteControllers satellite={$satelliteStore} />
					{/if}
				{/if}
			</MissionControlGuard>
		</SatelliteGuard>
	</Tabs>
</IdentityGuard>
