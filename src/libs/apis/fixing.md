# Fixing APIs

So, we actually lied: some breaking changes can actually be made in the standard library, assuming we do so very carefully. This page lists the shenanigans the library team has performed to get around stability teams, to hopefully be expanded infrequently.

## Deprecation

Sometimes, an API is just bad, and we want to forget it happened. For example, [`std::fs::soft_link`] was deprecated in Rust 1.1.0 and replaced by OS-specific functions, since Windows needs to distinguish between directory and file links.

[`std::fs::soft_link`]: https://doc.rust-lang.org/std/fs/fn.soft_link.html

Other times, we make a feature better and want to forget when it wasn't. For example, the [`try!`] macro was deprecated in Rust 1.39.0 and replaced by the dedicated `?` operator, which allows types like [`Option`] in addition to [`Result`].

[`try!`]: https://doc.rust-lang.org/std/macro.try.html
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html

## Deprecated safety

Sometimes, making a function safe was a mistake, and we want to mark it as unsafe after the fact. For example, [`std::env::set_var`] is genuinely unsafe, potentially beyond the point of usability on some platforms. And yet, before Rust 2024, this method was completely safe to call, and its unsafety was only added on an edition boundary.

[`std::env::set_var`]: https://doc.rust-lang.org/std/env/fn.set_var.html

## Edition-dependent prelude

Some traits are so useful to Rust that we add them to the prelude, including them in all Rust code by default. However, [this can sometimes break code](./stabilization.md#method-resolution), and thus be a breaking change.

To get around this, the standard library prelude depends on the current edition of Rust, and traits can be added or removed at an edition boundary:

* [Rust 2021] added [`TryFrom`], [`TryInto`], and [`FromIterator`] to the prelude.
* [Rust 2024] added [`Future`] and [`IntoFuture`] to the prelude.

[Rust 2021]: https://doc.rust-lang.org/nightly/edition-guide/rust-2021/prelude.html
[Rust 2024]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/prelude.html
[`TryFrom`]: https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/nightly/std/convert/trait.TryInto.html
[`FromIterator`]: https://doc.rust-lang.org/nightly/std/iter/trait.FromIterator.html
[`Future`]: https://doc.rust-lang.org/nightly/std/future/trait.Future.html
[`IntoFuture`]: https://doc.rust-lang.org/nightly/std/future/trait.IntoFuture.html

## Edition-dependent resolution

Sometimes, adding a new trait implementation or method breaks existing code in ways we really can't ignore, and we want to add them anyway. In these cases, we explicitly modify the compiler to avoid a method before an edition boundary:

* [Before Rust 2021], [`IntoIterator`] for arrays is hidden.
* [Before Rust 2024], [`IntoIterator`] for boxed slices is hidden.

[`IntoIterator`]: https://doc.rust-lang.org/nightly/std/iter/trait.IntoIterator.html
[Before Rust 2021]: https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html
[Before Rust 2024]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/intoiterator-box-slice.html

## Edition redirects

In the future, the standard library will be able to "redirect" paths to different places based upon the edition. This unlocks many new possibilities.

However, it is not yet implemented: [#160227]

[#160227]: https://github.com/rust-lang/rust/pull/160227
