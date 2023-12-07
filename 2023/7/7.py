#!/usr/bin/env python3

from functools import cmp_to_key
import sys

def read_hands():
	res = []
	f = open(sys.argv[1], "r")
	for line in f.readlines():
		[cards, bid_str] = line.split(" ")
		bid = int(bid_str)
		res.append((cards, bid, strength(cards)))
	return res

def strength(cards):
	card_cnts = dict()
	jokers = 0
	for card in cards:
		if card == "J":
			jokers += 1
		elif card in card_cnts:
			card_cnts[card] += 1
		else:
			card_cnts[card] = 1
	cnts = list(reversed(sorted(card_cnts.values())))
	if len(cnts) > 0:
		cnts[0] += jokers
	else:
		cnts = [jokers]
	if cnts[0] == 5:
		return 7
	elif cnts[0] == 4:
		return 6
	elif cnts[0] == 3 and cnts[1] == 2:
		return 5
	elif cnts[0] == 3:
		return 4
	elif cnts[0] == 2 and cnts[1] == 2:
		return 3
	elif cnts[0] == 2:
		return 2
	else:
		return 1

def card_val(card):
	if card == "T":
		return 10
	elif card == "J":
		return 1
	elif card == "Q":
		return 12
	elif card == "K":
		return 13
	elif card == "A":
		return 14
	else:
		return int(card)

def cmp_hands(h1, h2):
	if h2[2] > h1[2]:
		return 1
	elif h2[2] < h1[2]:
		return -1
	for p in zip(h1[0], h2[0]):
		v1 = card_val(p[0])
		v2 = card_val(p[1])
		if v2 > v1:
			return 1
		elif v2 < v1:
			return -1
	return 0

hands = read_hands()
hands.sort(key=cmp_to_key(cmp_hands))
hands = list(reversed(hands))

rank = 1
tot = 0
for hand in hands:
	tot += rank * hand[1]
	rank += 1

print(tot)
