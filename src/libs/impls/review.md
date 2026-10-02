# Code review

When reviewing changes to standard library code, there are a lot of sneaky details that can escape even the keenest reviewer's eyes. This attempts to cover several of the various details that the library team thinks are worth mentioning, even though this list will likely always be incomplete.

[maintaining APIs]: ../apis/index.md

More specific information on maintaining the standard library can also be found at the [standard library developer's guide][std-dev-guide].

[std-dev-guide]: https://std-dev-guide.rust-lang.org/

## Intra-doc links

At time of writing, rustdoc has a known bug where standard library crates cannot be directly linked *specifically* when compiling standard library crates. This means that, for example, you can't link to the `std` crate from the `core` and `alloc` crates, at least not with the dedicated intra-doc link syntax.

To get around this, many standard library docs unfortunately use absolute paths to refer to specific parts of the standard library documentation, and the [`linkchecker` test] will later verify that there aren't any broken links.

[`linkchecker` test]: https://rustc-dev-guide.rust-lang.org/tests/intro.html#documentation-link-checker

In general, anything in documentation that *can* be a link to a specific item should be a link, and this includes linking something multiple times in the same passage. This is in contrast with other style manuals, like [Wikipedia's], which encourage not linking the same thing multiple times.

[Wikipedia's]: https://en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Linking#Duplicate_and_repeat_links

The only exception to the linking rule is that you shouldn't link to what you're already talking about: for example, in [`String::push_str`], you should not link to [`String`] or [`push_str`][`String::push_str`].

[`String::push_str`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html#method.push_str
[`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html

## `#[must_use]`

In general, `#[must_use]` can be applied to types if failing to consider them is almost certainly a bug. The most common example of this is [`Result`], since failing to consider whether the [`Err`] variant is present is generally a bug: the programmer probably didn't realize that the method might error at all.

[`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
[`Err`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Err

Functions can have `#[must_use]` applied when failing to consider their return values is almost certainly a bug. If the return type is already `#[must_use]`, this is generally redundant, but the attribute can still be added if a more specific message is provided. For example, the operator `saturating_add` is `#[must_use]` because a programmer might wrongfully believe that it mutates its output instead of returning it.

In some cases, explicitly ignoring return values is valid, like for [`thread::JoinHandle`]. In this case, dropping the handle without joining it is a legitimate use case, and we shouldn't require users to explicitly call [`drop`] to accomplish this. If adding `#[must_use]` might lead to cases like this, then it should not be added.

[`thread::JoinHandle`]: https://doc.rust-lang.org/nightly/std/thread/struct.JoinHandle.html
[`drop`]: https://doc.rust-lang.org/nightly/std/mem/fn.drop.html

## `#[doc(alias)]`

Rustdoc allows adding [aliases for items in documentation][doc aliases] via `#[doc(alias = "string")]`. This will recommend the marked items whenever they're searched to bring better visibility to them.

[doc aliases]: https://doc.rust-lang.org/rustdoc/advanced-features.html#add-aliases-for-an-item-in-documentation-search

In general, aliases must represent things users would be *likely to search* in the standard library, and which aliases are allowed is restricted based upon this. Similarly, we expect aliases to represent *other names* for things people are looking for, and not general terms associated with them. We currently *do not* simply accept aliases to other languages' terms or functions in the standard library.

In general, common names for popular system functionality are always allowed as doc aliases for clarity, such as `getcwd`/`GetCurrentDirectory` for [`std::env::current_dir`] and `mkdir`/`CreateDirectory` for [`std::fs::create_dir`]. Even `memcpy` is allowed as an alias for [`std::ptr::copy_nonoverlapping`] since the pervasiveness of C has meant that many people treat it as a fundamental operation on memory.

[`std::env::current_dir`]: https://doc.rust-lang.org/nightly/std/env/fn.current_dir.html
[`std::fs::create_dir`]: https://doc.rust-lang.org/nightly/std/fs/fn.create_dir.html
[`std::ptr::copy_nonoverlapping`]: https://doc.rust-lang.org/nightly/std/ptr/fn.copy_nonoverlapping.html

In the rare case where a system name for an operation is split across multiple items, it is *acceptable* to put the alias on both items. For example, `stat`/`GetFileAttributes` can be aliases for both [`std::fs::metadata`] and [`std::fs::symlink_metadata`]. However, if there ever is a distinction that could make them separate, they must be separate, like how [`std::fs::set_permissions`] is `chmod` and [`std::fs::File::set_permissions`] is `fchmod`.

[`std::fs::metadata`]: https://doc.rust-lang.org/nightly/std/fs/fn.metadata.html
[`std::fs::symlink_metadata`]: https://doc.rust-lang.org/nightly/std/fs/fn.symlink_metadata.html
[`std::fs::set_permissions`]: https://doc.rust-lang.org/nightly/std/fs/fn.set_permissions.html
[`std::fs::File::set_permissions`]: https://doc.rust-lang.org/nightly/std/fs/struct.File.html#method.set_permissions

There are also cases where aliases are added to primitive operations, and in this case, we unfortunately have no choice but to add the aliases to all of them. For example, `popcount` is added for [`{integer}::count_ones`] and `fma` for [`{float}::mul_add`].

[`{integer}::count_ones`]: https://doc.rust-lang.org/nightly/std/primitive.u32.html#method.count_ones
[`{float}::mul_add`]: https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.mul_add

Specifically for [`stdarch`], methods are allowed to have aliases to the relevant hardware instructions as long as there aren't too many duplicates.

[`stdarch`]: https://github.com/rust-lang/stdarch

We additionally allow crate names to be used in the case where popular crates existed *before* things were added to the standard library: for example,  `num_cpus` is an alias for [`std::thread::available_parallelism`] and `BStr` is an alias for [`ByteStr`].

[`std::thread::available_parallelism`]: https://doc.rust-lang.org/nightly/std/thread/fn.available_parallelism.html
[`ByteStr`]: https://doc.rust-lang.org/nightly/std/bstr/struct.ByteStr.html

## Safety comments

The standard library uses [`tidy`] to enforce safety comments on all unsafe code. In general, all `unsafe { ... }` blocks, all `#[unsafe(...)]` attributes, and all `unsafe impl` blocks should have a comment above them that starts with `// SAFETY:` to indicate documentation on why a particular operation is safe to perform or why a trait implementation is safe to implement.

[`tidy`]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/tidy/Readme.md

In general, `ignore-tidy-undocumented-unsafe` should *never* be used except when porting code which currently *does not* have safety comments to enable enforcement via tidy. If a change ever touches code with such undocumented unsafe markers, it should update to use a proper safety comment. Ideally, unsafe blocks should be relatively minimal to ensure that each unsafe operation is documented, but unsafe blocks are allowed to contain multiple related operations if they would have similar safety comments.

## LLM usage

By default, the library team follows the [`rust-lang/rust` LLM policy](../../policies/llm-usage.md) unless otherwise stated.
