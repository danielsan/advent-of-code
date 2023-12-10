#!/usr/bin/env node
import fs from 'node:fs/promises'

/**
 * reads a file and returns its contents as a string
 * @param {string} filename
 * @returns string
 */
export async function readInputFile (filename) {
  /**
   * @type {Buffer}
   */
  const fileContent = await fs.readFile(filename, 'utf8')

  return fileContent.toString()
}

export class CommandLineArgs {
  inputFilename = ''
  startingNodes = []
  initialCount = 0
  initialDirectionIndex = 0

  constructor () {
    this.inputFilename = process.argv[2]
    this.startingNodes = process.argv[3]?.split(/\W+/).filter(_ => /^\w{3}$/.test(_)) ?? []
    this.initialCount = +process.argv[4] || 0
    this.initialDirectionIndex = +process.argv[5] || 0
  }
}

class Node {
  L = ''
  R = ''

  constructor (L, R) {
    this.L = L
    this.R = R
  }
}

export const createMap = (rawLines) => {
  /**
   * @type {Map<string, Node>}
   */
  const map = new Map()
  const startingNodes = []

  for (const line of rawLines) {
    if (line === '') break

    const [, key, L, R] = line.match(/^(\w+)\s+=\s+\((\w+)\W+(\w+)/)
    if (key.endsWith('A')) {
      startingNodes.push(key)
    }

    map.set(key, new Node(L, R))
  }

  return { map, startingNodes }
}

function * loopSequenceGenerator (sequence) {
  let i = 0
  const seqLen = sequence.length

  while (true) {
    yield sequence[i++]

    if (i === seqLen) i = 0
  }
}

const processStep1 = (map, firslLine) => {
  let key = 'AAA'
  let total = 0
  for (const diretion of loopSequenceGenerator(firslLine.trim())) {
    total++

    const val = map.get(key)[diretion]

    if (val === 'ZZZ') break

    key = val
  }

  return total
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

  const firslLine = rawLines.shift()
  // console.log({ firslLine })
  rawLines.shift() // removes empty line

  console.time('createMap')
  const { map } = createMap(rawLines)
  console.timeEnd('createMap')

  console.time('processStep1')
  const total = processStep1(map, firslLine)
  console.timeEnd('processStep1')

  console.log({ total })
}

import.meta.url.endsWith(process.argv[1]) && main()
