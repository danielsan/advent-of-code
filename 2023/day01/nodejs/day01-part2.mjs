#!/usr/bin/env node
import fs from 'node:fs'
import path from 'node:path'

const lines = fs.readFileSync(path.join(process.cwd(), process.argv[2])).toString().split('\n')
const numberList = 'one,two,three,four,five,six,seven,eight,nine,ten'.split(',')
const startRegExp = new RegExp(`(${['\\d', ...numberList].join('|')}).*$`)
const endRegExp = new RegExp(`^.*(${['\\d', ...numberList].join('|')})`)

const result = lines.map((_, i) => {
  const firstMatch = _.match(startRegExp)[1]
  const secondMatch = _.match(endRegExp)[1]

  const _1stDigit = numberList.indexOf(firstMatch) + 1 || firstMatch
  const _2ndDigit = numberList.indexOf(secondMatch) + 1 || secondMatch

  return 0 | `${_1stDigit}${_2ndDigit}`
}).reduce((a, b) => a + b)

console.log(result)
