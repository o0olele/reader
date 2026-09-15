/**
 * 换源后定位同一章：`legado-with-MD3` 的 `BookHelp.getDurChapter()` 移植。
 *
 * 新书源的目录通常和旧书源一一对应，但不保证：章节可能被拆分或合并，序号可能
 * 重新编排，标题里可能多出「（第2页）」这类尾巴。所以先在**旧序号与按比例推算
 * 出的新序号之间 ±10 章**的窗口里找名字最像的一章（Jaccard 相似度 > 0.96 即认定
 * 同一章），否则退回到章序号最接近的一章；两者都不可信时才原样使用旧序号。
 */

/** 全角 → 半角，等价于参考项目的 `StringUtils.fullToHalf`。 */
function fullToHalf(input: string): string {
  let output = ''
  for (const char of input) {
    const code = char.charCodeAt(0)
    if (code === 12288) output += ' '
    else if (code >= 65281 && code <= 65374) output += String.fromCharCode(code - 65248)
    else output += char
  }
  return output
}

const CHINESE_DIGITS = '零一二三四五六七八九十'
const CHINESE_FORMAL_DIGITS = '〇壹贰叁肆伍陆柒捌玖拾'
const SINGLE_DIGIT_PATTERN = /^[〇零一二三四五六七八九壹贰叁肆伍陆柒捌玖]$/u

/** `char -> value`，与参考项目 `StringUtils.chnMap` 一致：个位数字 + 十/百/千/万/亿。 */
const CHN_MAP = new Map<string, number>()
for (let index = 0; index <= 10; index++) {
  CHN_MAP.set(CHINESE_DIGITS[index], index)
  CHN_MAP.set(CHINESE_FORMAL_DIGITS[index], index)
}
CHN_MAP.set('百', 100)
CHN_MAP.set('佰', 100)
CHN_MAP.set('千', 1000)
CHN_MAP.set('仟', 1000)
CHN_MAP.set('万', 10000)
CHN_MAP.set('亿', 100000000)

/** 「一千零二十五」「一千二」这类中文数字，等价于 `StringUtils.chineseNumToInt`。 */
function chineseNumToInt(value: string): number {
  const chars = [...value]
  // 「一零二五」形式：逐字读音拼成数字，而不是按单位求和。
  if (chars.length > 1 && SINGLE_DIGIT_PATTERN.test(value)) {
    const digits = chars.map((char) => CHN_MAP.get(char) ?? 0).join('')
    return Number.parseInt(digits, 10)
  }
  let result = 0
  let tmp = 0
  let billion = 0
  for (let index = 0; index < chars.length; index++) {
    const value = CHN_MAP.get(chars[index])
    if (value === undefined) return -1
    if (value === 100000000) {
      result += tmp
      result *= value
      billion = billion * 100000000 + result
      result = 0
      tmp = 0
    } else if (value === 10000) {
      result += tmp
      result *= value
      tmp = 0
    } else if (value >= 10) {
      if (tmp === 0) tmp = 1
      result += value * tmp
      tmp = 0
    } else {
      // 「一千二」= 1200：末尾数字紧跟单位时按单位的十分之一计。
      tmp =
        index >= 2 && index === chars.length - 1 && (CHN_MAP.get(chars[index - 1]) ?? 0) > 10
          ? (value * (CHN_MAP.get(chars[index - 1]) ?? 0)) / 10
          : tmp * 10 + value
    }
  }
  return result + tmp + billion
}

/** 阿拉伯数字或中文数字 → 整数；两个正则都不匹配时给 -1。 */
function stringToInt(value: string): number {
  const normalized = fullToHalf(value).replace(/\s+/gu, '')
  if (/^-?\d+$/.test(normalized)) return Number.parseInt(normalized, 10)
  return chineseNumToInt(normalized)
}

const CHAPTER_NUMBER_PATTERN = /.*?第([\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+)[章节篇回集话]/
const BARE_NUMBER_PATTERN =
  /^(?:[\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+[,:、])*([\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+)(?:[,:、]|\.[^\d])/
const CHAPTER_PREFIX_PATTERN =
  /^.*?第(?:[\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+)[章节篇回集话](?!$)|^(?:[\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+[,:、])*(?:[\d零〇一二两三四五六七八九十百千万壹贰叁肆伍陆柒捌玖拾佰仟]+)(?:[,:、](?!$)|\.(?=[^\d]))/g
const CHAPTER_DECORATION_PATTERN =
  /(?!^)(?:[〖【《〔[{(][^〖【《〔[{()〕》》】〗\]}]+)?[)〕》》】〗\]}]$|^[〖【《〔[{(](?:[^〖【《〔[{()〕》》】〗\]}]+[〕》》】〗\]})])?(?!$)/g
const NON_WORD_PATTERN = /[^\w\u4E00-\u9FEF〇\u3400-\u4DBF\u{20000}-\u{2A6DF}\u{2A700}-\u{2EBEF}]/gu

/** 章序号；`第12章` / `12、` 都能取到 12，取不到给 -1。 */
export function getChapterNum(title: string | undefined): number {
  if (!title) return -1
  const normalized = fullToHalf(title).replace(/\s/gu, '')
  const matched = CHAPTER_NUMBER_PATTERN.exec(normalized) ?? BARE_NUMBER_PATTERN.exec(normalized)
  return stringToInt(matched?.[1] ?? '-1')
}

/** 去掉序号与括号装饰后的章节名，用来做相似度比较。 */
export function getPureChapterName(title: string | undefined): string {
  if (!title) return ''
  return fullToHalf(title)
    .replace(/\s/gu, '')
    .replace(CHAPTER_PREFIX_PATTERN, '')
    .replace(CHAPTER_DECORATION_PATTERN, '')
    .replace(NON_WORD_PATTERN, '')
}

/** Apache Commons Text 的 `JaccardSimilarity`：字符集合的交并比。 */
export function jaccardSimilarity(left: string, right: string): number {
  const leftChars = new Set(left)
  const rightChars = new Set(right)
  if (!leftChars.size || !rightChars.size) return 0
  let intersection = 0
  for (const char of leftChars) if (rightChars.has(char)) intersection++
  return intersection / (leftChars.size + rightChars.size - intersection)
}

/**
 * 旧目录的第 `oldIndex` 章在新目录里是第几章。
 *
 * `oldSize` 是旧目录的章数（`0` 表示不知道），用来把旧序号按阅读比例折算成新序号；
 * 折算结果与旧序号之间的 ±10 章就是搜索窗口。
 */
export function matchChapterIndex(
  oldIndex: number,
  oldTitle: string | undefined,
  newTitles: string[],
  oldSize = 0,
): number {
  if (oldIndex <= 0) return 0
  if (!newTitles.length) return oldIndex
  const oldChapterNum = getChapterNum(oldTitle)
  const oldName = getPureChapterName(oldTitle)
  const newSize = newTitles.length
  const scaledIndex = oldSize === 0 ? oldIndex : Math.floor((oldIndex * oldSize) / newSize)
  const from = Math.max(0, Math.min(oldIndex, scaledIndex) - 10)
  const to = Math.min(newSize - 1, Math.max(oldIndex, scaledIndex) + 10)

  let nameSimilarity = 0
  let newIndex = 0
  let newNum = 0
  if (oldName) {
    for (let index = from; index <= to; index++) {
      const similarity = jaccardSimilarity(oldName, getPureChapterName(newTitles[index]))
      if (similarity > nameSimilarity) {
        nameSimilarity = similarity
        newIndex = index
      }
    }
  }
  if (nameSimilarity < 0.96 && oldChapterNum > 0) {
    for (let index = from; index <= to; index++) {
      const num = getChapterNum(newTitles[index])
      if (num === oldChapterNum) {
        newNum = num
        newIndex = index
        break
      }
      if (Math.abs(num - oldChapterNum) < Math.abs(newNum - oldChapterNum)) {
        newNum = num
        newIndex = index
      }
    }
  }
  if (nameSimilarity > 0.96 || Math.abs(newNum - oldChapterNum) < 1) return newIndex
  return Math.min(Math.max(0, newSize - 1), oldIndex)
}
