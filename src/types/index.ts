export type Config = {
  project_dirs: String[];
};

export type Project = {
  name: String;
  path: String;
  git: Array<String>;
  description: String;
  language_map: Map<String, number>;
  project_type: ProjectType;
  last_modified: Date;
};

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
