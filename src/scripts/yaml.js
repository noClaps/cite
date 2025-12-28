// @ts-check

/**
 * @param {import("./crossref").Reference} a
 */
export function toYAML(a) {
  return `
type: "article"
title: "${a.title}"
author:
${a.author.map((author) => `  - "${author.family}, ${author.given}"`).join("\n")}
doi: "${a.DOI}"
date: ${new Date(a.created.timestamp).toISOString().split("T", 2)[0]}
page-range: "${a.page}"
parent:
  volume: ${a.volume}
  issue: ${a.issue}
  title: "${a["short-container-title"]}"
`.trim();
}
