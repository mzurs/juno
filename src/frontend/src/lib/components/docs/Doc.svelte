<script lang="ts">
	import { DATA_CONTEXT_KEY, type DataContext } from '$lib/types/data.context';
	import type { Doc } from '$declarations/satellite/satellite.did';
	import { getContext } from 'svelte';
	import type { Principal } from '@dfinity/principal';
	import { isNullish, nonNullish } from '$lib/utils/utils';
	import { fromArray, fromNullable } from '$lib/utils/did.utils';
	import Identifier from '$lib/components/ui/Identifier.svelte';
	import { formatToDate } from '$lib/utils/date.utils';
	import Json from '$lib/components/ui/Json.svelte';
	import Value from '$lib/components/ui/Value.svelte';
	import { i18n } from '$lib/stores/i18n.store';
	import { deleteDoc } from '$lib/api/satellites.api';
	import { toasts } from '$lib/stores/toasts.store';
	import DataToolbar from '$lib/components/data/DataToolbar.svelte';

	const { store, resetData }: DataContext<Doc> = getContext<DataContext<Doc>>(DATA_CONTEXT_KEY);

	let key: string | undefined;
	$: key = $store?.key;
	let doc: Doc | undefined;
	$: doc = $store?.data;

	let owner: Principal | undefined;
	$: owner = doc?.owner;

	let description: string | undefined;
	$: description = nonNullish(doc) ? fromNullable(doc.description) : undefined;

	let obj: unknown | undefined = undefined;
	$: (async () =>
		(obj = nonNullish(doc) && nonNullish(doc?.data) ? await fromArray(doc.data) : undefined))();

	let deleteData: (params: { collection: string; satelliteId: Principal }) => Promise<void>;
	$: deleteData = async (params: { collection: string; satelliteId: Principal }) => {
		if (isNullish(key) || key === '') {
			toasts.error({
				text: $i18n.errors.key_invalid
			});
			return;
		}

		await deleteDoc({
			...params,
			key,
			doc
		});

		resetData();
	};
</script>

<p class="title doc">{key ?? ''}</p>

{#if nonNullish(doc)}
	<article class="doc">
		<div class="owner">
			<Value>
				<svelte:fragment slot="label">{$i18n.document.owner}</svelte:fragment>
				{#if nonNullish(owner)}
					<Identifier identifier={owner.toText()} />
				{/if}
			</Value>
		</div>

		{#if nonNullish(description)}
			<Value>
				<svelte:fragment slot="label">{$i18n.document.description}</svelte:fragment>
				<p class="description">{description}</p>
			</Value>
		{/if}

		<div class="date">
			<Value>
				<svelte:fragment slot="label">{$i18n.document.created}</svelte:fragment>
				{formatToDate(doc.created_at)}
			</Value>
		</div>

		<div class="date">
			<Value>
				<svelte:fragment slot="label">{$i18n.document.updated}</svelte:fragment>
				{formatToDate(doc.updated_at)}
			</Value>
		</div>

		<div class="data">
			<Value>
				<svelte:fragment slot="label">{$i18n.document.data}</svelte:fragment>
				<div class="json"><Json json={obj} /></div>
			</Value>
		</div>

		<DataToolbar {deleteData}>
			<svelte:fragment slot="del-title">{$i18n.document.delete}</svelte:fragment>
			<svelte:fragment slot="del-content">{key}</svelte:fragment>
		</DataToolbar>
	</article>
{/if}

<style lang="scss">
	@use '../../styles/mixins/collections';
	@use '../../styles/mixins/media';
	@use '../../styles/mixins/text';

	.title {
		@include collections.title;
	}

	.doc {
		grid-column: span 4;

		@include media.min-width(large) {
			grid-column-start: 3;
			grid-column-end: 5;
		}
	}

	article {
		display: flex;
		flex-direction: column;

		padding: var(--padding-2x) var(--padding-2x) 0;
	}

	.owner {
		padding: 0 0 var(--padding);
	}

	.data,
	.date {
		padding: 0 0 var(--padding-2x);
	}

	.json {
		padding: var(--padding-2x) 0;
	}

	.description {
		@include text.clamp(5);
	}
</style>
