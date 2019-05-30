def validate(func):
    def wrapper(integer_number):
        if integer_number < 1 or integer_number > 64:
            raise ValueError('Number must be a positive integer between 1 and 64.')
        return func(integer_number)

    return wrapper

@validate
def on_square(integer_number):
    return 2**(integer_number-1)


@validate
def total_after(integer_number):
    return sum([on_square(x+1) for x in range(integer_number)])
