#!/usr/bin/env node
import fs from 'node:fs/promises'

async function main () {
  console.time('readFile')
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.trim().split('\n')
  console.timeEnd('readFile')

  const [time, distance] = rawLines.map(line => +line.replace(/\D+/g, ''))
  let diff = time

  console.log({ time, distance })
  console.time('loops')
  for (let j = 1; j < time; j++) {
    if (((time - j) * j) > distance) break

    diff--
  }

  for (let j = time; j--;) {
    if (((time - j) * j) > distance) break

    diff--
  }

  console.timeEnd('loops')

  console.log({ diff, loops: time - diff, ratio: (time - diff) / time })
}

main()
