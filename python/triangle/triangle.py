def valid_triangle(func):
    def wrapper(*args):
        valid = (min(*args) > 0 and sum(sorted(*args)[:2]) >= max(*args))
        return valid if not valid else func(*args)

    return wrapper


@valid_triangle
def is_equilateral(sides):
    return len(set(sides)) == 1

@valid_triangle
def is_isosceles(sides):
    return len(set(sides)) <= 2

@valid_triangle
def is_scalene(sides):
    return len(set(sides)) == 3
