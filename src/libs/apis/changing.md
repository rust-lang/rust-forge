# Changing APIs

All new guarantees made by the standard library need [an FCP](../membership.md#fcp-process), but what constitutes a new guarantee or breaking change is extremely unclear. This list attempts to cover as many cases as possible that need stabilization based upon experience, although it definitely may be incomplete.

For changes which aren't of concern for API surface, see the section on [code review].

[code review]: ../impls/review.md

## API surface

Any new API added to the standard library needs an FCP. New types, new methods, new traits, and new trait implementations all represent changes that need an FCP to make. These will usually, but not always, involve changing an `#[unstable]` attribute to a `#[stable]` attribute, or a `#[rustc_const_unstable]` attribute to a `#[rustc_const_stable]` attribute. Note that stability attributes will also change from including a tracking issue to a Rust version, and the special `CURRENT_RUSTC_VERSION` string should be used for new stabilizations; these will be replaced with the correct version when the version is actually released.

## Implementation-derived guarantees

There are several ways that changing Rust code can unintentionally make new guarantees for an API. For example, changing trait bounds *usually* represents a new guarantee, although some bounds can never be changed due to them *removing* guarantees. Changing [restrictions] (unstable) can also affect guarantees.

[restrictions]: https://github.com/rust-lang/rust/issues/105077

Even outside trait bounds, changing the inner contents of a type may affect its [variance] in ways which are publicly noticeable, which are new guarantees. The contents of a type may also subtly affect its implementation of [auto traits] like [`Send`] and [`Sync`] which can cause publicly noticeable changes.

[variance]: https://doc.rust-lang.org/nightly/reference/subtyping.html#subtyping.variance
[auto traits]: https://doc.rust-lang.org/nightly/reference/special-types-and-traits.html#auto-traits
[`Send`]: https://doc.rust-lang.org/nightly/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/nightly/std/marker/trait.Sync.html

Even something simple as field reordering can change the drop order of things, which is also worth keeping in mind.

## Type inference

[RFC 1105] explicitly details what kinds of breakages are considered acceptable, and one of those breakages is adding new trait implementations. Unfortunately, things aren't that easy.

[RFC 1105]: https://rust-lang.github.io/rfcs/1105-api-evolution.html

Due to the way the type system works, type inference can break if new trait implementations are added. For example, imagine the following trait impl:

```rust
impl From<&str> for Arc<str> { /* ... */ }
```

With only this impl, the following code works and will correctly infer a conversion from `&str` to `Arc<str>`:

```rust
let b = Arc::from("a");
```

However, if we add a new impl:

```rust
impl From<&str> for Arc<[u8]> { /* ... */ }
```

All of a sudden, the code becomes ambiguous; the type of the result cannot be inferred from the method call. Technically, breakages like this are allowed by our guarantees, but if too many Rust users rely on it, we may decide to disallow them anyway. Similarly, generalizing methods with traits can *also* have this kind of issue, for example:

```rust
fn new<T>(a: &str) -> Arc<T>
where
    Arc<T>: From<&str>
{ /* ... */ }
```

has the same issue. Sometimes, even adding *unstable* features can still result in inference failures.

While [crater] can be used to determine the exact impact of a change on the larger ecosystem, it is not bulletproof and the team may choose to be overly cautious when accepting changes.

## Deref coercion

In addition to type inference, deref coercion can also break depending on new implementations. For example, with just the following:

```rust
impl Deref for String { type Target = str; /* ... */ }
impl Add<&str> for String { /* ... */ }
```

The following code works, since `b` is deref-coerced from `&String` into `&str`:

```rust
let a = String::from("a");
let b = String::from("b");
let c = a + &b;
```

However, if we add a new impl:

```rust
impl Add<char> for String { /* ... */ }
```

Suddenly, Rust won't perform deref coercion and complain about `Add<&String>` missing instead. These types of cases are especially tricky to notice.

## Method resolution

New methods can sometimes affect code in unpredictable ways. For example, unstable methods added to `Iterator` with the same name as methods on other popular crates like [`itertools`] can cause unintended side effects. In general, these cases will trigger the [`unstable-name-collisions` lint], but libs can still be reluctant to make changes for extremely common method names. In the past, we've even made [dedicated compiler workarounds], [multiple times] to get around method resolution issues.

[`Iterator`]: https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html
[`itertools`]: https://docs.rs/itertools
[`unstable-name-collisions` lint]: https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unstable-name-collisions
[dedicated compiler workarounds]: https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html
[multiple times]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/IntoIterator-box-slice.html

This also includes the case where `TryFrom` and `TryInto` were added to the prelude for [future editions] due to method resolution issues.

[future editions]: https://doc.rust-lang.org/nightly/edition-guide/rust-2021/prelude.html

## Macro resolution

Due to a current bug in the compiler(?), unstable macros have the same priority as stable macros and macro additions can have noticeable effects on crates *even when unstable*. This was encountered when attempting to create the [`assert_matches!` macro].

[`assert_matches!` macro]: https://github.com/rust-lang/rust/issues/82913

## Unspecified behavior

In general, users of the standard library shouldn't rely on undocumented guarantees of APIs, but sometimes, it happens anyway. Every beta release of Rust is run through [crater] to find regressions across the ecosystem, and sometimes, this means that the library team will need to FCP changes that already were made, or ones that can technically be made within our guarantees.

[crater]: https://rustc-dev-guide.rust-lang.org/tests/crater.html

In general, any documentation change which represents a new guarantee for an API should have FCP approval, and any implementation change which might substantially affect users should *also* have FCP approval, at the discretion of the [FCP team].

[FCP team]: ../membership.md#fcp-membership

If a change does get made but crater reports too many breakages, the team may opt to work with crate maintainers to fix the issue before stabilizing a feature, or implement [dedicated compiler workarounds](./changing.md#edition-dependent-resolution).
