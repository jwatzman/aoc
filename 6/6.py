#!/usr/bin/env python3

import sys

f = open(sys.argv[1], "r")
line = f.readlines()[0]

uniq = 14
i = uniq
while i <= len(line):
	if len(set(line[i-uniq:i])) == uniq:
		print(i)
		sys.exit(0)
	i += 1
