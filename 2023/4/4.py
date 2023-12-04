#!/usr/bin/env python3

import sys

def mk_set(s):
	res = set()
	for n in s.split(" "):
		if n == "":
			continue
		else:
			res.add(int(n))
	return res

def score(n):
	if n == 0:
		return 0
	else:
		return 2 ** (n-1)

f = open(sys.argv[1], "r")

tot = 0
for line in f.readlines():
	line_split = line.split(":")
	numbers_split = line_split[1].strip().split("|")
	winning_numbers_set = mk_set(numbers_split[0].strip())
	my_numbers_set = mk_set(numbers_split[1].strip())
	i = winning_numbers_set.intersection(my_numbers_set)
	tot += score(len(i))
print(tot)
