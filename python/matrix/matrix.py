class Matrix(object):
    def __init__(self, matrix_string):
        self.rows = [[int(number) for number in numbers.split(' ')] for numbers in matrix_string.split('\n')]

    def row(self, index):
        return self.rows[index - 1]

    def column(self, index):
        return [row[index - 1] for row in self.rows]
