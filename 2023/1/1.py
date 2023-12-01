#!/usr/bin/env python3

import regex as re
import sys

def tonum(s):
	if s == "one":
		return 1
	elif s == "two":
		return 2
	elif s == "three":
		return 3
	elif s == "four":
		return 4
	elif s == "five":
		return 5
	elif s == "six":
		return 6
	elif s == "seven":
		return 7
	elif s == "eight":
		return 8
	elif s == "nine":
		return 9
	else:
		return int(s)

reg = re.compile('(?:one)|(?:two)|(?:three)|(?:four)|(?:five)|(?:six)|(?:seven)|(?:eight)|(?:nine)|1|2|3|4|5|6|7|8|9')

f = open(sys.argv[1], "r")

tot = 0
for line in f.readlines():
	matches = reg.findall(line, overlapped=True)
	print(line)
	print(matches[0])
	print(matches[-1])
	print(10 * tonum(matches[0]) + tonum(matches[-1]))
	print('---')
	tot += 10 * tonum(matches[0]) + tonum(matches[-1])

print(tot)
