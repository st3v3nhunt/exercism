import random
from string import ascii_uppercase

class Robot(object):
    def __init__(self):
        self.reset()

    def reset(self):
        random.seed()
        self.name = f"{''.join(random.sample(ascii_uppercase, 2))}{random.randrange(100,999)}"
