// @ts-check

/**
 * @typedef {{
 *   "container-title": [string];
 *   issue: string;
 *   DOI: string;
 *   published: {
 *     "date-parts": [[number, number?, number?]];
 *   };
 *   page: string;
 *   title: [string];
 *   volume: string;
 *   author: {
 *     given: string;
 *     family: string;
 *     sequence: "first" | "additional";
 *   }[];
 * }} Reference
 */

/**
 * @param {string} doi
 */
export async function getRef(doi) {
  try {
    const res = await fetch(`https://api.crossref.org/works/doi/${doi}`).catch(
      (err) => {
        throw err;
      },
    );
    if (res.status != 200) throw "Error fetching reference";

    return res
      .json()
      .catch((err) => {
        throw err;
      })
      .then((r) => /** @type {Reference} */ (r.message));
  } catch (err) {
    return `This DOI didn't return any results: ${err}`;
  }
}
