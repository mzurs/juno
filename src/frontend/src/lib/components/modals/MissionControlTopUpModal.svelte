<script lang="ts">
	import { missionControlStore } from '$lib/stores/mission-control.store';
	import { nonNullish } from '$lib/utils/utils';
	import CanisterTopUpModal from '$lib/components/modals/CanisterTopUpModal.svelte';
	import { i18nFormat } from '$lib/utils/i18n.utils';
	import { i18n } from '$lib/stores/i18n.store';
	import type { JunoModalDetail, JunoModalTopUpMissionControlDetail } from '$lib/types/modal';

	export let detail: JunoModalDetail;

	$: balance = (detail as JunoModalTopUpMissionControlDetail).missionControlBalance?.balance ?? 0n;
</script>

{#if nonNullish($missionControlStore)}
	<CanisterTopUpModal canisterId={$missionControlStore} {balance} on:junoClose>
		<svelte:fragment slot="intro">
			<h2>
				{@html i18nFormat($i18n.canisters.top_up_title, [
					{
						placeholder: '{0}',
						value: 'mission control center'
					}
				])}
			</h2>
		</svelte:fragment>

		<svelte:fragment slot="outro">
			<p>{$i18n.canisters.top_up_mission_control_done}</p>
		</svelte:fragment>
	</CanisterTopUpModal>
{/if}
