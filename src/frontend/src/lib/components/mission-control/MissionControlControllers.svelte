<script lang="ts">
	import type { Principal } from '@dfinity/principal';
	import {
		deleteMissionControlController,
		listMissionControlControllers,
		setMissionControlController
	} from '$lib/api/mission-control.api';
	import { authStore } from '$lib/stores/auth.store';
	import { nonNullish } from '$lib/utils/utils';
	import type { Controller } from '$declarations/mission_control/mission_control.did';
	import Controllers from '$lib/components/controllers/Controllers.svelte';
	import type { SetControllerParams } from '$lib/types/controllers';

	export let missionControlId: Principal;

	let controllers: Principal[] = [];

	const list = (): Promise<[Principal, Controller][]> =>
		listMissionControlControllers({ missionControlId });

	const remove = (params: { missionControlId: Principal; controller: Principal }): Promise<void> =>
		deleteMissionControlController(params);

	const add = (
		params: {
			missionControlId: Principal;
		} & SetControllerParams
	): Promise<void> => setMissionControlController(params);

	let extraControllers: [Principal, Controller | undefined][] = [
		[missionControlId, undefined],
		...(nonNullish($authStore.identity)
			? [[$authStore.identity.getPrincipal(), undefined] as [Principal, Controller | undefined]]
			: [])
	];
</script>

<Controllers {list} {remove} {add} {extraControllers} />
