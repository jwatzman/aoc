#!/usr/bin/env python3

import sys

cals = []
calCur = 0

f = open(sys.argv[1], "r")
for line in f.readlines():
    if (line == "\n"):
        cals.append(calCur)
        calCur = 0
    else:
        calCur += int(line)

cals.sort(reverse=True)
print(cals[0] + cals[1] + cals[2])
