#!/usr/bin/env python3

import sys

"""
class SpringFail(Exception):
	pass
"""

UNFURLING_FACTOR = 5

def parse_line(l):
	[springs_str, groups_str] = l.strip().split(" ")
	unfurled_springs_str = "?".join([springs_str] * UNFURLING_FACTOR)
	springs = filter(lambda x: x != "", unfurled_springs_str.split("."))
	groups = map(int, groups_str.split(","))
	return (list(springs), list(groups) * UNFURLING_FACTOR)

def is_all_unknown(s):
	for c in s:
		if c != "?":
			return False
	return True

def solve(springs, groups):
	if len(springs) == 0 and len(groups) == 0:
		return 1
	if len(springs) == 0:
		return 0
	if len(groups) == 0:
		if is_all_unknown("".join(springs)):
			return 1
		else:
			return 0

	spring_block = springs[0]
	group_size = groups[0]

	if spring_block == "":
		return solve(springs[1:], groups)

	solns = 0

	if group_size > len(spring_block):
		if is_all_unknown(spring_block):
			return solve(springs[1:], groups)
		else:
			return 0

	if (len(spring_block) == group_size):
		solns += solve(springs[1:], groups[1:])
		if (is_all_unknown(spring_block)):
			solns += solve(springs[1:], groups)
		return solns

	if (len(spring_block) > group_size and spring_block[group_size] == "?"):
		solns += solve([spring_block[(group_size+1):], *springs[1:]], groups[1:])

	if spring_block[0] == "?":
		solns += solve([spring_block[1:], *springs[1:]], groups)

	return solns

f = open(sys.argv[1], "r")
i = 0
tot = 0
for line in f.readlines():
	i += 1
	print(i)
	(springs, groups) = parse_line(line)
	tot += solve(springs, groups)

print(tot)
