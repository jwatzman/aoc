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
	ore: int
	clay: int
	obs: int
	ge: int
	orerob: int
	clayrob: int
	obsrob: int
	gerob: int

	def step(self, blueprint, accum, enable_orerob, enable_clayrob, enable_obsrob, enable_gerob):
		newore = self.ore + self.orerob
		newclay = self.clay + self.clayrob
		newobs = self.obs + self.obsrob
		newge = self.ge + self.gerob
		accum.add(State(newore, newclay, newobs, newge, self.orerob, self.clayrob, self.obsrob, self.gerob))
		if enable_orerob and self.ore >= blueprint.orerob_ore:
			accum.add(State(newore - blueprint.orerob_ore, newclay, newobs, newge, self.orerob + 1, self.clayrob, self.obsrob, self.gerob))
		if enable_clayrob and self.ore >= blueprint.clayrob_ore:
			accum.add(State(newore - blueprint.clayrob_ore, newclay, newobs, newge, self.orerob, self.clayrob + 1, self.obsrob, self.gerob))
		if enable_obsrob and self.ore >= blueprint.obsrob_ore and self.clay >= blueprint.obsrob_clay:
			accum.add(State(newore - blueprint.obsrob_ore, newclay - blueprint.obsrob_clay, newobs, newge, self.orerob, self.clayrob, self.obsrob + 1, self.gerob))
		if enable_gerob and self.ore >= blueprint.gerob_ore and self.obs >= blueprint.gerob_obs:
			accum.add(State(newore - blueprint.gerob_ore, newclay, newobs - blueprint.gerob_obs, newge, self.orerob, self.clayrob, self.obsrob, self.gerob + 1))

def stepall(blueprint, accum, enable_orerob, enable_clayrob, enable_obsrob, enable_gerob):
	r = set()
	while accum:
		state = accum.pop()
		state.step(blueprint, r, enable_orerob, enable_clayrob, enable_obsrob, enable_gerob)
		del state
	return r

def sumto(n):
	return n * (n + 1) / 2

def prune(blueprint, states, remsteps):
	#print("p", end="", flush=True)
	do_nothing_ge = -1
	do_nothing_state = None
	for state in states:
		ge = state.ge + state.gerob * remsteps
		if ge > do_nothing_ge:
			do_nothing_ge = ge
			do_nothing_state = state

	r = set()
	while states:
		state = states.pop()

		# Produce one new obsrob every step
		potential_obs = state.obs + state.obsrob * remsteps + sumto(remsteps)
		# Which lets us produce only so many gerob
		potential_gerob = potential_obs // blueprint.gerob_obs

		# Number of gerob we can produce is limited by obs and by time
		potential_gerob = min(remsteps, potential_gerob)

		# Produce one new gerob every step
		potential_ge = state.ge + state.gerob * remsteps + sumto(potential_gerob)

		if potential_ge >= do_nothing_ge:
			r.add(state)
		#else:
			#print(".", end = "", flush=True)
		del state
	#print("p", end="", flush=True)
	return r

r = re.compile("Blueprint (.+): Each ore robot costs (.+) ore. Each clay robot costs (.+) ore. Each obsidian robot costs (.+) ore and (.+) clay. Each geode robot costs (.+) ore and (.+) obsidian.")

blueprints = []
f = open(sys.argv[1], "r")
for line in f.readlines():
	m = r.match(line)
	args = map(int, m.groups())
	blueprints.append(Blueprint(*args))

NSTEPS = int(sys.argv[2])

tot = 1
for blueprint in blueprints:
	print(blueprint)
	states = set()
	states.add(State(0, 0, 0, 0, 1, 0, 0, 0))
	for stepnum in range(NSTEPS):
		print(stepnum, end=" ", flush=True)
		newstates = stepall(blueprint, states, stepnum < NSTEPS - 4, stepnum < NSTEPS - 3, stepnum < NSTEPS - 2, stepnum < NSTEPS - 1)
		del states
		states = newstates

		remsteps = NSTEPS - stepnum
		newstates = prune(blueprint, states, remsteps - 1)
		del states
		states = newstates
	m = 0
	while states:
		s = states.pop()
		if s.ge > m:
			m = s.ge
		del s
	print("-> ", m)
	tot *= m

print(tot)
