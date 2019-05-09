protein_names = {
    'AUG': 'Methionine',
    'UUU': 'Phenylalanine',
    'UUC': 'Phenylalanine',
    'UUA': 'Leucine',
    'UUG': 'Leucine',
    'UCU': 'Serine',
    'UCC': 'Serine',
    'UCA': 'Serine',
    'UCG': 'Serine',
    'UAU': 'Tyrosine',
    'UAC': 'Tyrosine',
    'UGU': 'Cysteine',
    'UGC': 'Cysteine',
    'UGG': 'Tryptophan',
    'UAA': 'STOP',
    'UAG': 'STOP',
    'UGA': 'STOP',
}

def proteins(strand):
    # split strand every 3rd
    n = 3
    codons = [(strand[i:i+n]) for i in range(0, len(strand), n)]

    p = []
    for codon in codons:
        name = protein_names[codon]
        if name == 'STOP':
            break

        p.append(name)

    return p
    # temp = [protein_names[c] for c in codons]
    # if 'STOP' in temp:
    #     i = temp.index('STOP')
    #     return temp[:i]
    # return temp
