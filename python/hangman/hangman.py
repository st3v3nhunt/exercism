# Game status categories
# Change the values as you see fit
STATUS_WIN = "win"
STATUS_LOSE = "lose"
STATUS_ONGOING = "ongoing"


class Hangman(object):
    def __init__(self, word):
        self.remaining_guesses = 9
        self.status = STATUS_ONGOING
        self.word = word
        self.masked_word = '_' * len(word)
        self.correct_guesses = []

    def guess(self, char):
        if self.status != STATUS_ONGOING:
            raise ValueError('The game is no longer running.')

        if char in self.word and char not in self.correct_guesses:
            self.correct_guesses.append(char)
            self.masked_word = self.word
            for letter in self.masked_word:
                if letter not in self.correct_guesses:
                    self.masked_word = self.masked_word.replace(letter, '_')

            if '_' not in self.masked_word:
                self.status = STATUS_WIN
        else:
            self.remaining_guesses -= 1
            if self.remaining_guesses < 0:
                self.status = STATUS_LOSE

    def get_masked_word(self):
        return self.masked_word

    def get_status(self):
        return self.status
