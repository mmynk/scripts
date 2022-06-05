def split(breakup, total, subtotal):
	tax = total - subtotal
	for i, item in breakup.items():
		amount = breakup[i]['amount']
		breakup[i]['total'] = amount + (amount * tax / total)
	return breakup

def main():
	total = float(input('Enter total bill amount: '))
	n = int(input('How many people are involved in the transaction? '))
	breakup = {}
	subtotal = 0
	for i in range(n):
		amounts = input(f'Enter individual item prices separated by space for person {i+1}:\n')
		amount = sum(map(float, amounts.split(' ')))
		subtotal += amount
		breakup[i] = {
			'amount': amount
		}
	print(f'Subtotal={subtotal} Tax={total - subtotal}')
	print(split(breakup, total, subtotal))

if __name__ == '__main__':
	main()
