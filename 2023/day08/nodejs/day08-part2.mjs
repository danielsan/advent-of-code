#!/usr/bin/env node
import { readInputFile, CommandLineArgs, createMap } from './day08-part1.mjs'

const countHops = (map, initialKey, firstLine) => {
  const firstLineLen = firstLine.length
  let key = initialKey
  let total = 0
  let i = 0

  while (true) {
    const direction = firstLine[i++]

    if (i === firstLineLen) i = 0

    total++

    const node = map.get(key)
    const nextKey = node[direction]

    if (nextKey.endsWith('Z')) break

    key = nextKey
  }

  return total
}

// calculate greatest common divisor
const gcd = (a, b) => (b === 0 ? a : gcd(b, a % b))
// calculate lowest common multiple
const lcm = (a, b) => (a * b) / gcd(a, b)

function processStep2 (map, firstLine, startingNodes) {
  const resultMap = new Map()

  console.time('countHops')
  for (const key of startingNodes) {
    const total = countHops(map, key, firstLine)
    console.log(`${key}: ${total}`)
    resultMap.set(key, total)
  }
  console.timeEnd('countHops')

  console.time('lcm')
  const totalLcm = [...resultMap.values()].reduce(lcm)
  console.timeEnd('lcm')

  return totalLcm
}

async function main () {
  console.time('args')
  const args = new CommandLineArgs()
  console.timeEnd('args')

  console.time('readInputFile')
  const fileString = await readInputFile(args.inputFilename)
  console.timeEnd('readInputFile')

  console.debug({ args })
  const rawLines = fileString.trim().split('\n')

  const firstLine = rawLines.shift()
  // console.log({ firstLine })
  rawLines.shift() // removes empty line

  console.time('createMap')
  const { map, startingNodes } = createMap(rawLines)
  console.timeEnd('createMap')

  console.time('processStep2')
  const totalLcm = processStep2(map, firstLine, startingNodes)
  console.timeEnd('processStep2')

  console.log(`lowest common multiple: ${totalLcm}`)
}

import.meta.url.endsWith(process.argv[1]) && main()
