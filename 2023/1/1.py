#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
tot = 0
for line in f.readlines():
	n = 0
	for char in line:
		if char.isdigit():
			n = 10 * int(char)
			break
	for char in line[::-1]:
		if char.isdigit():
			n += int(char)
			break
	tot += n

print(tot)
