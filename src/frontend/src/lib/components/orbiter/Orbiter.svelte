<script lang="ts">
	import type { Orbiter } from '$declarations/mission_control/mission_control.did';
	import { i18n } from '$lib/stores/i18n.store';
	import Value from '$lib/components/ui/Value.svelte';
	import Identifier from '$lib/components/ui/Identifier.svelte';
	import { versionStore } from '$lib/stores/version.store';
	import CanisterOverview from '$lib/components/canister/CanisterOverview.svelte';

	export let orbiter: Orbiter;
</script>

<div class="card-container">
	<Value>
		<svelte:fragment slot="label">{$i18n.analytics.id}</svelte:fragment>
		<Identifier identifier={orbiter.orbiter_id.toText()} shorten={false} small={false} />
	</Value>

	<Value>
		<svelte:fragment slot="label">{$i18n.core.version}</svelte:fragment>
		<p>v{$versionStore?.orbiter?.current ?? '...'}</p>
	</Value>

	<CanisterOverview canisterId={orbiter.orbiter_id} />
</div>
