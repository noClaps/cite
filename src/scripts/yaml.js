// @ts-check

/**
 * @param {import("./crossref").Reference} a
 */
export function toYAML(a) {
  const published = a.published["date-parts"][0];
  return `
  type: "article"
  title: "${a.title}"
  author:
  ${a.author.map((author) => `  - "${author.family}, ${author.given}"`).join("\n  ")}
  date: ${published[0].toString().padStart(4, "0")}-${published[1]?.toString().padStart(2, "0") ?? ""}-${published[2]?.toString().padStart(2, "0") ?? ""}
  page-range: "${a.page}"
  serial-number:
    doi: "${a.DOI}"
  parent:
    volume: ${a.volume}
    issue: ${a.issue}
    title: "${a["container-title"][0]}"
`;
}
