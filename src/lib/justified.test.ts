import { expect, it } from 'vitest'
import { layout, visibleIndices } from './justified'

it('lays out rows and returns exactly the boxes in the window', () => {
  const ratios = Array.from({ length: 1000 }, (_, i) => (i % 3) + 0.5)
  const lay = layout(ratios, 1000, 200, 4)
  expect(lay.boxes).toHaveLength(1000)
  expect(lay.height).toBeGreaterThan(0)

  const top = visibleIndices(lay.boxes, 0, 600, 0)
  expect(top[0]).toBe(0)
  const lastVisible = lay.boxes[top[top.length - 1]]
  expect(lastVisible.top).toBeLessThan(600)
  expect(lay.boxes[top.length].top).toBeGreaterThanOrEqual(600)

  const mid = visibleIndices(lay.boxes, 5000, 600, 0)
  expect(mid.length).toBeGreaterThan(0)
  for (const i of mid) {
    expect(lay.boxes[i].top + lay.boxes[i].height).toBeGreaterThanOrEqual(5000)
    expect(lay.boxes[i].top).toBeLessThan(5600)
  }
  expect(lay.boxes[mid[0] - 1].top + lay.boxes[mid[0] - 1].height).toBeLessThan(5000)
  expect(lay.boxes[mid[mid.length - 1] + 1].top).toBeGreaterThanOrEqual(5600)

  expect(visibleIndices(lay.boxes, lay.height + 100, 600, 0)).toHaveLength(0)
  expect(layout([], 1000).boxes).toHaveLength(0)
})
