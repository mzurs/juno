<script lang="ts">
	import type { Principal } from '@dfinity/principal';
	import { i18n } from '$lib/stores/i18n.store';
	import Value from '$lib/components/ui/Value.svelte';
	import Canister from '$lib/components/canister/Canister.svelte';
	import type { CanisterData, CanisterSyncStatus } from '$lib/types/canister';
	import { formatTCycles } from '$lib/utils/cycles.utils';
	import SkeletonText from '$lib/components/ui/SkeletonText.svelte';

	export let canisterId: Principal;

	let data: CanisterData | undefined;
	let sync: CanisterSyncStatus | undefined;

	let idle_cycles_burned_per_day: bigint | undefined;

	$: idle_cycles_burned_per_day = data?.idle_cycles_burned_per_day;
</script>

<div class="status">
	<Value>
		<svelte:fragment slot="label">{$i18n.core.status}</svelte:fragment>
		<Canister {canisterId} bind:data bind:sync />
	</Value>
</div>

<div class="consumption">
	<Value>
		<svelte:fragment slot="label">{$i18n.canisters.daily_consumption}</svelte:fragment>
		{#if ['synced', 'syncing'].includes(sync ?? '')}
			<p>
				{formatTCycles(idle_cycles_burned_per_day ?? 0n)} T Cycles
			</p>
		{:else if sync === 'loading'}
			<p><SkeletonText /></p>
		{/if}
	</Value>
</div>

<style lang="scss">
	p {
		max-width: 300px;
		color: var(--value-color);
	}

	.status {
		min-height: calc(100px + var(--padding-2_5x));
	}

	.consumption {
		min-height: calc(48px + var(--padding-2_5x));
	}
</style>
