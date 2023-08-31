import { writable, type Writable } from "svelte/store";
import type { Config, Project } from "../types";
import { getContext, setContext } from "svelte";

export type AppData = {
  config: Config;
  projects: Array<Project>;
};

export function setConfig(config: any) {
  setContext("app", {
    config,
  });
}
