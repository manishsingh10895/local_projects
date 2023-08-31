import { ProjectType } from "../types";

const projectTypeIconMap = {
    "angular": 'angular.svg',
    "react": 'react.png',
    "flutter": 'flutter.svg',
    "nextjs": 'nextjs.svg',
    "node": 'node.png',
    "python": 'python.png',
    "ruby": 'ruby.png',
    "rust": 'rust.svg',
    "svelte": 'svelte.png',
    "vue": 'vue.png'
}

export function getIconForProject(type: string) {
    return `/project-icons/${projectTypeIconMap[type]}`; 
}