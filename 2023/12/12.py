#!/usr/bin/env python3

import sys

"""
class SpringFail(Exception):
	pass
"""

def parse_line(l):
	[springs_str, groups_str] = l.strip().split(" ")
	springs = filter(lambda x: x != "", springs_str.split("."))
	groups = map(int, groups_str.split(","))
	return (list(springs), list(groups))

def is_all_unknown(s):
	return all(map(lambda c: c == "?", s))

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

	if (len(spring_block) == group_size):
		solns += solve(springs[1:], groups[1:])
	elif (len(spring_block) > group_size and spring_block[group_size] == "?"):
		solns += solve([spring_block[(group_size+1):], *springs[1:]], groups[1:])

	if spring_block[0] == "?":
		solns += solve([spring_block[1:], *springs[1:]], groups)

	return solns

f = open(sys.argv[1], "r")
tot = 0
for line in f.readlines():
	(springs, groups) = parse_line(line)
	tot += solve(springs, groups)

print(tot)
