<script lang="ts">
	import { onMount } from 'svelte';
	import type { Principal } from '@dfinity/principal';
	import { toasts } from '$lib/stores/toasts.store';
	import { i18n } from '$lib/stores/i18n.store';
	import ButtonTableAction from '$lib/components/ui/ButtonTableAction.svelte';
	import type { Controller } from '$declarations/mission_control/mission_control.did';
	import { metadataProfile } from '$lib/utils/metadata.utils';
	import ControllerDelete from '$lib/components/controllers/ControllerDelete.svelte';
	import { missionControlStore } from '$lib/stores/mission-control.store';
	import { authStore } from '$lib/stores/auth.store';
	import { nonNullish } from '$lib/utils/utils';
	import ControllerInfo from '$lib/components/controllers/ControllerInfo.svelte';
	import ControllerAdd from '$lib/components/controllers/ControllerAdd.svelte';
	import Identifier from '$lib/components/ui/Identifier.svelte';
	import type { SetControllerParams } from '$lib/types/controllers';

	export let list: () => Promise<[Principal, Controller][]>;
	export let remove: (params: {
		missionControlId: Principal;
		controller: Principal;
	}) => Promise<void>;
	export let add: (
		params: {
			missionControlId: Principal;
		} & SetControllerParams
	) => Promise<void>;

	// The canister and user are controllers of the mission control but not added in its state per default
	export let extraControllers: [Principal, Controller | undefined][] = [];

	let controllers: [Principal, Controller | undefined][] = [];

	const load = async () => {
		try {
			controllers = [...(await list()), ...extraControllers];
		} catch (err: unknown) {
			toasts.error({
				text: $i18n.errors.controllers_listing,
				detail: err
			});
		}
	};

	onMount(async () => await load());

	let visibleDelete = false;
	let visibleInfo = false;
	let selectedController: [Principal, Controller | undefined] | undefined;

	const canEdit = (controllerId: Principal): boolean =>
		nonNullish($authStore.identity) &&
		nonNullish($missionControlStore) &&
		![$missionControlStore.toText(), $authStore.identity.getPrincipal().toText()].includes(
			controllerId.toText()
		);
</script>

<div class="table-container">
	<table>
		<thead>
			<tr>
				<th class="tools" />
				<th class="controller"> {$i18n.controllers.title} </th>
				<th class="profile"> {$i18n.controllers.profile} </th>
				<th class="scope"> {$i18n.controllers.scope} </th>
			</tr>
		</thead>
		<tbody>
			{#each controllers as [controllerId, controller] (controllerId.toText())}
				<tr>
					<td class="actions">
						{#if canEdit(controllerId)}
							<ButtonTableAction
								icon="delete"
								ariaLabel={$i18n.controllers.delete}
								on:click={() => {
									selectedController = [controllerId, controller];
									visibleDelete = true;
								}}
							/>
						{:else}
							<ButtonTableAction
								icon="info"
								ariaLabel={$i18n.controllers.info}
								on:click={() => (visibleInfo = true)}
							/>
						{/if}
					</td>

					<td>
						<Identifier identifier={controllerId.toText()} shorten={false} small={false} />
					</td>

					<td class="profile">{metadataProfile(controller?.metadata ?? [])}</td>

					<td class="scope">
						{#if nonNullish(controller)}
							{#if 'Write' in controller?.scope}
								{$i18n.controllers.write}
							{:else}
								{$i18n.controllers.admin}
							{/if}{/if}</td
					>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<ControllerAdd {add} {load} />

<ControllerDelete bind:selectedController bind:visible={visibleDelete} {load} {remove} />

<ControllerInfo bind:visible={visibleInfo} />

<style lang="scss">
	@use '../../styles/mixins/media';

	.tools {
		width: 48px;
	}

	.controller {
		@include media.min-width(small) {
			width: 60%;
		}
	}

	.profile,
	.scope {
		display: none;

		@include media.min-width(small) {
			display: table-cell;
		}
	}
</style>
