#!/usr/bin/env node
import fs from 'node:fs'
import path from 'node:path'

const lines = fs.readFileSync(path.join(process.cwd(), process.argv[2])).toString().split('\n')
const startRegExp = /(\d).*$/
const endRegExp = /^.*(\d)/
const result = lines.map(_ => 0 | _.match(startRegExp)[1] + _.match(endRegExp)?.[1]).reduce((a, b) => a + b)
console.log(result)
