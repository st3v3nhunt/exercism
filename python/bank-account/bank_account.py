from threading import RLock

class BankAccount(object):
    def __init__(self):
        self.is_open = False
        self.balance = 0
        self.lock = RLock()

    def get_balance(self):
        if not self.is_open:
            raise ValueError('Account is not open.')
        return self.balance

    def open(self):
        if self.is_open:
            raise ValueError('Account already opened.')
        self.is_open = True
        self.balance = 0

    def deposit(self, amount):
        if not self.is_open:
            raise ValueError('Account is not open.')
        elif amount < 0:
            raise ValueError('Can not deposit a negative amount.')
        with self.lock:
            self.balance += amount

    def withdraw(self, amount):
        if not self.is_open:
            raise ValueError('Account is not open.')
        elif amount < 0:
            raise ValueError('Can not withdraw a negative amount.')
        elif self.balance < amount:
            raise ValueError('Can not withdraw more than is available.')
        with self.lock:
            self.balance -= amount

    def close(self):
        if not self.is_open:
            raise ValueError('Account is not open.')
        self.is_open = False
        self.balance = 0
