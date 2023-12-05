#!/usr/bin/env node
import fs from 'node:fs/promises'

const splitList = list => [...new Set(list.trim().split(/\D+/).sort((a, b) => +a - +b))]

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.trim().split('\n')

  // console.log(rawLines)
  let totalPoints = 0
  const docs = rawLines.map((line) => {
    const [card, winNumbers, myNumbers] = line.split(/[:|]/)

    const doc = {
      card,
      winNumbers: splitList(winNumbers),
      myNumbers: splitList(myNumbers),
      matchings: [],
      points: 0
    }

    for (const number of doc.winNumbers) {
      if (doc.myNumbers.includes(number)) {
        doc.matchings.push(number)
      }
    }

    doc.points = 0 | (2 ** (doc.matchings.length - 1))
    totalPoints += doc.points

    return doc
  })

  // console.log(docs)
  console.log({ totalPoints })
}

main()
