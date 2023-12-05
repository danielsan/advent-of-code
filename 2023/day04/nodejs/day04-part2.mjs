#!/usr/bin/env node
import fs from 'node:fs/promises'

const splitList = list => [...new Set(list.trim().split(/\D+/).sort((a, b) => +a - +b))]

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.trim().split('\n')
  const docs = new Array(rawLines.length)

  // console.log(rawLines)
  let totalPoints = 0
  let allCopies = []

  const listLen = rawLines.length
  for (let i = listLen; i-- > 0;) {
    const line = rawLines[i]
    const [card, winNumbers, myNumbers] = line.split(/[:|]/)

    const doc = {
      card,
      winNumbers: splitList(winNumbers),
      myNumbers: splitList(myNumbers),
      matchings: [],
      points: 0,
      copies: []
    }
    docs[i] = doc

    for (const number of doc.winNumbers) {
      if (doc.myNumbers.includes(number)) {
        doc.matchings.push(number)
      }
    }

    doc.points = 0 | (2 ** (doc.matchings.length - 1))
    totalPoints += doc.points

    for (let y = i + 1, x = 0; y < listLen && x < doc.matchings.length; y++, x++) {
      doc.copies = doc.copies.concat(docs[y].copies, y + 1)
    }

    allCopies = allCopies.concat(i, doc.copies)
    docs[i] = doc
  }

  // console.log(docs)
  console.log({ totalPoints })
  console.log({
    allCopies,
    allCopiesLen: allCopies.length,
    map: Object.fromEntries(allCopies.reduce((map, x) => map.set(x, map.get(x) + 1 || 1), new Map()))
  })
}

main()
