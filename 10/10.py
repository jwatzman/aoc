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
strength = 0
for uop in uops:
	cycle += 1
	if (cycle - 20) % 40 == 0:
		strength += cycle * regX
	regX += uop

print(strength)
