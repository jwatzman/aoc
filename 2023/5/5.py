#!/usr/bin/env python3

from functools import reduce
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

"""
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
"""

def translate_ranges(seed_ranges, all_maps):
	cur = seed_ranges
	for m in all_maps:
		new_ranges = []
		for seed_range in cur:
			new_ranges.extend(translate_one_range(seed_range, m))
		cur = new_ranges
	return cur

def translate_one_range(seed_range, m):
	[seed_start, seed_len] = seed_range
	seed_end = seed_start + seed_len
	for line in m:
		[dest_start, src_start, length] = line
		src_end = src_start + length
		if seed_start >= src_end or src_start >= seed_end:
			continue
		elif seed_start >= src_start and seed_end <= src_end:
			return [(dest_start + (seed_start - src_start), seed_len)]
		else:
			seed_splits = []
			if seed_start < src_start:
				seed_splits.append((seed_start, src_start - seed_start))
			intersec_start = max(seed_start, src_start)
			intersec_end = min(seed_end, src_end)
			seed_splits.append((intersec_start, intersec_end - intersec_start))
			if seed_end > src_end:
				seed_splits.append((src_end, seed_end - src_end))
			assert reduce(lambda n, s: n + s[1], seed_splits, 0) == seed_len
			res = []
			for split in seed_splits:
				res.extend(translate_one_range(split, m))
			return res
	return [seed_range]

f = open(sys.argv[1], "r")
lines = f.readlines()

seed_ints = list(map(int, lines.pop(0)[7:].split(" ")))
seed_ranges = []
while len(seed_ints) > 0:
	start = seed_ints.pop(0)
	length = seed_ints.pop(0)
	seed_ranges.append((start, length))
lines.pop(0)

all_maps = []
while len(lines) > 0:
	all_maps.append(read_one_map(lines))

translated_ranges = translate_ranges(seed_ranges, all_maps)
print(min(map(lambda r: r[0], translated_ranges)))
