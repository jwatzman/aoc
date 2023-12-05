#!/usr/bin/env python3

import sys

def read_one_map(lines):
	m = []
	lines.pop(0)
	while len(lines) > 0:
		line = lines.pop(0).strip()
		if (line == ""):
			break
		m.append(list(map(int, line.split(" "))))
	return m

def locate_seed(seed, all_maps):
	for m in all_maps:
		seed = locate_seed_one_map(seed, m)
	return seed

def locate_seed_one_map(seed, m):
	for line in m:
		[dest_start, src_start, length] = line
		if seed >= src_start and seed < src_start + length:
			return dest_start + (seed - src_start)
	return seed

f = open(sys.argv[1], "r")
lines = f.readlines()

seeds = map(int, lines.pop(0)[7:].split(" "))
lines.pop(0)

all_maps = []
while len(lines) > 0:
	all_maps.append(read_one_map(lines))

locations = map(lambda s: locate_seed(s, all_maps), seeds)
print(min(locations))
