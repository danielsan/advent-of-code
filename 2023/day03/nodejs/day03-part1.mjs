#!/usr/bin/env node
import fs from 'node:fs/promises'

class State {
  num = ''
  semi = false
  started = false

  reset () {
    this.num = ''
    this.semi = false
    this.started = false
  }

  saveNum (semiList, nonSemiList, x, y) {
    (this.semi ? semiList : nonSemiList).push({ n: +this.num, x, y })

    this.reset()
  }
}

const isSymbol = (char = '.', code = char != null && char.charCodeAt(0)) =>
  code !== 46 && (code < 48 || code > 57)

const isSemi = (rowBlock, y) =>
  isSymbol(rowBlock.prevLine[y - 1]) ||
  isSymbol(rowBlock.prevLine[y]) ||
  isSymbol(rowBlock.prevLine[y + 1]) ||
  isSymbol(rowBlock.currLine[y - 1]) ||
  isSymbol(rowBlock.currLine[y + 1]) ||
  isSymbol(rowBlock.nextLine[y - 1]) ||
  isSymbol(rowBlock.nextLine[y]) ||
  isSymbol(rowBlock.nextLine[y + 1])

async function main () {
  const fileString = (await fs.readFile(process.argv[2], 'utf8')).toString()
  const rawLines = fileString.split('\n')

  const setOfChars = new Set([...fileString])
  const mapOfChars = new Map([...setOfChars].filter(_ => Number.isNaN(+_)).map(_ => [_, _.charCodeAt(0)]))

  console.log(fileString)
  console.log('setOfChars', [...setOfChars])
  console.log('mapOfChars', [...mapOfChars].sort((a, b) => (a[1] - b[1])))

  const columnsLen = rawLines[0].length
  const blankLine = new Array(columnsLen).fill('.').join('')
  const newLines = [blankLine, ...rawLines, blankLine]
  const rowsLen = newLines.length

  const nonSemiList = []
  const semiList = []
  const state = new State()

  for (let x = 1; x < rowsLen - 1; x++) {
    const prevLine = newLines[x - 1]
    const currLine = newLines[x]
    const nextLine = newLines[x + 1]
    const rowBlock = { prevLine, currLine, nextLine }

    for (let y = 0; y < columnsLen; y++) {
      const c = currLine.charCodeAt(y)
      // between 48 (0) and 57 (9)
      if (c > 47 && c < 58) {
        state.started = true
        state.num = `${state.num}${currLine[y]}`

        if (!state.semi && isSemi(rowBlock, y)) {
          state.semi = true
        }
      } else if (state.started) {
        state.saveNum(semiList, nonSemiList, x, y)
      }
    }

    // adds the last number of the row
    if (state.started) {
      state.saveNum(semiList, nonSemiList, x)
    }
  }

  // console.log({ nonSemiList, semiList })
  console.log({ sum: nonSemiList.reduce((a, b) => a + b.n, 0) })
  console.log({ semiSum: semiList.reduce((a, b) => a + b.n, 0) })
  console.log({ totalLen: nonSemiList.length + semiList.length })
}

main()
