import { Command, open } from "@tauri-apps/api/shell";

/**
 * Processes Doc Html to open
 * links in new window
 **/
export function processDocHtml(html: string): string {
  const parser = new DOMParser();

  const doc = parser.parseFromString(html, "text/html");

  let anchors = doc.querySelectorAll("a");

  for (let a of anchors) {
    let url = a.href;

    a.setAttribute("href", url);
    a.setAttribute("target", "_blank");
  }

  let final_doc = doc.querySelector(".wasm_md_markdown");

  return final_doc?.innerHTML ?? "";
}
