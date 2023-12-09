#!/usr/bin/env python3

import sys

def is_all_zeroes(l):
	for i in l:
		if i != 0:
			return False
	return True

def solve(input):
	pyramid = []
	pyramid.append(list(map(int, input.split(" "))))
	
	while (not is_all_zeroes(pyramid[-1])):
		cur = pyramid[-1]
		next = []
		for i in range(len(cur) - 1):
			next.append(cur[i+1] - cur[i])
		pyramid.append(next)

	for i in reversed(range(len(pyramid) - 1)):
		pyramid[i].append(pyramid[i][-1] + pyramid[i+1][-1])

	return pyramid[0][-1]

f = open(sys.argv[1], "r")
tot = 0
for line in f.readlines():
	tot += solve(line.strip())

print(tot)
