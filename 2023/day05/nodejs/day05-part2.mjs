#!/usr/bin/env node
import fs from 'node:fs/promises'

/*
function getDestination2 (rangesArray, index) {
  const range = rangesArray.find(([min, max]) => index >= min && index <= max)

  return index + (range?.[2] ?? 0)
}
*/

const compileBlock = (block) => {
  const [header, ...lines] = block.trim().split('\n')

  if (!header.endsWith('map:')) {
    throw new Error('Invalid block header')
  }

  const ranges = new Array(lines.length)

  for (let l = 0; l < lines.length; l++) {
    const line = lines[l]
    const [to, from, offset] = line.split(' ').map(Number)

    ranges[l] = [from, from + offset - 1, to - from]
  }

  return ranges
}

const getBlocks = async () => {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()

  return fileString.trim().split(/\n{2,}/)
}

async function main () {
  const blocks = await getBlocks()
  // for (const block of blocks) console.log(block)

  const seedList = blocks.shift().slice(6).trim().split(/\D+/).map(Number)
  const compiledBlocks = blocks.map(block => compileBlock(block))

  let closestLocation = Infinity
  let index = 0

  for (let i = 0; i < seedList.length; i += 2) {
    let seed = seedList[i]
    const max = seed + seedList[i + 1]

    index++
    console.log({ index, seed, max })
    // if (index !== +process.argv[3]) continue
    if (!process.argv.includes(index.toString())) continue

    console.time(`seedGroup ${index}`)

    while (seed < max) {
      let input = seed++

      for (const block of compiledBlocks) {
        input = input + (block.find(([min, max]) => input >= min && input <= max)?.[2] ?? 0)
      }

      if (closestLocation > input) {
        closestLocation = input
      }
    }

    console.timeEnd(`seedGroup ${index}`)
    console.log(`\tclosestLocation at ${index} is [${closestLocation}]\n}]`)
  }

  console.log({ closest: closestLocation })
}

main()
