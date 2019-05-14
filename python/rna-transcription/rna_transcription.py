def to_rna(dna_strand):
    in_table = 'GCTA'
    out_table = 'CGAU'
    translation_table = ''.maketrans(in_table, out_table)
    return dna_strand.translate(translation_table)
