export type Config = {
  project_dirs: string[];
};

export interface IProject {
  name: string;
  path: string;
  git: Array<string>;
  description?: string;
  language_map: Map<string, number>;
  project_type: ProjectType;
  last_modified: Date;
  documentation_file?: string;
}

export enum ProjectType {
  Rust,
  Python,
  Flutter,
  Ruby,
  NextJs,
  Svelte,
  React,
  Angular,
  Node,
  Vue,
}
