<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { afterUpdate, onMount } from "svelte";
  import * as parser from "wasm_md_parser";
    import { processDocHtml } from "../helpers/html.helper";

  export let file: string | undefined;

  let doc_html: string;

  // let fs: any = {};

  let fileName: string;

  afterUpdate(() => {
    console.log("[afterUpdate]");
    readContents();
  });

  async function readContents() {
    console.log("[readContents] -> ", file);
    try {
      if (!file) {
        doc_html = "";
        fileName = "";
        return;
      }

      let contents: string = await invoke("get_file_contents", { file: file });

      // get file extenstion
      let ext = file?.split(".").pop();
      // get file name
      let name = file.split("/").pop();

      let html: string = "";

      switch (ext) {
        case "md": {
          let parsed = parser.parse(contents);

          html = processDocHtml(parsed);
          break;
        }
        default: {
          html = contents;
        }
      }

      doc_html = html;
      fileName = name ?? "";
    } catch (err) {
      console.error("Cannot read file ${file}");
      console.error(err);
    }
  }

  onMount(() => {
    readContents();
  });

</script>

<div style="width: 100%">
  <div class="title">
    {fileName}
  </div>
  {#if doc_html}
    <div class="doc-contents">
      {@html doc_html}
    </div>
  {:else}
    <div class="no-doc">
      No doc file
    </div>
  {/if}
</div>

  <style>
  .title {
    text-align : center;
    font-size: 1.5rem;
  }

  .doc-contents {
    padding: 10px 15px;
  }

  .no-doc {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>
