def is_armstrong_number(number):
    n = str(number)

    return sum(int(i) ** len(n) for i in n) == number
