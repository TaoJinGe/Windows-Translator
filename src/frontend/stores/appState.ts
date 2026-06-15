import { writable } from "svelte/store";

export type ActiveView = "translate" | "settings" | "history";

export const activeView = writable<ActiveView>("translate");
