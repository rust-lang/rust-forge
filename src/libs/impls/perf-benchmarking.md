# Performance and benchmarking

The performance of the standard library is a topic of frequent discussion, and people frequently propose changes that allege better performance or code generation. How do we distinguish an actually good change from a bad one?

## Codegen tests

For small functions, [codegen tests] are the most reliable way to ensure that a function is compiled in the most optimal way. For example, the [`checked-ilog`] test ensures that the `checked_ilog` function can be compiled such that it doesn't actually perform any division or multiplication instructions, which are very expensive to perform.

[codegen tests]: https://rustc-dev-guide.rust-lang.org/tests/compiletest.html#codegen-tests
[`checked-ilog`]: https://github.com/rust-lang/rust/blob/f45772eb69d6ed3cc23be40625411a75f9f32c9d/tests/codegen-llvm/checked_ilog.rs

In general, codegen tests should match against the generated LLVM IR to ensure that compilation is optimal for all targets, although target-specific optimizations can be checked against the generated assembly instructions instead.

Additionally, it's worth mentioning that the presence of a codegen test with the change doesn't actually mean that the change improved the code generation, just that the code is generating correctly *with* the change. In some cases, it's best to verify that the codegen test actually fails without a change before accepting it, since the test may be useful to include even if the associated change isn't.

## Benchmarking the compiler

If a part of the standard library is used heavily by the compiler itself, the [`@rust-timer`] tool can be used to run compiler benchmarks and check the effect of changes.

[`@rust-timer`]: https://rustc-dev-guide.rust-lang.org/tests/perf.html#manual-perf-runs

Note that this only checks the *compile time* of various crates and not their actual runtime, and thus it won't report any changes if code isn't used by the compiler, like floating-point arithmetic, linked lists, MPSC channels, etc.

Additionally, note that even the compiler's benchmarking tool can be unreliable in some cases, although it is definitely much more reliable than the alternatives.

## Manual benchmarks

In some cases, benchmarks are added directly to the standard library and can be run directly for changes. Additionally, the compiler can be built with standard library changes and then used to benchmark popular crates which have their own benchmarks.

Unfortunately, these benchmarks can be very limited in their usefulness, especially on systems with a lot of noise. Here are a few ways you can help:

* Setting `rust.incremental = false` in [`bootstrap.toml`].
* Ensure the system is as idle as possible.
* Disable [address space layout randomization].
* [Pin the benchmark process] to a single core.
* Change the [CPU scaling governor] to a fixed frequency.
* Disable [CPU clock boosts].

[`bootstrap.toml`]: https://github.com/rust-lang/rust/blob/main/bootstrap.example.toml
[address space layout randomization]: https://man7.org/linux/man-pages/man8/setarch.8.html
[Pin the benchmark process]: https://man7.org/linux/man-pages/man8/taskset.8.html
[CPU scaling governor]: https://wiki.archlinux.org/title/CPU_frequency_scaling#Scaling_governors
[CPU clock boosts]: https://wiki.archlinux.org/title/CPU_frequency_scaling#Configuring_frequency_boosting

However, in general, performance checks via manual benchmarking are discouraged unless changes are extremely noticeable, since they can be extremely unreliable. Performance can be *extremely* hard to determine, specific to hardware, misleading, and generally unintuitive.

## `#[inline]`

One of the most powerful and most misleading tools for improving performance is `#[inline]`, since Rust relies heavily on multiple ways to inline function calls. That said, the circumstances in which inlining are helpful are very unintuitive and can effect unrelated things in unintended ways, and in general, the compiler probably knows better than you do in terms of when inlining is helpful.

To start, it's important to remember that in general, inlining is already considered for all functions *inside the same crate*. This means that private functions are effectively already marked with `#[inline]` and adding the attribute likely won't do anything. Additionally, this guarantee is extended to functions that are generic, including default trait methods, and thus adding the attribute won't do anything there either.

All `#[inline]` does is give a nudge to the compiler to inline more often, so, generally the worst effect it can have is longer compile times, and usually it will just have no effect. In general, this means that substantial effort doesn't need to go into ensuring that `#[inline]` won't have a *negative* effect, but that at least some effort should be put into ensuring that its effect is *positive*.

## `#[inline]`'s evil twins

There are other methods of inlining that can be used, but should always be associated with strong motivation and, ideally, real data supporting that motivation.

### Manual inlining

The first one is a bit weird, but you can technically *manually* inline things. For example, instead of using [`Option::map`], it can sometimes be beneficial to just use a `match` statement and effectively "inline" the `map` function directly into a method. This is because every function call layer makes the compiler that much less inclined to inline things, and closures in particular can generate a lot of code that then has to be optimized out. Simply removing the closure entirely can, in some cases, make the difference for optimizations.

[`Option::map`]: https://doc.rust-lang.org/nightly/std/option/enum.Option.html#method.map

### `#[inline(always)]`

`#[inline(always)]` is the much-stronger version of `#[inline]` that effectively tells the compiler to *always* inline the function, and it can have very unintuitive effects. These changes should almost always be associated with real data, and particularly data which shows a *strong* improvement with the attribute, since it can be unreliable.

In general, the cases where this applies most are debug builds, which usually don't inline anything. However, note that `#[inline(always)]` doesn't mean `#[inline(but apply in the debug profile too)]`, it means `#[inline(always)]`. These annotations are also particularly helpful for overcoming the compiler's aversion to inlining deeply nested call stacks, but usually it's better to avoid having such deeply nested calls in the first place, like via manual inlining.

### `#[inline(never)]`

Similar to `#[inline(always)]`, there is also `#[inline(never)]`, which always prevents inlining. For very similar reasons, this should be avoided unless you absolutely know what you're doing, and have data to prove that.

The most common use case for `#[inline(never)]` is on cold functions, however, there is another attribute you can use for those: `#[cold]`, to indicate they're rarely run. The two can be combined, but in general, the addition of `#[inline(never)]` is not necessary and should be avoided unless accompanied by motivated reasoning.

## `assert_unchecked`

One of the biggest hammers the standard library offers is [`std::hint::assert_unchecked`], which informs the compiler that a logical statement is always true. In pretty much all cases, this hammer should be avoided over other types of unsafe code, since these types of conditions are very difficult for the optimizer to use appropriately, and sometimes result in fewer optimizations being run.

[`std::hint::assert_unchecked`]: https://doc.rust-lang.org/nightly/std/hint/fn.assert_unchecked.html

Not only should uses of this function be associated with motivated reasoning and data, but you should attempt to exhaust all other avenues of achieving the same effect *before* relying on this function, since generally other forms of reasoning are more useful to the optimizer.

### Pattern types

[Pattern types] currently exist as a compiler-internal way to convey range information to the optimizer. This allows marking specific values as being contained in a particular range, and is particularly used for the built-in [`NonNull`] and [`NonZero`] types.

[Pattern types]: https://github.com/rust-lang/rust/issues/123646
[`NonNull`]: https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html
[`NonZero`]: https://doc.rust-lang.org/nightly/std/num/struct.NonZero.html

While pattern types are unstable, they are okay to use in the standard library to achieve range information and better optimization in some cases. For example, the internal-only [`UsizeNoHighBit`] type is used for [`RawVec`] to enforce the invariant that memory allocations [cannot be larger than `isize::MAX`][`isize::MAX`].

[`UsizeNoHighBit`]: https://github.com/rust-lang/rust/blob/f45772eb69d6ed3cc23be40625411a75f9f32c9d/library/core/src/num/niche_types.rs#L129
[`RawVec`]: https://github.com/rust-lang/rust/blob/f45772eb69d6ed3cc23be40625411a75f9f32c9d/library/alloc/src/raw_vec/mod.rs#L40
[`isize::MAX`]: https://doc.rust-lang.org/nightly/std/ptr/index.html#allocation

### Unsafe arithmetic

Unsafe arithmetic functions are also a valid avenue for optimization, since they tell the compiler that certain cases cannot occur. For example, [`unchecked_sub`] is used for unsafe indexing operations ssince they can implicitly assume that the end of a range is greater than the start, and thus won't overflow.

[`unchecked_sub`]: https://github.com/rust-lang/rust/blob/f45772eb69d6ed3cc23be40625411a75f9f32c9d/library/core/src/str/traits.rs#L221

## Placeholder

* Removing redundant UB checks
* Funrolling loops
* Autovectorization
