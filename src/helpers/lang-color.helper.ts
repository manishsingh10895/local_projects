import { ProjectType } from "../types";

const LANGUAGE_COLOR_MAP = {
    "rust": "#795548",
    "javascript": "#9CCC65",
    "js": '#9CCC65',
    "jsx": '#9CCC65',
    "svelte": "#9CCC65",
    "vue": "#9CCC65",
    'dart': "#5C6BC0",
    "py": "#0288D1",
    "python": "#0288D1",
    "ruby": "#FF8A65",
    "rb": "#FF8A65",
    "toml": "#9FA8DA",
    "md": "#FFFDE7",
    "html": "#80CBC4",
    "css": "#FFCA28",
    "scss": "#FFCA28",
    "less": "#FFCA28",
    "xml": "#4E342E",
    "xhtml": "#80CBC4",
    "ts": "#1A237E",
    "tsx": "#1A237E",
    "typescript": "#1A237E",
    "java": "#004D40",
    "yaml": "#C2185B",
    "json": "#C2185B",
    "plaintext": "#EEEEEE",
    "kotlin": "#880E4F",
    "swift": "#A5D6A7",
    "xcodeconfig": "#BDBDBD",
    "d": "#BDBDBD",
    "shell": "#90A4AE",
    "batch": "#546E7A",
    "objective-c": "#F57C00",
    "objectivec": "#F57C00",
    "bash": "#90A4AE",
    "cmake": "#B39DDB",
    "c++": "#BF360C"
}

export function getLanguageColor(lang: string) {
    let color = LANGUAGE_COLOR_MAP[lang];

    if (!color) {
        return "#CFD8DC";
    }

    return color;
}