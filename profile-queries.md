# Profiling Queries

In an effort to support _incremental compilation_, the latest design of
the Rust compiler consists of a _query-based_ model.

The details of this model are (currently) outside the scope of this
document, however, we explain
[some background of this model](#background), in an effort to explain
how we profile it. We intend this profiling effort to address
[issue 42678](https://github.com/rust-lang/rust/issues/42678).

## Quick Start

1. Compile the compiler, as usual

```
python x.py --stage 1
```

2. Run the compiler on a source file:

```
rustc -Z profile-queries -Z dump-dep-graph foo.rs
```

Notice the two additional parameters:

- `-Z profile-queries` tells the compiler to run a separate thread
  that profiles the queries made by the main compiler thread(s).
- `-Z dump-dep-graph` tells the compiler to "dump" various files that
  describe the compilation dependencies.

This command will generate the following files:

- `profile_queries.html` consists of an HTML-based representation of
  the [trace of queries](#trace-of-queries).
- `profile_queries.counts.txt` consists of a histogram, where each histogram "bucket" is a query provider.

Older stuff, also generated as output:

- `dep_graph.dot` consists of old stuff: a representation of dependencies that are _outside_ the newer query model.
- `dep_graph.txt` consists of old stuff: a representation of dependencies that are _outside_ the newer query model.


## Interpret the HTML Output

### Example 0

The following image gives some example output, from tracing the
queries of `hello_world.rs` (a single `main` function, that prints
`"hello world"` via the macro `println!`).  This image only shows a
short prefix of the total output; the _actual_ output is much longer.

![](profile-queries/example0.png)


### Example 0 explanation

The trace of the queries has a formal structure; see
[Trace of Queries](#trace-of-queries), for more details.

- Blue dots represent query hits.  They consist of leaves in the
  trace's tree. CSS class: `hit`.
- Red boxes represent query misses. They consist of internal nodes in
  the trace's tree. CSS class: `miss`.
- Some red boxes are _nested within others_.  This nesting structure
  reflects that some providers _depend on_ results from other
  providers, which consist of their nested children.
	  - For example, the red box labeled as `typeck_tables_of` depends
		on the one labeled `adt_dtorck_constraint`, which itself
		depends on one labeled `coherent_trait`.
- Some red boxes are _labeled_ with text.  (See
  [heuristics](#heuristics) for details).  Where they are present, the
  labels give the following information:
     - The [query's _provider_](#queries), sans its _key_ and its _result_,
       which are often too long to include in these labels.
     - The _duration_ of the provider, in seconds. This time includes
       the query's entire extent (that is, the sum total of all of its
       sub-queries).


## Heuristics

XXX


# Background

We give some background about the query model of the Rust compiler.

## Def IDs

In the query model, many queries have a key that consists of a Def ID.
The Rust compiler uses Def IDs to distinguish definitions in the input
Rust program.

From the compiler source code (`src/librustc/hir/def_id.rs`):

```
/// A DefId identifies a particular *definition*, by combining a crate
/// index and a def index.
#[derive(Clone, Eq, Ord, PartialOrd, PartialEq, RustcEncodable, RustcDecodable, Hash, Copy)]
pub struct DefId {
    pub krate: CrateNum,
    pub index: DefIndex,
}
```

## Queries

A query relates a _key_ to a _result_, either by invoking a _provider_
that computes this result, or by reusing a cached result that was
provided earlier.  We explain each term in more detail:

- Query **Provider**: Each kind of query has a pre-defined _provider_,
  which refers to the compiler behavior that provides an answer to the
  query.  These providers may nest; see [trace of
  queries](#trace-of-queries).
- Query **Key**: The input/arguments to the provider.  Often, this
  consists of a particular [Def ID](#def-ids).
- Query **Result**: The output of the provider.
- Example queries:

    - `typeck_tables_of` -- Typecheck a Def ID; produce "tables" of type information.
    - `borrowck` -- Borrow-check a Def ID.
    - `optimized_mir` -- Generate an optimized MIR for a Def ID; produce MIR.

## Trace of Queries

Formally, a _trace_ of the queries consists of a _tree_, where
sub-trees represent sub-traces.  In particular, the nesting structure
of the trace of queries describes how the queries depend on one
another.

Even more precisely, this tree represents a directed acyclic graph
(DAG), where shared sub-graphs consist of tree nodes that occur
multiple times in the tree, first as "cache misses" and later as
"cache hits".

**Cache hits and misses.**
The trace is a tree with the following possible tree nodes:

- Query, with cache **miss**: The query's result is **unknown**, and
  its provider runs to compute it.  In this case, the dynamic extent
  of the query's trace consists of the traced behavior of its
  provider.
- Query, with cache **hit**: The query's result is **known**, and is
  reused; its provider does not rerun.  These nodes are leaves in the
  trace, since they have no dynamic extent.  These leaves also
  represent where the tree, represented as a DAG, would _share_ a
  sub-graph (namely, the sub-graph of the query that was reused from
  the cache).

**Tree node metrics.**
To help determine how to style this tree, we define the following tree
node metrics:

- **Depth**: The number of **ancestors** of the node in its path from the tree root.
- **Extent**: The number of **immediate children** of the node.

Intuitively, a dependency tree is "good" for incremental caching when
the depth and extent of each node is relatively small.  It is
pathological when either of these metrics grows too large.  For
instance, a tree node whose extent consists of 1M immediate children
means that if and when this node is re-computed, all 1M children must
be re-queried, at the very least (some may also require recomputation,
too).

## Links

- https://internals.rust-lang.org/t/incremental-compilation-beta/4721
- https://blog.rust-lang.org/2016/09/08/incremental.html
