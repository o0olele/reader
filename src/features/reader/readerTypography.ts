/**
 * Typography helpers shared by the reader catalog and the text stage.
 * Kept out of the SFCs so both stay under the 200-line frontend limit
 * (ROADMAP-v3 §3.5).
 */

/** `第X卷 / 第X部 / 第X篇` prefix, matching the catalog's volume grouping. */
const VOLUME_PATTERN = /^(第[零一二三四五六七八九十百千万两0-9]+[卷部篇])/

/** Prototype `.reader__pages .chapter-eyebrow` — the volume line above the title. */
export function volumeLabel(title: string): string {
  const match = VOLUME_PATTERN.exec(title.trim())
  return match ? match[1] : '正文'
}

/**
 * Prototype `.dialog-line` — spoken paragraphs lose the first-line indent and
 * gain a 2em left inset. Detection is structural (leading quote or dash), never
 * a guess about the text itself.
 */
export function isDialogueLine(text: string): boolean {
  return /^["“「『—]/.test(text.trim())
}

/** Paragraph list of a chapter body; blank lines are dropped, not rendered. */
export function splitParagraphs(content: string): string[] {
  return content
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
}
