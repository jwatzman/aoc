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

f = open(sys.argv[1], "r")

card_matchers = dict()
max_card_num = 0
for line in f.readlines():
	line_split = line.split(":")
	card_num = int(line_split[0][4:])
	numbers_split = line_split[1].strip().split("|")
	winning_numbers_set = mk_set(numbers_split[0].strip())
	my_numbers_set = mk_set(numbers_split[1].strip())
	i = winning_numbers_set.intersection(my_numbers_set)
	card_matchers[card_num] = len(i)
	max_card_num = card_num

cards_cache = dict()
def cards(card_num):
	if card_num not in cards_cache:
		ret = card_matchers[card_num]
		for i in range(ret):
			ret += cards(card_num + i + 1)
		cards_cache[card_num] = ret
	return cards_cache[card_num]

tot = 0
for i in range(max_card_num):
	tot += cards(i + 1) + 1
print(tot)
