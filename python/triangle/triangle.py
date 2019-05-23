def valid_triangle(func):
    def wrapper(sides):
        valid = (min(sides) > 0 and sum(sorted(sides)[:2]) >= max(sides))
        return valid if not valid else func(sides)

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
