import numpy as np

def parse_arg(tree1, tree2, aln1, aln2, MCCs):
    from Bio import Phylo, AlignIO
    from Bio.Align import MultipleSeqAlignment

    t1 = Phylo.read(tree1, 'newick')
    t2 = Phylo.read(tree2, 'newick')

    MCCs = []
    with open(MCCs) as fh:
        for line in fh:
            if line.strip():
                MCCs.append(line.strip().split(','))

    a1 = {s.id:s for s in AlignIO.read(aln1, 'fasta')}
    a2 = {s.id:s for s in AlignIO.read(aln2, 'fasta')}
    all_leaves = set.union(a1.keys(), a2.keys())

    aln_combined = []
    for leaf in all_leaves:
        seq = a1[leaf] + a2[leaf]
        seq.id = leaf
        aln_combined.append(seq)

    combined_mask = np.ones(a1.get_alignment_length() + a2.get_alignment_length())
    mask1 = np.zeros(a1.get_alignment_length() + a2.get_alignment_length())
    mask2 = np.zeros(a1.get_alignment_length() + a2.get_alignment_length())
    mask1[:a1.get_alignment_length()] = 1
    mask2[a1.get_alignment_length():] = 1

    return {"MCCs": MCCs, "trees":[t1,t2], "alignment":MultipleSeqAlignment(aln_combined),
            "masks":[mask1,mask2], "combined_mask":combined_mask}


def setup_arg(T, aln, total_mask, segment_mask, dates, MCCs, gtr='JC69',
              verbose=0, fill_overhangs=True):
    from treetime import TreeTime
    from collections import defaultdict

    tt = TreeTime(dates=dates, tree=T,
                  aln=aln, gtr=gtr, verbose=verbose,
                  fill_overhangs=fill_overhangs, compress=False)


    tt.reroot("least-squares", force_positive=True)

    leaf_to_MCC = {}
    for mi,mcc in enumerate(MCCs):
        for leaf in mcc:
            leaf_to_MCC[leaf] = mi

    for leaf in tt.tree.get_terminals():
        leaf.mcc = leaf_to_MCC[leaf]

    for n in tt.tree.get_nonterminals(order='postorder'):
        n.child_mccs = set([c.mcc for c in n])

    mcc_intersection = set.intersection(*[c.child_mccs for c in tt.tree.root])
    if len(mcc_intersection):
        tt.tree.root.mcc = list(mcc_intersection)[0]
    else:
        tt.tree.root.mcc = None

    for n in tt.tree.get_nonterminals(order='preorder'):
        if n==tt.tree.root:
            continue
        else:
            if n.up.mcc in n.child_mccs:
                n.mcc = n.up.mcc
            elif len(n.child_mccs)==1:
                n.mcc = list(n.child_mcc)[0]
            else:
                n.mcc is None

    for n in tt.find_clades():
        if n.up and n.up.mcc==n.mcc:
            n.mask=total_mask
        else:
            n.mask = segment_mask

    return tt