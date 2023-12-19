#!/usr/bin/env python3

from dataclasses import dataclass
import re
import sys

def get_r(r):
	if r == "A":
		return True
	elif r == "R":
		return False
	else:
		return r

@dataclass(frozen=True)
class Widget:
	x: int
	m: int
	a: int
	s: int

class AlwaysRule:
	def __init__(self, r):
		self.r = get_r(r)

	def run(self, w):
		return r

rule_reg = re.compile("(.+)(<|>)(.+):(.+)")
class CondRule:
	def __init__(self, r):
		m = rule_reg.match(r)
		self.field = m.group(1)
		self.op = m.group(2)
		self.v = int(m.group(3))
		self.r = get_r(m.group(4))

	def run(self, w):
		wv = w[self.field]
		if op == "<" and vw < self.v:
			return self.r
		elif op == ">" and vw > self.v:
			return self.r
		else:
			return None

workflow_reg = re.compile("(.*){(.*)}")
class Workflow:
	def __init__(self, line):
		m = workflow_reg.match(line)
		self.name = m.group(1)
		self.rules = []
		for rule_str in m.group(2).split(":"):
			if rule_str.contains(">") or rule_str.contains("<"):
				self.rules.append(CondRule(rule_str))
			else:
				self.rules.append(AlwaysRule(rule_str))

	def run(self, w):
		for rule in self.rules:
			res = rule.run(w)
			if res != None:
				return res
		assert False

f = open(sys.argv[1], "r")
print(Workflow(f.readlines()[0]))
