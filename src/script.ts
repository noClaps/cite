import { getRef } from "./scripts/crossref";
import { toYAML } from "./scripts/yaml";

const input = document.querySelector("input")!;
const output = document.querySelector("output")!;

input.addEventListener("input", async () => {
  const doi = `10.${input.value.split("10.", 2)[1]}`;
  const ref = await getRef(doi);
  output.innerHTML = `<pre><code data-lang="yaml">${typeof ref == "string" ? ref : toYAML(ref)}</code></pre>`;
});

const button = document.querySelector("button")!;

button.addEventListener("click", () => {
  navigator.clipboard.writeText(output.innerText);
});
