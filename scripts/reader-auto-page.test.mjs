import assert from 'node:assert/strict'
import { Buffer } from 'node:buffer'
import { readFileSync } from 'node:fs'
import { setImmediate as settle } from 'node:timers/promises'
import test from 'node:test'
import { createRenderer, ref } from 'vue'
import ts from 'typescript'

// Run the production composables with Node's clock and Vue's real lifecycle.
function moduleUrl(path, imports = {}) {
  const source = readFileSync(new URL(path, import.meta.url), 'utf8')
  let { outputText } = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.ESNext } })
  for (const [specifier, url] of Object.entries({ vue: import.meta.resolve('vue'), ...imports })) {
    outputText = outputText.replaceAll(`from '${specifier}'`, `from '${url}'`)
  }
  outputText += `\n//# sourceURL=${new URL(path, import.meta.url).href}\n`
  return `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
}
const { useReaderAutoPage } = await import(moduleUrl('../src/features/reader/useReaderAutoPage.ts'))
const { useReaderPaging } = await import(
  moduleUrl('../src/features/reader/useReaderPaging.ts', {
    './readerPosition': moduleUrl('../src/features/reader/readerPosition.ts'),
  })
)
const renderer = createRenderer({
  createComment: () => ({}),
  insert() {},
  remove() {},
  parentNode: () => null,
  nextSibling: () => null,
})

function mount(t, setup) {
  let state
  const app = renderer.createApp({
    setup() {
      state = setup()
      return () => null
    },
  })
  app.mount({})
  t.after(() => app.unmount())
  return { state, unmount: () => app.unmount() }
}

test('waits for loaded content and gives a newly selected chapter a full interval', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  const ready = ref(true)
  const chapter = ref('1')
  const advance = t.mock.fn(() => true)
  const { state } = mount(t, () =>
    useReaderAutoPage(
      () => ready.value,
      () => chapter.value,
      advance,
      assert.fail,
    ),
  )
  state.start()
  t.mock.timers.tick(11_000)
  assert.equal(advance.mock.callCount(), 0)
  assert.equal(state.secondsLeft.value, 1)
  ready.value = false
  chapter.value = '2'
  t.mock.timers.tick(60_000)
  assert.equal(advance.mock.callCount(), 0)
  ready.value = true
  t.mock.timers.tick(11_000)
  assert.equal(advance.mock.callCount(), 0)
  t.mock.timers.tick(1000)
  await settle()
  assert.equal(advance.mock.callCount(), 1)
  assert.equal(state.secondsLeft.value, 12)
})

test('does not overlap slow chapter requests and stops when a new chapter fails to load', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  const chapter = ref('1')
  let finish
  const advance = t.mock.fn(
    () =>
      new Promise((resolve) => {
        finish = resolve
      }),
  )
  const { state } = mount(t, () =>
    useReaderAutoPage(
      () => true,
      () => chapter.value,
      advance,
      assert.fail,
    ),
  )
  state.start()
  t.mock.timers.tick(12_000)
  chapter.value = '2'
  t.mock.timers.tick(60_000)
  assert.equal(advance.mock.callCount(), 1)
  finish(false)
  await settle()
  assert.equal(state.active.value, false)
  t.mock.timers.tick(60_000)
  assert.equal(advance.mock.callCount(), 1)
})

test('stops at the end of the book and clears timers on pause and unmount', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  const advance = t.mock.fn(() => false)
  const { state, unmount } = mount(t, () =>
    useReaderAutoPage(
      () => true,
      () => '1',
      advance,
      assert.fail,
    ),
  )
  state.start()
  t.mock.timers.tick(12_000)
  await settle()
  assert.equal(state.active.value, false)
  state.start()
  state.stop()
  t.mock.timers.tick(60_000)
  assert.equal(advance.mock.callCount(), 1)
  state.start()
  unmount()
  t.mock.timers.tick(60_000)
  assert.equal(advance.mock.callCount(), 1)
})

test('an old request cannot turn off a restarted auto-page session', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  let finish
  const advance = t.mock.fn(
    () =>
      new Promise((resolve) => {
        finish = resolve
      }),
  )
  const { state } = mount(t, () =>
    useReaderAutoPage(
      () => true,
      () => '1',
      advance,
      assert.fail,
    ),
  )
  state.start()
  t.mock.timers.tick(12_000)
  state.stop()
  state.start()
  finish(false)
  await settle()
  assert.equal(state.active.value, true)
  t.mock.timers.tick(12_000)
  assert.equal(advance.mock.callCount(), 2)
  finish(true)
  await settle()
})

test('reports a failed advance once and stops retrying', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  const error = new Error('read failed')
  const report = t.mock.fn()
  const { state } = mount(t, () =>
    useReaderAutoPage(
      () => true,
      () => '1',
      () => Promise.reject(error),
      report,
    ),
  )
  state.start()
  t.mock.timers.tick(12_000)
  await settle()
  assert.equal(state.active.value, false)
  assert.equal(report.mock.calls[0].arguments[0], error)
  t.mock.timers.tick(60_000)
  assert.equal(report.mock.callCount(), 1)
})

test('validates, remembers and applies a custom interval', async (t) => {
  t.mock.timers.enable({ apis: ['setInterval'] })
  const previousStorage = globalThis.localStorage
  const values = new Map([['reader-auto-page-interval', '5']])
  globalThis.localStorage = {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, value),
  }
  const advance = t.mock.fn(() => true)
  const { state } = mount(t, () =>
    useReaderAutoPage(
      () => true,
      () => '1',
      advance,
      assert.fail,
    ),
  )
  t.after(() => {
    globalThis.localStorage = previousStorage
  })
  assert.equal(state.intervalSeconds.value, 5)
  for (const invalid of [0, -1, 1.5, 301, NaN]) assert.equal(state.start(invalid), false)
  assert.equal(state.active.value, false)
  assert.equal(state.start(3), true)
  assert.equal(values.get('reader-auto-page-interval'), '3')
  t.mock.timers.tick(2999)
  assert.equal(advance.mock.callCount(), 0)
  t.mock.timers.tick(1)
  await settle()
  assert.equal(advance.mock.callCount(), 1)
  assert.equal(state.secondsLeft.value, 3)
  state.start(10)
  t.mock.timers.tick(9999)
  assert.equal(advance.mock.callCount(), 1)
  t.mock.timers.tick(1)
  await settle()
  assert.equal(advance.mock.callCount(), 2)
})

for (const [mode, height, width, horizontal] of [
  ['scroll', 950, 1000, false],
  ['paged', 720, 1000, false],
  ['paged', 950, 700, false],
  ['paged', 950, 1000, true],
]) {
  test(`detects chapter end along the actual axis: ${mode}, ${width}×${height}`, (t) => {
    const previousWindow = globalThis.window
    const previousStyle = globalThis.getComputedStyle
    const listeners = new Map()
    globalThis.window = {
      innerHeight: height,
      addEventListener: (type, handler) => listeners.set(type, handler),
      removeEventListener() {},
    }
    globalThis.getComputedStyle = () => ({ paddingLeft: '0', paddingRight: '0', getPropertyValue: () => '64' })
    const element = ref({
      clientWidth: width,
      clientHeight: 500,
      scrollWidth: width,
      scrollHeight: 500,
      scrollLeft: 0,
      scrollTop: 0,
      style: { setProperty() {} },
      addEventListener() {},
      removeEventListener() {},
      scrollBy: t.mock.fn(),
    })
    const { state } = mount(t, () =>
      useReaderPaging(
        element,
        () => mode,
        () => {},
        () => {},
      ),
    )
    t.after(() => {
      globalThis.window = previousWindow
      globalThis.getComputedStyle = previousStyle
    })
    // A short chapter already fits the viewport and must advance to the next chapter.
    assert.equal(state.advance(), 'end')
    // Typing or activating the auto-page controls must not also turn the text page.
    listeners.get('keydown')({ key: ' ', target: { closest: () => ({}) }, preventDefault: assert.fail })
    assert.equal(element.value.scrollBy.mock.callCount(), 0)
    if (horizontal) element.value.scrollWidth += 1000
    else element.value.scrollHeight += 1000
    assert.equal(state.advance(), 'moved')
    const options = element.value.scrollBy.mock.calls[0].arguments[0]
    assert.equal(horizontal ? options.left > 0 : options.top > 0, true)
    // 99.8% rounds to 100%, but still has text left to reveal.
    if (horizontal) element.value.scrollLeft = 998
    else element.value.scrollTop = 998
    assert.equal(state.advance(), 'moved')
    if (horizontal) element.value.scrollLeft = 999.5
    else element.value.scrollTop = 999.5
    assert.equal(state.advance(), 'end')
    element.value = null
    assert.equal(state.advance(), 'unavailable')
  })
}
