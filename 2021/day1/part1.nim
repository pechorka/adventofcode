import os
let fileContent = readFile(paramStr(1)) 

import strutils

var prev = -1
var count = 0
for line in splitLines(fileContent):
    let cur = parseInt(line)
    if cur > prev and prev != -1:
        count = count + 1
    prev = cur

echo(count)