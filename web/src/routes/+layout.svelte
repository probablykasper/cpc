<script lang="ts">
  import "../app.css";

  let { children } = $props();

  import { wasm_eval } from "cpc";
</script>

{@render children()}
<p>{wasm_eval("1 + 1")}</p>
