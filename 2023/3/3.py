#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")

nums = []
symbols = dict()
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
				nums.append((n, linenum, colnum - 1))
				n = 0
			if char != ".":
				symbols[(linenum, colnum)] = True
	if n > 0:
		nums.append((n, linenum, colnum))

def is_near_symbol(num):
	for linenum in range(num[1] - 1, num[1] + 2):
		for colnum in range(num[2] - len(str(num[0])), num[2] + 2):
			if (linenum, colnum) in symbols:
				return True
	return False

tot = 0
for num in nums:
	if is_near_symbol(num):
		tot += num[0]

print(tot)
