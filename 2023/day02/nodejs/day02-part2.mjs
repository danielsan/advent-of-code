#!/usr/bin/env node
import fs from 'node:fs'
import path from 'node:path'
import RGB from './RGB.class.mjs'
import RGBList from './RGBList.class.mjs'

const lines = fs.readFileSync(path.join(process.cwd(), process.argv[2])).toString().split('\n')
const params = RGB.fromObject(JSON.parse(process.argv[3]))

// eslint-disable-next-line no-extend-native
Map.prototype.toJSON = function () {
  return Object.fromEntries([...this.entries()])
}

const splits = new Map(lines.map(_ => [
  +_.slice(0, _.indexOf(':')).replace('Game ', ''),
  RGBList.fromString(_.slice(_.indexOf(':') + 1).trim())
]))

const results = []
for (const [game, { list }] of splits) {
  let compatible = true

  for (const rgb of list) {
    if (rgb.hasAnyLargerThan(params)) {
      compatible = false

      break
    }
  }

  if (compatible) {
    results.push(game)
  }
}

console.log(JSON.stringify(splits, null, 2))
console.log({ results, len: results.length, sum: results.reduce((a, b) => a + b, 0) })
console.log([...splits.values()].reduce((a, b) => a + b.getPowerFrpmMaxPossibleRGB(), 0))
