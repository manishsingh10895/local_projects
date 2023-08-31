import { writable, type Writable } from "svelte/store";
import type { Config, IProject } from "../types";
import { getContext, setContext } from "svelte";

export type AppContext = {
  config: Writable<Config>;
  projects: Writable<Array<IProject>>;
};
