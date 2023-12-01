import sys

def snafu_char_to_digit(c):
	if c == "=":
		return -2
	elif c == "-":
		return -1
	elif c == "0":
		return 0
	elif c == "1":
		return 1
	elif c == "2":
		return 2
	else:
		assert False

def digit_to_snafu_char(n):
	if n == -2:
		return "="
	elif n == -1:
		return "-"
	elif n == 0:
		return "0"
	elif n == 1:
		return "1"
	elif n == 2:
		return "2"
	else:
		assert False

def snafu_str_to_dec(s):
	i = len(s) - 1
	factor = 1
	result = 0
	while i >= 0:
		result += factor * snafu_char_to_digit(s[i])
		i -= 1
		factor *= 5
	return result

def dec_to_reversed_base5_digits(n):
	digits = []
	while n:
		digits.append(n % 5)
		n //= 5
	return digits

def dec_to_snafu_str(n):
	chars = []
	carry = 0
	for digit in dec_to_reversed_base5_digits(n):
		digit += carry
		carry = 0
		while digit > 2:
			digit -= 5
			carry += 1
		chars.append(digit_to_snafu_char(digit))
	if carry != 0:
		chars.append(digit_to_snafu_char(carry))
	return "".join(reversed(chars))

testcases = [
	("1", 1),
	("2", 2),
	("1=", 3),
	("1-", 4),
	("10", 5),
	("11", 6),
	("12", 7),
	("2=", 8),
	("2-", 9),
	("20", 10),
	("1=0", 15),
	("1-0", 20),
	("1=11-2", 2022),
	("1-0---0", 12345),
	("1121-1110-1=0", 314159265),
]

for (snafu_str, dec) in testcases:
	assert snafu_str_to_dec(snafu_str) == dec
	assert dec_to_snafu_str(dec) == snafu_str

tot = 0
f = open(sys.argv[1], "r")
for line in f.readlines():
	tot += snafu_str_to_dec(line.rstrip())
print(dec_to_snafu_str(tot))
