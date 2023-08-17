#!/usr/bin/env python3

import argparse


def split(breakup, total, subtotal):
	tax = total - subtotal
	for i, item in breakup.items():
		amount = breakup[i]['amount']
		breakup[i]['total'] = amount * (1 + (tax / subtotal))
	return breakup


def process(args):
	total = args.total
	true_subtotal = args.subtotal
	true_subtotal = float(true_subtotal) if true_subtotal else 0

	n = args.people
	subtotal = 0
	breakup = {}
	if args.individual:
		for i in range(n):
			amounts = input(f'Enter individual item prices separated by space for person {i+1}:\n')
			amount = 0
			if amounts:
				amount = sum(map(float, amounts.split(' ')))
			subtotal += amount
			breakup[i] = {
				'amount': amount
			}

	if args.exclude:
		for i in range(n):
			amounts = input(f'Enter items to exclude from person {i+1}:\n')
			if amounts:
				amount = sum(map(float, amounts.split(' ')))
				subtotal += amount
				for j in range(n):
					if j != i:
						breakup[j]['amount'] += amount / (n - 1)

	total_common = true_subtotal - subtotal
	common = total_common / n
	subtotal = true_subtotal
	for i in range(n):
		breakup[i]['amount'] = breakup[i]['amount'] + common

	print(f'Subtotal={subtotal} Tax={total - subtotal}, Common={total_common}')
	print(split(breakup, total, true_subtotal))


def main():
	parser = argparse.ArgumentParser(description='A better Splitwise')
	parser.add_argument('-t', '--total', type=float, help='Total amount of the bill')
	parser.add_argument('-s', '--subtotal', type=float, help='Subtotal amount of the bill')
	parser.add_argument('-p', '--people', type=int, help='Number of people involved in the transaction')
	parser.add_argument('-i', '--individual', action='store_true', help='Whether people have bought individual items')
	parser.add_argument('-e', '--exclude', action='store_true', help='Whether some items are excluded from individual people')
	args = parser.parse_args()
	process(args)


if __name__ == '__main__':
	main()
