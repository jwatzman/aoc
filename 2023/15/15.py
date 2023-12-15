#!/usr/bin/env python3

import sys

def hashalgo(s):
	cv = 0
	for c in s:
		cv += ord(c)
		cv *= 17
		cv %= 256
	return cv

f = open(sys.argv[1], "r")
line = f.readline().strip()

tot = 0
for step in line.split(","):
	tot += hashalgo(step)
print(tot)
