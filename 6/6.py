#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
line = f.readlines()[0]
i = 4
while i <= len(line):
	if len(set(line[i-4:i])) == 4:
		print(i)
		sys.exit(0)
	i += 1
