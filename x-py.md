---
layout: default
title: Building rustc with x.py; The Rust Forge
---

# What is `x.py`?

`x.py` is the script used to orchestrate the tooling in the `rustc` repository.
It is the script that can build docs, run tests, and compile `rustc`. It is the
now preferred way to build `rustc` and it replaces the old makefiles from
before. Below are the different ways to utilize `x.py` in order to effectively
deal with the repo for various common tasks.

# Table of Contents
- [Building `rustc`](#building-rustc)
  - [Build everything](#build-everything)
  - [Build different stages](#build-different-stages)
  - [Build specific components](#build-specific-components)
  - [Other Flags](#other-flags)
- [Testing rustc](#testing-rustc)
  - [Run all tests](#run-all-tests)
  - [Run specific tests](#run-specific-tests)
  - [Other Flags](#other-flags-2)
- [Benchmarking rustc](#benchmarking-rust)
  - [Other Flags](#other-flags-3)
- [Documenting rustc](#documenting-rustc)
  - [Document everything](#document-everything)
  - [Document specific components](#document-specific-components)
  - [Document internal rustc items](#document-internal-rustc-items)
  - [Other Flags](#other-flags-4)
- [Cleaning out build directories]()
  - [Other Flags](#other-flags-5)
- [Build distribution artifacts]()
  - [Other Flags](#other-flags-6)
- [Install distribution artifacts]()
  - [Other Flags](#other-flags-7)

# Building `rustc`
If you want to build the compiler to add new features you'll need to run one of
the build commands. Note that the first time you build the compiler it will take
a longer time to do so since you'll need to compile the entire LLVM library.
However, subsequent builds won't be as long unless LLVM is updated.

### Build everything

```bash
./x.py build
```

This command will build all stages of the compiler. First it will build a Stage
0 version of the compiler to make sure it can be bootstrapped from the previous
stable version of the compiler. It'll then build a Stage 1 compiler to see if it
can compile itself at that stage. At this point your changes to the compiler
should be visible if compilation doesn't fail. After this it builds a Stage
2 compiler as a sanity check.

More often than not this is not the command you want to run unless you plan on
double checking that everything is working.

### Build different stages

```bash
./x.py build --stage 0

# Stage 1 is enough to test out all of your changes
# to the compiler
./x.py build --stage 1

# Equivalent to ./x.py build
./x.py build --stage 2
```

You can pass the `--stage` flag with what stage you want to build to. It is
recommended that you build to Stage 1 as this is enough to know your changes can
successfully compile and should let you run tests with your changes.

### Build specific components

```bash
# Build only the libcore library
./x.py build src/libcore

# Build the libcore and libproc_macro library only
./x.py build src/libcore src/libproc_macro

# Build only libcore up to Stage 1
./x.py build src/libcore --stage 1
```

Sometimes you might just want to test if the part you're working on can compile.
Using these commands you can test that it compiles before doing a bigger build
to make sure it works with the compiler. As shown before you can also pass flags
at the end such as `--stage`

### Other Flags

There are other flags you can pass to the build portion of `x.py` that can be
beneficial to cutting down compile times or fitting other things you might need
to change. They are:

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
```

Note that the options `--incremental`, `--keep-stage 0` and `--jobs JOBS` can be
used in tandem with `--stage` to help reduce build times significantly by
reusing already built components, reusing the first bootstrapped stage, and
running compilation in parallel. To test changes you could run something like:

```bash
./x.py build --stage 1 --keep-stage 0 -j 4 -i
```


# Testing rustc

If you have successfully built rustc then you'll need to test that you didn't
break anything. It's recommended that you run all of the tests as well as the
tidy script before putting in a PR as this can help you catch errors in your
code as well as enforcing the style guidelines of the repo before submission.

### Run all tests

```bash
./x.py test
```

Much like `build` this will compile all of the tests and run them. This is good to
run right before submission.

### Run specific tests

```bash
# Run only the tidy script
./x.py test src/tools/tidy

# Run tests on the standard library
./x.py test src/libstd

# Run tests on the standard library and run the tidy script
./x.py test src/libstd src/tools/tidy

# Run tests on the standard library using a stage 1 compiler
./x.py test src/libstd --stage 1
```

By listing which test suites you want to run you avoid having to run tests for
components you did not change at all.

### Other Flags

Much like the `build` subcommand many of the same flags are still around as well
as a few new ones:

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
        --no-fail-fast  Run all tests regardless of failure
        --test-args ARGS
                        extra arguments
```

The last two flags `--no-fail-fast` and `--test-args` are the extra flags
available for `./x.py test` compared to `build`.

# Benchmarking rustc

This one is a easier compared to the others. All you're doing is
running benchmarks of the compiler itself so it'll build it and run the one set
of benchmarks available to it. The command is:

```bash
./x.py bench
```

### Other Flags

The `bench` option does have flags available to it and they're shown below.
They're the same almost as `test` and `build` but it lacks the `--no-fail-fast`
flag that `test` has.

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
        --test-args ARGS
                        extra arguments
```

# Documenting rustc

You might want to build documentation of the various components available like
the standard library. There's two ways to go about this. You can run `rustdoc`
directly on the file to make sure the HTML is correct which is fast or you can
build the documentation as part of the build process through `x.py`. Both are
viable methods since documentation is more about the content.

### Document everything

```bash
# Document it all
./x.py doc

# If you want to avoid the whole Stage 2 build
./x.py doc --stage 1
```

First the compiler and rustdoc get built to make sure everything is okay and
then it documents the files.

### Document specific components

```bash
./x.py doc src/doc/book
./x.py doc src/doc/nomicon
./x.py doc src/doc/book src/libstd
```

Much like individual tests or building certain components you can build only the
documentation you want.

### Document internal rustc items

By default `rustc` does not build the compiler for it's internal items.
Mostly because this is useless for the average user. However, you might need to
have it available so you can understand the types. Here's how you can compile it
yourself:

```bash
cp src/bootstrap/config.toml.example config.toml
```

Next open up `config.toml` and make sure these two lines are set to true:

```
docs = true
compiler-docs = true
```

When you want to build the compiler docs as well run this command:

```bash
./x.py doc --config config.toml
```

This will see that the `docs` and `compiler-docs` options are set to true and
build the normally hidden compiler docs!

### Other Flags

Like `build`, `doc` also has many optional flags available for it to use:

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
```

# Cleaning out build directories

Sometimes you need to start fresh. You only need to run one command!

```bash
./x.py clean
```

### Other Flags

The same flags from `build` are available as well but they're less effective
here:

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
```

# Build distribution artifacts

You might want to build and package up the compiler for distribution. You'll
want to run this command to do it:

```bash
./x.py dist
```

### Other Flags

The same flags from `build` are available. You might want to consider adding on
the `-j` flag for faster builds when building a distribution artifact.

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
```

# Install distribution artifacts
If you've built a distribution artifact you might want to install it and test
that it works on your target system. You'll want to run this command:

```bash
./x.py install
```

### Other Flags

The same flags from `build` are available as well but they're less effective
here:

```
Options:
    -v, --verbose       use verbose output (-vv for very verbose)
    -i, --incremental   use incremental compilation
        --config FILE   TOML configuration file for build
        --build BUILD   build target of the stage0 compiler
        --host HOST     host targets to build
        --target TARGET target targets to build
        --on-fail CMD   command to run on failure
        --stage N       stage to build
        --keep-stage N  stage to keep without recompiling
        --src DIR       path to the root of the rust checkout
    -j, --jobs JOBS     number of jobs to run in parallel
    -h, --help          print this help message
```
