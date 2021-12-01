import os
let fileContent = readFile(paramStr(1)) 

from strutils import parseInt, splitLines
from sequtils import map

let lines = splitLines(fileContent).map(parseInt)
if lines.len < 3:
    echo("les than 3 lines")
    system.quit(1)

var count = 0
var prev = -1
for i in 0..(lines.len - 3):
    let cur = lines[i] + lines[i + 1] + lines[i + 2]
    if cur > prev and prev != -1:
        count = count + 1
    prev = cur

echo(count)