#!/usr/bin/env python3

import re
import sys

f = open(sys.argv[1], "r")

r = re.compile("(\d+) (red|green|blue)")
def is_valid_pull(pull):
	m = r.match(pull)
	n = int(m.group(1))
	colour = m.group(2)
	if colour == "red" and n > 12:
		return False
	elif colour == "green" and n > 13:
		return False
	elif colour == "blue" and n > 14:
		return False
	else:
		return True

def is_valid_game(game):
	for reveal in game.split(";"):
		for pull in reveal.strip().split(","):
			if not is_valid_pull(pull.strip()):
				return False
	return True

tot = 0
for line in f.readlines():
	line_split = line.split(":")
	if is_valid_game(line_split[1].strip()):
		gamenum = int(line_split[0][5:])
		tot += gamenum

print(tot)
