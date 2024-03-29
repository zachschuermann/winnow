# winnow
[![build](https://github.com/schuermannator/winnow/workflows/build/badge.svg?branch=master)](https://github.com/schuermannator/winnow/actions)

Software source code similarity detection similar to [Moss][moss]. It is based
on incremental [winnowing][winnowing-paper] of commits.

## Our approach
We desire to detect similar code between files in different git repositories over the set of all commits. That is, we wish to be able to identify if there is shared code between any two files in any two _different_ repositories across all commits in those repositories.

TODO:
- Since we are winnowing _hunks_ rather than files, the fingerprints will not match on the boundaries. A temporary workaround is to use small ngrams until this is more properly considered.

Definitions:
- A _document_ is a set of additive changes corresponding to a hunk in a diff for a particular file in a git repository. A git repository contains a set of commits, each of which contain a number of files, themselves containing a number of _hunks_, or individual sets of changes. These additive subset of all _hunks_ in all files in all commits gives the set of all _documents_ in a repository.
- A _location_ uniquely identifies the line number of a _fingerprint_ in a _document_, given by the tuple `(repository, filename, commit hash, line number)`.
- A _fingerprint_ is a hash resulting from the application of the [winnowing][winnowing-paper] algorithm to subset (_window_) of a specific _document_. _Fingerprints_ are always associated with the _documents_ they derive from by means of their _location_.

0. Given a set of repository urls, `minp`, and `maxp`
1. Load the set of all _documents_
    - Pull all repositories
    - Per repository, read all commits
    - Per commit, read all _hunks_
    - Per _hunk_, do the following
2. Compute the set of all _fingerprints_
    - Initialize an empty vector of _fingerprints_ and _location_ tuples, `fv`
    - Given _hunk_, perform winnowing on the text, giving a set of _fingerprints_
    - Per resulting _fingerprint_, construct a _location_ given the current context of repository, filename (from commit), commit, and line number (from hunk)
    - Store in `fv`
3. Construct a reverse index on the set of _fingerprints_ `fv`
    - Initialize an empty nested hashmap of _fingerprint_ hash to hashmap of repository name to _location_, `fi`
    - Per element of `fv`, insert _fingerprint_ into `fi` using hash, repository (from _location_), and _location_ from tuple
4. Prune set of _fingerprints_
    - Initialize an empty hashmap of `(repository, filename, commit hash)` tuple to vector of _fingerprint_ hash, `fd`
    - Per `key,value` in `fv`, use _fingerprint_ hash (`key`) to query index `fi`
    - Compute _fingerprint popularity_ `p` from length of the keys of the `value` (nested) map (the number of unique repositories corresponding to the _locations_ where this _fingerprint_ hash can be found) minus one (discounting the current repository)
    - If `p` < the minimum _popularity_ cutoff `minp`, there are not enough matches across the set of _documents_ for this _fingerprint_ to be interesting to identify similarity. If `p` > the maximum _popularity_ cutoff `maxp`, this _fingerprint_ is likely a language keyword, boilerplate code, or something else shared amongst almost all files/repositories.
    - Otherwise, insert element into `fd` using the `value` (nested) map's data to give the _location_ and the `key` _fingerprint_ hash
5. Perform quadratic (pairwise) _document_ comparison
    - Initialize an empty output map of `(document, document)` tuple to vector of _fingerprints_, `out`
    - Iterate through keys in `fd` as _document_ `a`
    - Per document _a_, iterate through other keys in `fd` as _document_ `b`
    - Use `fi` to determine fingerprints existing in both `a` and `b`
    - Save tuple `(a,b)` and matching _fingerprint_ in map
6. Return rank-ordered map `out`, sorted by the length of the value (the number of matches for a given pair of _documents_)
7. Compute useful metrics from `out`


## Past work

### Winnowing paper
[Link to paper][winnowing-paper]  
> For this application, positional information (document and line number) is
> stored with each selected fingerprint. The first step builds an index mapping
> fingerprints to locations for all documents, much like the inverted index
> built by search engines mapping words to positions in documents. In the
> second step, each document is fingerprinted a second time and the selected
> fingerprints are looked up in the index; this gives the list of all matching
> fingerprints for each document.  Now the list of matching fingerprints for a
> document d may contain fingerprints from many different documents d1, d2,
> . . .. In the next step, the list of matching fingerprints for each document
> d is sorted by document and the matches for each pair of documents (d, d1
> ), (d, d2 ), . . . is formed. Matches between documents are rank-ordered by
> size (number of fingerprints) and the largest matches are reported to the
> user. Note that up until this last step, no explicit consideration of pairs
> of documents is required. This is very important, as we could not hope to
> carry out copy detection by comparing each pair of documents in a large
> corpus. By postponing the quadratic computation to the last step, we can
> optimize it by never materializing the matching for a pair of documents if it
> falls below some user-specified threshold.

> Presentation of the results is another important issue for users. Statistics
> such as reporting the percentage of overlap between two documents are useful,
> but not nearly as useful as actually showing the matches marked-up in the
> original text. MOSS uses the fingerprints to determine where the longest
> matching sequences are; in particular, if a1 in document 1 matches a2 in
> document 2, and b1 in document 1 matches b2 in document 2, and furthermore a1
> and b1 are consecutive in document 1 and a2 and b2 are consecutive in
> document 2, then we have discovered a longer match across documents
> consisting of a followed by b. While this merging of matches is easy to
> implement, k-grams are naturally coarse and some of the match is usually lost
> at the beginning and the end of the match. It is possible that once a pair of
> similar documents are detected using fingerprinting that it would be better
> to use a suffix-tree algorithm [\[15\]][suffix-tree] to find maximal matches
> in just that pair of documents.

### Others
#### Preprocessing
> "It does this by preprocessing the source code files, calculating a numeric
> fingerprint for each file, and then performing a longest common sequence
> search on the two fingerprints. The preprocessing stage replaces all function
> and variable names with a single token, and removes all comments and
> whitespace from the source code. The fingerprint stage calculates hash values
> for windows of characters in the resulting file, preserving the lowest hash
> values as the file’s fingerprint" [Engels et al. 2007][engels-paper]

### Notes
- check out https://blog.tonari.no/why-we-love-rust
- Btree Map for winnowing?

[moss]: https://theory.stanford.edu/~aiken/moss/
[engels-paper]: https://dl.acm.org/doi/pdf/10.1145/1227310.1227324
[winnowing-paper]: https://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf
[suffix-tree]: https://www.cs.helsinki.fi/u/ukkonen/SuffixT1withFigs.pdf
