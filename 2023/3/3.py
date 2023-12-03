#!/usr/bin/env python3

from collections import defaultdict
import sys

f = open(sys.argv[1], "r")

nums = defaultdict(list)
gear_candidates = set()

def insert_num(num):
	for linenum in range(num[1] - 1, num[1] + 2):
		for colnum in range(num[2] - len(str(num[0])), num[2] + 2):
			nums[(linenum, colnum)].append(num)

linenum = 0
for line in f.readlines():
	linenum += 1
	n = 0
	colnum = 0
	for char in line.strip():
		colnum += 1
		if char.isdigit():
			n *= 10
			n += int(char)
		else:
			if n > 0:
				insert_num((n, linenum, colnum - 1))
				n = 0
			if char == "*":
				gear_candidates.add((linenum, colnum))
	if n > 0:
		insert_num((n, linenum, colnum))

tot = 0
for candidate in gear_candidates:
	nearby = nums[candidate]
	if len(nearby) == 2:
		tot += nearby[0][0] * nearby[1][0]

print(tot)
