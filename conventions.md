---
layout: default
title: The Rust Forge &middot; Conventions
---

# Conventions

The Rust Programming Language has many conventions when it comes to library
design. This document intends to document all of them.

## Naming

### Lints

* Lint names should state the bad thing being checked for,
  e.g. `deprecated`, so that `#[allow(deprecated)]` (items) reads
  correctly. Thus `ctypes` is not an appropriate name; `improper_ctypes` is.

* Lints that apply to arbitrary items (like the stability lints) should just
  mention what they check for: use `deprecated` rather than `deprecated_items`.
  This keeps lint names short. (Again, think "allow *lint-name* items".)

* If a lint applies to a specific grammatical class, mention that class and use
  the plural form: use `unused_variables` rather than `unused_variable`.
  This makes `#[allow(unused_variables)]` read correctly.

* Lints that catch unnecessary, unused, or useless aspects of code
  should use the term `unused`, e.g. `unused_imports`, `unused_typecasts`.

* Use snake case in the same way you would for function names.

### Traits

The wiki guidelines have long suggested naming traits as follows:

* Prefer (transitive) verbs, nouns, and then adjectives. Avoid grammatical
  suffixes such as `able`. Examples include `Copy`, `Clone`, `Show`.

* If there is a single method that is the dominant functionality of the trait,
  consider using the same name for the trait itself. Examples include `Clone`
  and `ToCStr`.

### Associated Types

* Associated types should be given concise, but meaningful names, generally
  following the convention for type names rather than generic. For example, use
  `Err` rather than `E`, and `Item` rather than `T`.

## Error Handling

Rust provides two basic strategies for dealing with errors:

* *Panicking*, which unwinds to at least the task boundary, and by default
  propagates to other tasks through poisoned channels and mutexes. Panicking
  works well for coarse-grained error handling.

* *The Result type*, which allows functions to signal error conditions through
  the value that they return. Together with a lint and the `try!` macro,
  `Result` works well for fine-grained error handling.

Use these basic strategies to handle specific error classes.

* For *catastrophic errors*, abort the process or panic the task depending on
  whether any recovery is possible.

* For *contract violations*, panic the task. Recover from programmer errors at
  a coarse grain.

* For *obstructions to the operation*, use `Result` or, less often,
  `Option`. Recover from obstructions at a fine grain.

* Prefer liberal function contracts, especially if reporting errors in input
  values may be useful to a function's caller.

### Detailed Explanation

Errors fall into one of three categories:

* Catastrophic errors, e.g. out-of-memory.
* Contract violations, e.g. wrong input encoding, index out of bounds.
* Obstructions, e.g. file not found, parse error.

The basic principle of the conventions is that:

* Catastrophic errors and programming errors (bugs) can and should only be
  recovered at a *coarse grain*, i.e. a task boundary.
* Obstructions preventing an operation should be reported at a maximally *fine
  grain* -- to the immediate invoker of the operation.

#### Catastrophic errors

An error is _catastrophic_ if there is no meaningful way for the current task to
continue after the error occurs.

Catastrophic errors are _extremely_ rare, especially outside of `libstd`.

**Canonical examples**: out of memory, stack overflow.

##### For catastrophic errors, panic.

For errors like stack overflow, Rust currently aborts the process, but
could in principle panic, which, in the best case, would allow
reporting and recovery from a supervisory task.

#### Contract violations

An API may define a contract that goes beyond the type checking enforced by the
compiler. For example, slices support an indexing operation, with the contract
that the supplied index must be in bounds.

Contracts can be complex and involve more than a single function invocation. For
example, the `RefCell` type requires that `borrow_mut` not be called until all
existing borrows have been relinquished.

#### For contract violations, panic.

A contract violation is always a bug, and for bugs we follow the Erlang
philosophy of "let it crash": we assume that software *will* have bugs, and we
design coarse-grained task boundaries to report, and perhaps recover, from
these bugs.

##### Contract design

One subtle aspect of these guidelines is that the contract for a function is
chosen by an API designer — and so the designer also determines what counts as
a violation.

Without attempting to give hard-and-fast rules for designing
contracts, here are some rough guidelines:

* Prefer expressing contracts through static types whenever possible.

* It *must* be possible to write code that uses the API without violating the
  contract.

* Contracts are most justified when violations are *inarguably* bugs — but this
  is surprisingly rare.

* Consider whether the API client could benefit from the contract-checking
  logic. The checks may be expensive. Or there may be useful programming
  patterns where the client does not want to check inputs before hand, but would
  rather attempt the operation and then find out whether the inputs were invalid.

* When a contract violation is the *only* kind of error a function may encounter
  — i.e., there are no obstructions to its success other than "bad" inputs —
  using `Result` or `Option` instead is especially warranted. Clients can then use
  `unwrap` to assert that they have passed valid input, or re-use the error
  checking done by the API for their own purposes.

* When in doubt, use loose contracts and instead return a `Result` or `Option`.

#### Obstructions

An operation is *obstructed* if it cannot be completed for some reason, even
though the operation's contract has been satisfied. Obstructed operations may
have (documented!) side effects -- they are not required to roll back after
encountering an obstruction.  However, they should leave the data structures in
a "coherent" state (satisfying their invariants, continuing to guarantee safety,
etc.).

Obstructions may involve external conditions (e.g., I/O), or they may involve
aspects of the input that are not covered by the contract.

**Canonical examples**: file not found, parse error.

##### For obstructions, use `Result`

The
[`Result<T,E>` type](http://static.rust-lang.org/doc/master/std/result/index.html)
represents either a success (yielding `T`) or failure (yielding `E`). By
returning a `Result`, a function allows its clients to discover and react to
obstructions in a fine-grained way.

###### What about `Option`?

The `Option` type should not be used for "obstructed" operations; it
should only be used when a `None` return value could be considered a
"successful" execution of the operation.

This is of course a somewhat subjective question, but a good litmus
test is: would a reasonable client ever ignore the result? The
`Result` type provides a lint that ensures the result is actually
inspected, while `Option` does not, and this difference of behavior
can help when deciding between the two types.

Another litmus test: can the operation be understood as asking a
question (possibly with sideeffects)? Operations like `pop` on a
vector can be viewed as asking for the contents of the first element,
with the side effect of removing it if it exists -- with an `Option`
return value.

#### Do not provide both `Result` and `panic!` variants.

An API should not provide both `Result`-producing and `fail`ing versions of an
operation. It should provide just the `Result` version, allowing clients to use
`try!` or `unwrap` instead as needed. This is part of the general pattern of
cutting down on redundant variants by instead using method chaining.

There is one exception to this rule, however. Some APIs are strongly oriented
around failure, in the sense that their functions/methods are explicitly
intended as assertions.  If there is no other way to check in advance for the
validity of invoking an operation `foo`, however, the API may provide a
`try_foo` variant that returns a `Result`.

The main examples in `libstd` that *currently* provide both variants are:

* Channels, which are the primary point of failure propagation between tasks. As
  such, calling `recv()` is an _assertion_ that the other end of the channel is
  still alive, which will propagate failures from the other end of the
  channel. On the other hand, since there is no separate way to atomically test
  whether the other end has hung up, channels provide a
  [`try_recv`](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.try_recv)
  variant that produces a `Result`.

* `RefCell`, which provides a dynamic version of the borrowing rules. Calling
  the `borrow()` method is intended as an assertion that the cell is in a
  borrowable state, and will `fail!` otherwise. On the other hand, there is no
  separate way to check the state of the `RefCell`, so the module provides a
  `try_borrow` variant that produces a `Result`.

## Unsafe Code

* When an unsafe function/method is clearly "about" a certain type (as a way of
  constructing, destructuring, or modifying values of that type), it should be a
  method or static function on that type. This is the same as the convention for
  placement of safe functions/methods.

* `raw` submodules should only be used to *define* low-level
  types/representations (and methods/functions on them). Methods for converting
  to/from such low-level types should be available directly on the high-level
  types.

### Examples

Many data structures provide unsafe APIs either for avoiding checks or working
directly with their (otherwise private) representation. For example, `string`
provides:

* An `as_mut_vec` method on `String` that provides a `Vec<u8>` view of the
  string.  This method makes it easy to work with the byte-based representation
  of the string, but thereby also allows violation of the utf8 guarantee.

* A `raw` submodule with a number of free functions, like `from_parts`, that
  constructs a `String` instances from a raw-pointer-based representation, a
  `from_utf8` variant that does not actually check for utf8 validity, and so
  on. The unifying theme is that all of these functions avoid checking some key
  invariant.

Examples of raw submodules are `core::raw` and `sync::raw`.