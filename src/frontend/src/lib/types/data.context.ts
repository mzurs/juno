import type { Writable } from 'svelte/store';

export interface DataStore<T> {
	key: string | undefined;
	data: T | undefined;
}

export interface DataContext<T> {
	store: Writable<DataStore<T>>;
	resetData: () => void;
}

export const DATA_CONTEXT_KEY = Symbol('data');
