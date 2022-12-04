#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
cnt = 0
for line in f.readlines():
	line = line.rstrip()
	(first_st, first_end), (second_st, second_end) = map(lambda x: x.split("-"), line.split(","))
	first_st = int(first_st)
	first_end = int(first_end)
	second_st = int(second_st)
	second_end = int(second_end)

	if first_st >= second_st and first_end <= second_end:
		cnt += 1
	elif second_st >= first_st and second_end <= first_end:
		cnt += 1

print(cnt)
