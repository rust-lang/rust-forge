# Code review

When reviewing changes to standard library code, there are a lot of sneaky details that can escape even the keenest reviewer's eyes. This attempts to cover several of the various details that the library team thinks are worth mentioning, even though this list will likely always be incomplete.

[maintaining APIs]: ../apis/index.md

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

In general, `#[must_use]` can be applied to types if failing to consider them is almost certainly a bug. The most common example of this is [`Result`], since failing to consider whether the [`Err`] variant is present is generally a bug: the programmer probably didn't realise that the method might error at all.

[`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
[`Err`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Err

Functions can have `#[must_use]` applied when failing to consider their return values is almost certainly a bug. If the return type is already `#[must_use]`, this is generally redundant, but the attribute can still be added if a more specific message is provided. For example, the operator `saturating_add` is `#[must_use]` because a programmer might wrongfully believe that it mutates its output instead of returning it.

In some cases, explicitly ignoring return values is valid, like for [`thread::JoinHandle`]. In this case, dropping the handle without joining it is a legitimate use case, and we shouldn't require users to explicitly call [`drop`] to accomplish this. If adding `#[must_use]` might lead to cases like this, then it should not be added.

[`thread::JoinHandle`]: https://doc.rust-lang.org/nightly/std/thread/struct.JoinHandle.html
[`drop`]: https://doc.rust-lang.org/nightly/std/mem/fn.drop.html

## Placeholder

Wow, can you believe I didn't write this yet?

* `#[may_dangle]`
* `mem::forget` pitfalls
* `mem::replace` with *any* value (e.g. `MaybeUninit`)
* unstable features (e.g. specialization)
* doc alias policy
    * duplicates allowed for syscalls or aliases from other languages, not for bikeshedding
* Safety comments
* Target-specific code
* Unsafe generics
