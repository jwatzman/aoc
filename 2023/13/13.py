#!/usr/bin/env python3

import sys

def parse_file():
	res = []
	cur = []
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		line_strip = line.strip()
		if line_strip == "":
			if len(cur) > 0:
				res.append(cur)
				cur = []
		else:
			cur.append(line_strip)
	if len(cur) > 0:
		res.append(cur)
	return res

def rot(l):
	return list(zip(*l[::-1]))

def find_reflect(pattern):
	for reflect_line_after in range(len(pattern) - 1):
		okay = True
		for dist in range(reflect_line_after + 1):
			if reflect_line_after - dist < 0 or reflect_line_after + dist + 1 >= len(pattern):
				# Probably a smarter way to set the range of dist...
				break
			if pattern[reflect_line_after - dist] != pattern[reflect_line_after + dist + 1]:
				okay = False
				break
		if okay:
			return reflect_line_after + 1
	return None

def solve(pattern):
	row_reflect = find_reflect(pattern)
	if row_reflect != None:
		return row_reflect * 100
	col_reflect = find_reflect(rot(pattern))
	assert col_reflect != None
	return col_reflect

tot = 0
patterns = parse_file()
for pattern in patterns:
	tot += solve(pattern)
print(tot)
