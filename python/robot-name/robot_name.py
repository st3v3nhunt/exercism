import random
from string import ascii_uppercase

class Robot(object):
    def __init__(self):
        self.__name__()

    def __name__(self):
        self.name = f"{''.join(random.sample(ascii_uppercase, 2))}{random.randrange(100,999)}"

    def reset(self):
        random.seed()
        self.__name__()
