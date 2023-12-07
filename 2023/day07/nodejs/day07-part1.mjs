#!/usr/bin/env node
import fs from 'node:fs/promises'

const cardList = 'AKQJT98765432'.split('').reverse().join('')
console.log({ cardList })

const handsOrder = [
  '11111', // Five of a kind
  '2111', // Four of a kind
  '221', // Full house
  '311', // Three of a kind
  '32', // Two pair
  '41', // Pair
  '5' // High card
]

const hexas = {
  orig: '23456789TJQKA',
  dest: '23456789ABCDE'
}

const convertToHexa = (hand) => {
  let hexaVal = ''

  for (let i = 0; i < hand.length; i++) {
    hexaVal += hexas.dest[hexas.orig.indexOf(hand[i])]
  }

  return { hexaVal, hexaNum: parseInt(hexaVal, 16) }
}

const sortNumber = (a, b) => b - a

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.trim().split('\n')

  const handsAndBids = rawLines.map(line => {
    const [hand, bid] = line.split(/\s+/)
    const handMap = new Map()
    for (let i = 0; i < 5; i++) {
      handMap.set(hand[i], (handMap.get(hand[i]) ?? 0) + 1)
    }

    const handValue = handsOrder.indexOf([...handMap.values()].sort(sortNumber).join(''))
    const { hexaVal, hexaNum } = convertToHexa(hand)

    return { hand, bid: +bid, handMap, handValue, hexaVal, hexaNum, rankBidFactor: 0 }
  })

  const sorted = handsAndBids.toSorted((a, b) => (a.handValue !== b.handValue)
    ? a.handValue - b.handValue
    : a.hexaNum - b.hexaNum
  )

  const total = sorted.reduce((acc, o, i) => {
    const pos = i + 1
    acc += (o.rankBidFactor = o.bid * pos)
    console.log(`${o.hand} ${o.bid} * ${pos} = ${o.rankBidFactor}`)

    return acc
  }, 0)

  console.log({ total })
}

main()
