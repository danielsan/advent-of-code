#!/usr/bin/env node
import fs from 'node:fs/promises'

class Range {
  min = 0
  max = 0
  diff = 0

  constructor (min, to, offset) {
    this.min = min
    this.max = min + offset - 1
    this.diff = to - min
  }
}

class Block {
  name = ''
  ranges = []

  constructor (name, ranges) {
    this.name = name
    this.ranges = ranges
  }

  getDestination (index) {
    const range = this.ranges.find(({ min, max }) => index >= min && index <= max)

    return index + (range?.diff ?? 0)
  }
}

const compileBlock = (block, size = 100) => {
  const [header, ...lines] = block.trim().split('\n')

  if (!header.endsWith('map:')) {
    throw new Error('Invalid block header')
  }

  const ranges = new Array(lines.length)

  for (let l = 0; l < lines.length; l++) {
    const line = lines[l]
    const [to, from, offset] = line.split(' ').map(Number)

    ranges[l] = new Range(from, to, offset)
  }

  return new Block(header.slice(0, -4), ranges)
}

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const blocks = fileString.trim().split(/\n{2,}/)

  // for (const block of blocks) console.log(block)

  const seeds = blocks.shift().slice(6).trim().split(/\D+/).map(Number)
  console.log({ seeds })

  const compiledBlocks = blocks.map(block => compileBlock(block))

  console.log({ compiledBlocks })
  const finalLocations = new Array(seeds.length)
  for (let i = 0; i < seeds.length; i++) {
    const seed = seeds[i]
    let input = seed

    const pam = new Map()

    for (const block of compiledBlocks) {
      input = block.getDestination(input)
      pam.set(block.name, input)
    }

    finalLocations[i] = { seed, location: input }
    // console.log({ seed, index, map: Object.fromEntries(map) })
    // console.log({ seed, xendi, pam: Object.fromEntries(pam) })
  }

  const closest = Math.min(...finalLocations.map(({ location }) => location))
  // console.log({ finalLocations})
  console.log({ closest })
}

main()
