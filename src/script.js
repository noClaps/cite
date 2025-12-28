// @ts-check

import { getRef } from "./scripts/crossref.js";
import { toYAML } from "./scripts/yaml.js";

const input = document.querySelector("input");
if (!input) throw "Input element not found";

const output = document.querySelector("output");
if (!output) throw "Output element not found";

input.addEventListener("input", async () => {
  const doi = `10.${input.value.split("10.", 2)[1]}`;
  const ref = await getRef(doi);
  output.innerHTML = `<pre><code data-lang="yaml">${typeof ref == "string" ? ref : toYAML(ref)}</code></pre>`;
});

const button = document.querySelector("button");
if (!button) throw "Button element not found";

button.addEventListener("click", () => {
  navigator.clipboard.writeText(output.innerText);
});
