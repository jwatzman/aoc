#!/usr/bin/env python3

from dataclasses import dataclass
import re
import sys

@dataclass(frozen=True)
class Blueprint:
	n: int
	orerob_ore: int
	clayrob_ore: int
	obsrob_ore: int
	obsrob_clay: int
	gerob_ore: int
	gerob_obs: int

@dataclass(frozen=True)
class State:
	blueprint: Blueprint
	ore: int
	clay: int
	obs: int
	ge: int
	orerob: int
	clayrob: int
	obsrob: int
	gerob: int

	def step(self, accum):
		newore = self.ore + self.orerob
		newclay = self.clay + self.clayrob
		newobs = self.obs + self.obsrob
		newge = self.ge + self.gerob
		accum.add(State(self.blueprint, newore, newclay, newobs, newge, self.orerob, self.clayrob, self.obsrob, self.gerob))
		if self.ore >= self.blueprint.orerob_ore:
			accum.add(State(self.blueprint, newore - blueprint.orerob_ore, newclay, newobs, newge, self.orerob + 1, self.clayrob, self.obsrob, self.gerob))
		if self.ore >= self.blueprint.clayrob_ore:
			accum.add(State(self.blueprint, newore - blueprint.clayrob_ore, newclay, newobs, newge, self.orerob, self.clayrob + 1, self.obsrob, self.gerob))
		if self.ore >= self.blueprint.obsrob_ore and self.clay >= self.blueprint.obsrob_clay:
			accum.add(State(self.blueprint, newore - blueprint.obsrob_ore, newclay - self.blueprint.obsrob_clay, newobs, newge, self.orerob, self.clayrob, self.obsrob + 1, self.gerob))
		if self.ore >= self.blueprint.gerob_ore and self.obs >= self.blueprint.gerob_obs:
			accum.add(State(self.blueprint, newore - blueprint.gerob_ore, newclay, newobs - self.blueprint.gerob_obs, newge, self.orerob, self.clayrob, self.obsrob, self.gerob + 1))

def stepall(accum):
	r = set()
	for state in accum:
		state.step(r)
	return r

r = re.compile("Blueprint (.+): Each ore robot costs (.+) ore. Each clay robot costs (.+) ore. Each obsidian robot costs (.+) ore and (.+) clay. Each geode robot costs (.+) ore and (.+) obsidian.")

blueprints = []
f = open(sys.argv[1], "r")
for line in f.readlines():
	m = r.match(line)
	args = map(int, m.groups())
	blueprints.append(Blueprint(*args))

for blueprint in blueprints:
	print(blueprint)
	states = set()
	states.add(State(blueprint, 0, 0, 0, 0, 1, 0, 0, 0))
	for s in range(24):
		print(s, end=" ", flush=True)
		states = stepall(states)
	m = max(map(lambda s: s.ge, states))
	print("-> ", m)
