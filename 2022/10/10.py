#!/usr/bin/env python3

import sys

uops = []
f = open(sys.argv[1], "r")
for line in f.readlines():
	spl = line.rstrip().split(" ")
	if spl[0] == "noop":
		uops.append(0)
	elif spl[0] == "addx":
		uops.append(0)
		uops.append(int(spl[1]))

cycle = 0
regX = 1
for uop in uops:
	cycle += 1
	hpos = (cycle - 1) % 40
	if regX == hpos or regX - 1 == hpos or regX + 1 == hpos:
		print("#", end="")
	else:
		print(".", end="")
	if hpos == 39:
		print("")
	regX += uop
