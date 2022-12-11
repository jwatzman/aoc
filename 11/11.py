#!/usr/bin/env python3

import sys

ALL_MONKEYS = []
ALL_DIV = 1

def stripint(x):
	return int(x.strip())

def mkop(opline):
	expr = opline.split("=")[1].strip().split(" ")
	op = expr[1]
	if op == "+":
		n = int(expr[2])
		return lambda old: old + n
	elif op == "*":
		if expr[2] == "old":
			return lambda old: old * old
		else:
			n = int(expr[2])
			return lambda old: old * n
	else:
		print("Invalid op " + op)
		sys.exit(1)

class Monkey:
	def __init__(self, config):
		global ALL_DIV
		lines = config.split("\n")
		self.items = list(map(stripint, lines[1].split(":")[1].split(",")))
		self.op = mkop(lines[2])
		self.div_test = stripint(lines[3].split("by")[1])
		ALL_DIV *= self.div_test
		self.true_throw = stripint(lines[4].split("monkey")[1])
		self.false_throw = stripint(lines[5].split("monkey")[1])
		self.inspected = 0

	def catch(self, item):
		self.items.append(item)

	def turn(self):
		while len(self.items) > 0:
			global ALL_DIV
			self.inspected += 1
			item = self.items.pop(0)
			item = self.op(item)
			item = item % ALL_DIV
			if item % self.div_test == 0:
				throw_to = self.true_throw
			else:
				throw_to = self.false_throw
			ALL_MONKEYS[throw_to].catch(item)

	def __str__(self):
		lines = []
		lines.append("Items: " + str(self.items))
		lines.append("Op on input 2: " + str(self.op(2)))
		lines.append("Test divisible by " + str(self.div_test))
		lines.append("If true, " + str(self.true_throw))
		lines.append("If false, " + str(self.false_throw))
		return "\n".join(lines)

configs = open(sys.argv[1], "r").read().split("\n\n")
for config in configs:
	ALL_MONKEYS.append(Monkey(config))

NROUNDS = 10000
for r in range(NROUNDS):
	for monkey in ALL_MONKEYS:
		monkey.turn()

inspected = list(map(lambda m: m.inspected, ALL_MONKEYS))
inspected.sort(reverse=True)
print(inspected[0] * inspected[1])
