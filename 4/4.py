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

	if first_end < second_st:
		pass
	elif second_end < first_st:
		pass
	else:
		cnt += 1

print(cnt)
