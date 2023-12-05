#!/usr/bin/env node
import fs from 'node:fs/promises'

class NeighborCollection {
  neighborsMap = new Map()

  constructor () {
    this.neighborsMap = new Map()
  }

  get size () {
    return this.neighborsMap.size
  }

  add (neighbor) {
    const { num, start, end, lineIndex } = neighbor
    const key = `${lineIndex}:${start}:${end}`
    const found = this.neighborsMap.get(key)

    if (found != null) {
      if (found.num !== num) {
        throw new Error(`Found ${found} and ${num} at ${start}, ${end}`)
      }

      return found
    }


    this.neighborsMap.set(key, neighbor)

    return neighbor
  }
}

class Neighbor {
  num = ''
  start = -1
  end = -1
  lineIndex = -1

  constructor (num, start, end, lineIndex) {
    this.num = num
    this.start = start
    this.end = end
    this.lineIndex = lineIndex
  }
}

class Gear {
  neighbors = new NeighborCollection()

  constructor (rowBlock, ix, lineIndex) {
    this.neighbors = this.getAdjacentNumbers(rowBlock, ix, lineIndex)
  }

  isValid () {
    return this.neighbors.size === 2
  }

  get ratio () {
    if (!this.isValid()) return 0

    const [first, second] = [...this.neighbors.neighborsMap.values()]

    return first.num * second.num
  }

  getAdjacentNumbers (rowBlock, ix, lineIndex) {
    const { prevLine, currLine, nextLine } = rowBlock
    const neighbors = new NeighborCollection()

    for (let y = ix - 1; y <= ix + 1; y++) {
      if (isDigit(prevLine[y])) {
        const number = getNumberFromDigit(prevLine, y, lineIndex - 1)
        if (number != null) {
          neighbors.add(number)
        }
      }

      if (isDigit(currLine[y])) {
        const number = getNumberFromDigit(currLine, y, lineIndex)
        if (number != null) {
          neighbors.add(number)
        }
      }

      if (isDigit(nextLine[y])) {
        const number = getNumberFromDigit(nextLine, y, lineIndex + 1)
        if (number != null) {
          neighbors.add(number)
        }
      }
    }

    return neighbors
  }
}

function getNumberFromDigit (line, ix, lineIndex) {
  if (!isDigit(line[ix])) return

  let num = line[ix]
  let start = ix
  let end = ix
  let y = ix

  while (isDigit(line[--y])) {
    num = `${line[y]}${num}`
    start = y
  }

  y = ix
  while (isDigit(line[++y])) {
    num = `${num}${line[y]}`
    end = y
  }

  return new Neighbor(num, start, end, lineIndex)
}

const isDigit = (char = '.', code = char.charCodeAt(0)) => (code > 47 && code < 58)

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.split('\n')

  const columnsLen = rawLines[0].length
  const blankLine = new Array(columnsLen).fill('.').join('')
  const newLines = [blankLine, ...rawLines, blankLine]
  const rowsLen = newLines.length

  const gears = []

  for (let lineIndex = 1; lineIndex < rowsLen - 1; lineIndex++) {
    const prevLine = newLines[lineIndex - 1]
    const currLine = newLines[lineIndex]
    const nextLine = newLines[lineIndex + 1]
    const rowBlock = { prevLine, currLine, nextLine, lineIndex }

    let latest = -1
    let ix = 0

    while ((ix = currLine.indexOf('*', latest + 1)) > -1) {
      const gear = new Gear(rowBlock, ix, lineIndex)

      if (gear.isValid()) {
        gears.push(gear)
      }

      latest = ix
    }
  }

  const total = gears.reduce((total, gear) => total + gear.ratio, 0)

  console.log(total)
}

main()
