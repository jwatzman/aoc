#!/usr/bin/env python3

import sys

def read_ints(line):
	res = []
	for n in line.split(":")[1].split(" "):
		if n != "" and n != " ":
			res.append(int(n))
	return res

f = open(sys.argv[1], "r")
lines = f.readlines()

races = list(zip(read_ints(lines[0]), read_ints(lines[1])))

tot = 1
for race in races:
	ways_to_win = 0
	(max_time, target_distance) = race
	for hold_time in range(max_time+1):
		actual_distance = hold_time*(max_time-hold_time)
		if actual_distance > target_distance:
			ways_to_win += 1
	tot *= ways_to_win

print(tot)
