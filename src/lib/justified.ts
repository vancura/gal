import justifiedLayout from 'justified-layout'

export interface Box {
  top: number
  left: number
  width: number
  height: number
}

export interface Layout {
  boxes: Box[]
  height: number
}

/** Absolute boxes for every photo, Flickr-style justified rows. */
export function layout(
  ratios: number[],
  containerWidth: number,
  rowHeight = 240,
  gap = 4,
): Layout {
  if (ratios.length === 0 || containerWidth <= 0)
    return { boxes: [], height: 0 }
  const result = justifiedLayout(ratios, {
    containerWidth,
    targetRowHeight: rowHeight,
    boxSpacing: gap,
    containerPadding: 0,
  })
  return { boxes: result.boxes, height: result.containerHeight }
}

/**
 * Indices of boxes intersecting the viewport plus overscan. Rows share a top
 * and a height, so both `top` and `top + height` are non-decreasing across the
 * array and two binary searches find the window in O(log n).
 */
export function visibleIndices(
  boxes: Box[],
  scrollTop: number,
  viewportHeight: number,
  overscan = 600,
): number[] {
  const from = scrollTop - overscan
  const to = scrollTop + viewportHeight + overscan
  const start = lowerBound(
    boxes.length,
    (i) => boxes[i].top + boxes[i].height >= from,
  )
  const end = lowerBound(boxes.length, (i) => boxes[i].top >= to)
  return Array.from({ length: Math.max(0, end - start) }, (_, k) => start + k)
}

/** First index in [0, n) where `pred` is true, assuming it is monotonic (false..true). */
function lowerBound(n: number, pred: (i: number) => boolean): number {
  let lo = 0
  let hi = n
  while (lo < hi) {
    const mid = (lo + hi) >>> 1
    if (pred(mid)) hi = mid
    else lo = mid + 1
  }
  return lo
}
