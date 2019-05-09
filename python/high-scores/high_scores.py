def latest(scores):
    return scores[-1]


def personal_best(scores):
    scores.sort()
    return scores[-1]


def personal_top_three(scores):
    scores.sort()
    scores.reverse()

    if len(scores) < 4:
        return scores

    third = scores[2]
    return [s for s in scores if s >= third][:3]
