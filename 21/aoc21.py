from dataclasses import dataclass
import sys

@dataclass
class Monkey:
	name: str
	val: int = None
	lname: str = None
	rname: str = None
	op: str = None

def resolve(monkeys, name):
	m = monkeys[name]
	if m.val != None:
		return m.val

	assert m.lname != None
	assert m.rname != None
	assert m.op != None
	lval = resolve(monkeys, m.lname)
	rval = resolve(monkeys, m.rname)
	if m.op == "+":
		val = lval + rval
	elif m.op == "-":
		val = lval - rval
	elif m.op == "*":
		val = lval * rval
	elif m.op == "/":
		val = lval // rval
	else:
		assert False
	m.val = val
	return val

f = open(sys.argv[1], "r")
monkeys = dict()
for line in f.readlines():
	[name, rest] = line.rstrip().split(":")
	m = Monkey(name)
	shoutlist = rest.strip().split(" ")
	if len(shoutlist) == 1:
		m.val = int(shoutlist[0])
	elif len(shoutlist) == 3:
		m.lname = shoutlist[0]
		m.op = shoutlist[1]
		m.rname = shoutlist[2]
	else:
		assert False
	monkeys[m.name] = m

print(resolve(monkeys, "root"))
