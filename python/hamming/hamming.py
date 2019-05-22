def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError('Strands have different lengths.')

    diff = 0
    for idx, val in enumerate(strand_a):
        if val != strand_b[idx]:
            diff += 1

    return diff
