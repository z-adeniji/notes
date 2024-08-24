import { writable } from 'svelte/store';

export interface Element {
    component: any;
    component_lineno: number | null;
    content: string;
    id: string;
    component_no: number;
}

export const elements = writable<Element[][]>([]);
export const component_no = writable<number>(-1);
export const lineFocus = writable<number>(0);

export const headerCount = writable(0);
export const paraCount = writable(0);
export const listCount = writable(0);
export const toDoListCount = writable(0);