def split(breakup, total, subtotal):
	tax = total - subtotal
	for i, item in breakup.items():
		amount = breakup[i]['amount']
		breakup[i]['total'] = amount * (1 + (tax / subtotal))
	return breakup

def main():
	total = float(input('Enter total bill amount: '))
	true_subtotal = input('Enter subtotal amount: ')
	true_subtotal = float(true_subtotal) if true_subtotal else 0

	n = int(input('How many people are involved in the transaction? '))
	subtotal = 0
	breakup = {}
	for i in range(n):
		amounts = input(f'Enter individual item prices separated by space for person {i+1}:\n')
		amount = 0
		if amounts:
			amount = sum(map(float, amounts.split(' ')))
		subtotal += amount
		breakup[i] = {
			'amount': amount
		}

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

if __name__ == '__main__':
	main()
