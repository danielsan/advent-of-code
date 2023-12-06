#!/usr/bin/env node
import fs from 'node:fs/promises'

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.trim().split('\n')

  const map = new Map(rawLines.map(line => line.split(':')))
  const times = [+map.get('Time').trim().replace(/\D+/g, '')]
  const distances = [+map.get('Distance').trim().replace(/\D+/g, '')]
  const results = new Array(times.length).fill(0)

  console.log({ times, distances })

  for (let i = 0; i < times.length; i++) {
    for (let j = 1; j < times[i]; j++) {
      const xdistance = (times[i] - j) * j
      if (xdistance > distances[i]) {
        results[i]++
      }
    }
  }

  console.log({ results, total: results.reduce((a, b) => a * b, 1) })
}

main()
