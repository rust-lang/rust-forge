# Profiling Queries

In an effort to support incremental compilation, the latest design of
the Rust compiler consists of a _query-based_ model.

The details of this model are (currently) outside the scope of this
document, however, we explain some aspects of this model below, in an
effort to explain how we profile it. We intend this profiling effort
to address [issue
42678](https://github.com/rust-lang/rust/issues/42678).

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
  queries](#trace-of-queries), below.

- Query **Key**: The input/arguments to the provider.  Often, this
  consists of a particular [Def ID](#def-ids).

- Query **Result**: The output of the provider.

- Example queries:

    - `typeck_tables_of` -- Typecheck a Def ID; produce "tables" of type information. XXX
    - `borrowck` -- Borrow-check a Def ID; produce XXX
    - `optimized_mir` -- Generate an optimized MIR for a Def ID; produce MIR.

## Trace of Queries

**Cache hits and misses.**
The HTML output represents a trace of how the queries of the compiler
depend on one another.  Formally, a _trace_ of the queries consists of
a _tree_, with the following possible tree nodes:

- Query **hit**: The query's result is **known**, and is reused; its
  provider does not rerun.  These nodes are leaves in the trace, since
  they have no dynamic extent.  These leaves also represent where the
  tree, represented as a DAG, would _share_ a sub-graph (namely, the
  sub-graph of the query that was reused from the cache).

- Query **miss**: The query's result is **unknown**, and its provider
  runs to compute it.  In this case, the dynamic extent of the query's
  trace consists of the traced behavior of its provider.

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

## Interpret the HTML Output

The trace of the queries has a formal structure, which we reference
below.  See [Trace of Queries](#trace-of-queries), above, for more
details.

- Blue dots represent query hits.  They consist of leaves in the
  trace's tree. CSS class: `hit`.

- Red boxes represent query misses. They consist of internal nodes in
  the trace's tree. CSS class: `miss`.

## Heuristics

XXX

## Links

- https://internals.rust-lang.org/t/incremental-compilation-beta/4721
- https://blog.rust-lang.org/2016/09/08/incremental.html
