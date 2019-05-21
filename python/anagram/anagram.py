def find_anagrams(word, candidates):
    matches = []
    word_lower = word.lower()

    for candidate in candidates:
        candidate_lower = candidate.lower()
        if candidate_lower == word_lower:
            break

        candidate_copy = list(candidate_lower)
        for letter in word_lower:
            if letter in candidate_copy:
                candidate_copy.remove(letter)
            else:
                break

        if not candidate_copy:
            matches.append(candidate)

    return matches
