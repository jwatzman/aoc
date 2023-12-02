#!/usr/bin/env python3

from collections import defaultdict
import re
import sys

f = open(sys.argv[1], "r")

r = re.compile("(\d+) (red|green|blue)")
def pull_info(pull):
	m = r.match(pull)
	n = int(m.group(1))
	colour = m.group(2)
	return (colour, n)

def game_power(game):
	min_balls = defaultdict(int)
	for reveal in game.split(";"):
		for pull in reveal.strip().split(","):
			(colour, n) = pull_info(pull.strip())
			min_balls[colour] = max(min_balls[colour], n)
	return min_balls["red"] * min_balls["green"] * min_balls["blue"]

tot = 0
for line in f.readlines():
	line_split = line.split(":")
	tot += game_power(line_split[1].strip())

print(tot)
