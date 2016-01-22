---
layout: default
title: The Rust Test Suite &middot; The Rust Programming Language
---

# The Rust test suite

The rust test suite has several sets of tests for different purposes. As the compiler is built over multiple stages, and with varying host and target combinations, debugging and profiling settings, the tests can be run in many different ways.

## Recipes

* Run the test suite: `make check`. This runs all stage2 tests. This is the criterion for a successful build.
* Run the test suite with benchmarks: `make check PLEASE_BENCH=1`
* Run only ignored (broken) tests: `make check CHECK_IGNORED=1`
* Run with verbose output: `make check VERBOSE=1`. This will print useful information like the exact commands being run.
* Run with valgrind: `make check CFG_ENABLE_VALGRIND=1`
* Run a specific test: `make check TESTNAME=[...]`
  * The pattern `[...]` can be a complete path to a test, such as
    `run-pass/foobar.rs`; it can also be any substring of a path.
    For instance, `make check TESTNAME=foo` will run all tests that
    have `foo` in some part of their filename.
  * Note that while this will run only tests matching the given
    pattern, it will still execute all test runners - most of them
    will just not execute any tests. For more precise control, call
    `make` on one of the targets below.
  * You may need to `touch` the test source file to ensure that `make`
    runs the necessary test runners.
* Run without parallelism: `make check RUST_TEST_TASKS=1` - This can make it easier to understand failure output.
* Build and test std without re-bootstrapping: `make check-stage1-std NO_REBUILD=1` - This makes the build/test cycle **much** faster. (Note: `NO_REBUILD` can also prevent compile tests from being rebuilt. If you want to rebuild those but not the compiler, look for files with the `.ok` extension in the `tmp` subdirectory and remove the appropriate ones.)

These options can be combined.  For instance, `make check CHECK_IGNORED=1 TESTNAME=test/run-pass/foobar.rs` runs the ignored test `foobar.rs` in the `run-pass` directory.

## Language and compiler tests

These are tests of the compiler against Rust source code. They typically have a `main` function that takes no arguments and may have directives that instruct the test runner how to run the test. These tests may be compiled and executed, pretty-printed, jitted, etc. depending on the test configuration.

The test runner for these tests is at src/test/compiletest and is compiled to test/compiletest.stage[N].

A typical test might look like:

```rust
// ignore-pretty 'bool' doesn't pretty-print (#XXX)
// Regression test for issue #YYY

fn main() {
   let a: bool = 10; //~ ERROR mismatched types
}
```

There are seven different modes for compile tests. Each test is run under one or more modes:

* compile-fail - The test should fail to compile. Must include at least one expected error.
* run-fail - The test should compile but fail at runtime. Must include at least one error-pattern directive.
* run-pass - The test should compile and run successfully
* run-pass-valgrind - The test should compile and run (under Valgrind, where possible) successfully
* pretty - The test should round-trip through the pretty-printer and then compile successfully
* debug_info - The test will be compiled with debuginfo and an embedded `gdb` command script will be run to query the debuginfo
* codegen - The test will be compiled to LLVM bitcode, and a companion test written in C++ will be compiled to LLVM bitcode with `clang`, and the number of instructions will be compared

Valid directives include:

* `compile-flags:[...]` - Additional arguments to pass to the compiler
* `pp-exact` - The test should pretty-print exactly as written
* `pp-exact:[filename]` - The pretty-printed test should match the example in `filename`
* `ignore-test` - Test is broken, don't run it
* `ignore-pretty` - Test doesn't pretty-print correctly
* `error-pattern:[...]` - A message that should be expected on standard out. If multiple patterns are provided then they must all be matched, in order (**Note:** error-patterns are somewhat deprecated, see the section on Expected Errors below).
* `no-reformat` - Don't reformat the code when running the test through the pretty-printer
* `aux-build:foo.rs` - Compiles an auxiliary library for use in multi-crate tests.  See the section on multi-crate testing below.
* `exec-env:NAME` or `exec-env:NAME=VALUE` - Sets an environment variable during test execution
* `debugger:CMD` - Issues a command to the debugger in `debuginfo` mode
* `check:RESULT` - Checks the result of a previous `debugger:` directive in `debuginfo` mode

There are eight directories containing compile tests, living in the src/tests directory:

* run-pass - Tests that are expected to compile and run successfully. Also used for pretty-print testing.
* run-pass-valgrind - Tests that are expected to compile and run successfully under Valgrind. Also used for pretty-print testing.
* run-fail - Tests that are expected compile but fail when run. Also used for pretty-print testing.
* compile-fail - Tests that are expected not to compile
* bench - Benchmarks and miscellaneous snippets of code that are expected to compile and run successfully. Also used for pretty-print testing.
* pretty - Pretty-print tests
* debug-info - Debuginfo tests
* codegen - Codegen tests
* auxiliary - libraries used in multi-crate testing. See the section on this topic below.

And finally, build targets:

* `check-stage[N]-rpass` - The tests in the run-pass directory, in run-pass mode
* `check-stage[N]-rpass-valgrind` - The tests in the run-pass-valgrind directory, in run-pass-valgrind mode
* `check-stage[N]-rfail` - The tests in the run-fail-directory, in run-fail mode
* `check-stage[N]-cfail` - The tests in the compile-fail directory, in compile-fail mode
* `check-stage[N]-bench` - The tests in the bench directory, in run-pass mode
* `check-stage[N]-pretty` - All the pretty-print tests
* `check-stage[N]-pretty-rpass` - The tests in the run-pass directory, in pretty mode
* `check-stage[N]-pretty-rpass-valgrind` - The tests in the run-pass-valgrind directory, in pretty mode
* `check-stage[N]-pretty-rfail` - The tests in the run-fail directory, in pretty mode
* `check-stage[N]-pretty-bench` - The tests in the bench directory, in pretty mode
* `check-stage[N]-pretty-pretty` - The tests in the pretty directory, in pretty mode
* `check-stage[N]-debuginfo` - The tests in the debuginfo directory
* `check-stage[N]-codegen` - The tests in the codegen directory

### Specifying the expected errors and warnings

When writing a compile-fail test, you must specify at least one
expected error or warning message.  The preferred way to do this is to
place a comment with the form `//~ ERROR msg` or `//~ WARNING msg` on
the line where the error or warning is expected to occur.  You may
have as many of these comments as you like.  The test harness will
verify that the compiler reports precisely the errors/warnings that are
specified, no more and no less.  An example of using the error/warning
messages is:
```rust
// Regression test for issue #XXX

fn main() {
   let a: bool = 10; //~ ERROR mismatched types
   log (debug, b);
}
```
In fact, this test would fail, because there are two errors: the type
mismatch and the undefined variable `b`.  

Sometimes it is not possible or not convenient to place the `//~`
comment on precisely the line where the error occurs. For those cases,
you may make a comment of the form `//~^` where the caret `^`
indicates that the error is expected to appear on the line above.  You
may have as many caret as you like, so `//~^^^ ERROR foo` indicates
that the error message `foo` is expected to be reported 3 lines above
the comment.  We could therefore correct the above test like so:
```rust
// Regression test for issue #XXX

fn main() {
   let a: bool = 10; //~ ERROR mismatched types
   log (debug, b);
   //~^ ERROR undefined variable `b`
}
```

The older technique for specifying error messages was to use an
`error-pattern` directive.  These directives are placed at the top of
the file and each message found in an `error-pattern` directive must
appear in the output. 

Using error comments is preferred, however, because it is a more
thorough test:

- It verifies that the error is reported on the expected line number.
- It verifies that no additional errors or warnings are reported.

### Multi-crate testing

Sometimes it is useful to write tests that make use of more than one crate.  We have limited support for this scenario.  Basically, you can write and add modules into the `src/test/auxiliary` directory. These files are not built nor tested directly.  Instead, you write a main test in one of the other directories (`run-pass`, `compile-fail`, etc) and add a `aux-build` directive at the head of the main test.  When running the main test, the test framework will build the files it is directed to build from the auxiliary directory.  These builds *must* succeed or the test will fail.  You can then include `use` and `import` commands to make use of the byproducts from these builds as you wish.  

An example consisting of two files:
```rust
auxiliary/cci_iter_lib.rs:
  #[inline]
  fn iter<T>(v: [T], f: fn(T)) {...}

run-pass/cci_iter_exe.rs:
  // aux-build:cci_iter_lib.rs
  extern crate cci_iter_lib;
  fn main() {
    cci_iter_lib::iter([1, 2, 3]) {|i| ... }
  }
```

### Recipes

* Running the run-pass tests for stage1: `make check-stage1-rpass`
* Running a specific compile-fail test: `make check-stage2-cfail TESTNAME=type-mismatch`
* Finding the command to compile a failing test: `make check-stage1-rpass TESTNAME=hello VERBOSE=1`
* Running ignored tests: `make check-stage1-rpass CHECK_IGNORED=1`

## Unit Tests

Most crates include <a href="https://github.com/mozilla/rust/wiki/Doc-unit-testing">unit tests</a> which are part of the crate they test. These crates are built with the `--test` flag and run as part of `make check`.

`libcore` has its tests in a separate crate, named `libcoretest`.

All tests in a module should go in an inner module named `test`, with the attribute `#[cfg(test)]`. Placing tests in their own module is a practical issue - because test cases are not included in normal builds, building with `--test` require a different set of imports than without, and that causes 'unused import' errors.

```rust
use std::option;

fn do_something() { ... }

#[cfg(test)]
mod test {
   use std::vec;

   fn helper_fn() { ... }

   #[test]
   fn resolve_fn_types() {
     ...
   }
}
```

### Build targets

* `make check-stage[N]-crates`
* `make check-stage[N]-[crate name]test`

## Documentation tests

The build system is able to extract Rust code snippets from documentation and run them using the compiletest driver. Currently the tutorial and reference manual are tested this way. The targets are `make check-stage[N]-doc-tutorial` and `make check-stage[N]-doc-rust`, respectively. There are also several auxiliary guides; to run the tests extracted from them, do:

* `make check-stage[N]-doc-guide-borrowed-ptr`
* `make check-stage[N]-doc-guide-tasks`
* `make check-stage[N]-doc-guide-ffi`
* `make check-stage[N]-doc-guide-macros`
* `make check-stage[N]-doc-guide-testing`

Crate API docs are tested as well:

* `make check-stage[N]-doc-crate-std`

To run all doc tests use `make check-stage[N]-doc`.

## Minimal (but faster) checking on windows

Because Windows has slow process spawning running `make check` on that platform can take a long time. For this reason we have a `make check-lite` target that the Windows build servers run to keep the cycle time down. This is a stripped-down target which only checks run-pass, run-fail, compile-fail, run-make and target libraries.

## Benchmarks, saved metrics and ratchets

All benchmark metrics are saved by default. Depending on configuration, some benchmark metrics are ratcheted. The `codegen` compile tests are always ratcheted if they run, since they are deterministic / low-noise. See [[Doc unit testing]] for details on metrics and ratchets.
