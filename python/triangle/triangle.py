def is_equilateral(sides):
    valid = valid_triangle(sides)
    return valid if not valid else len(set(sides)) == 1


def is_isosceles(sides):
    valid = valid_triangle(sides)
    return valid if not valid else len(set(sides)) <= 2


def is_scalene(sides):
    valid = valid_triangle(sides)
    return valid if not valid else len(set(sides)) == 3


def valid_triangle(sides):
    return min(sides) > 0 and sum(sorted(sides)[:2]) >= max(sides)
