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

	def __getitem__(self, item):
		return getattr(self, item)

class AlwaysRule:
	def __init__(self, r):
		self.r = get_r(r)

	def run(self, w):
		return self.r

	def __repr__(self):
		return "Always goto %s" % (self.r)

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
		if self.op == "<" and wv < self.v:
			return self.r
		elif self.op == ">" and wv > self.v:
			return self.r
		else:
			return None

	def __repr__(self):
		return "If %s %s %d goto %s" % (self.field, self.op, self.v, self.r)

workflow_reg = re.compile("(.*){(.*)}")
class Workflow:
	def __init__(self, line):
		m = workflow_reg.match(line)
		self.name = m.group(1)
		self.rules = []
		for rule_str in m.group(2).split(","):
			if ">" in rule_str or "<" in rule_str:
				self.rules.append(CondRule(rule_str))
			else:
				self.rules.append(AlwaysRule(rule_str))

	def run(self, w):
		for rule in self.rules:
			res = rule.run(w)
			if res != None:
				return res
		assert False

	def __repr__(self):
		return "Workflow %s: %s" % (self.name, str(self.rules))

def run_all(workflows, w, state):
	new_state = workflows[state].run(w)
	if new_state == True or new_state == False:
		return new_state
	else:
		return run_all(workflows, w, new_state)

workflows = dict()
f = open(sys.argv[1], "r")
lines = f.readlines()
while len(lines) > 0:
	line = lines.pop(0).strip()
	if line == "":
		break
	flow = Workflow(line)
	workflows[flow.name] = flow

tot = 0
widget_reg = re.compile("{x=(.+),m=(.+),a=(.+),s=(.+)}")
for line in lines:
	m = widget_reg.match(line.strip())
	w = Widget(int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4)))
	accept = run_all(workflows, w, "in")
	if accept:
		#print(w)
		tot += w.x + w.m + w.a + w.s

print(tot)
