#!/usr/bin/env python3

import re
import sys

r = re.compile("Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")
targety = int(sys.argv[2])
hitx = set()
bhitx = set()

f = open(sys.argv[1], "r")
for line in f.readlines():
	m = r.match(line)
	sx = int(m.group(1))
	sy = int(m.group(2))
	bx = int(m.group(3))
	by = int(m.group(4))

	if by == targety:
		bhitx.add(bx)
	
	steps = abs(sx-bx) + abs(sy-by)
	if abs(sy-targety) <= steps:
		steps -= abs(sy-targety)
		hitx.add(sx)
		for dx in range(steps):
			hitx.add(sx+dx+1)
			hitx.add(sx-dx-1)

print(len(hitx) - len(bhitx))
