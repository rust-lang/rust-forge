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

## `#[fundamental]`

Normally, the orphan rule allows adding new trait implementations to types defined in a crate without worrying about ecosystem breakage. However, for some types, this actually becomes impossible, as is the case with:

* `&T`
* `&mut T`
* `Box<T>`
* `Pin<T>`

In all of these cases, even though the parent type (`&_`, `&mut _`, `Box<_>`, and `Pin<_>`) is defined in the standard library, downstream crates can
add trait implementations as long as `T` is a type added in their own crates.

This means that all of a sudden, stabilizations for traits are a very big deal, since we need to decide before stabilization whether *any* fundamental types should be included, or risk never being able to include them.

## `#[non_exhaustive]`

`#[non_exhaustive]` is a useful tool that should be applied to enums, structs, and enum variants *before* stabilization, or else it might be impossible to add.

`#[non_exhaustive]` enums specifically allow for extra variants to be added in the future without any breaking changes. This is especially important for [`std::io::ErrorKind`], which has new variants added all the time.

[`std::io::ErrorKind`]: https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html

`#[non_exhaustive]` structs specifically allow for extra fields to be added in the future, if the struct otherwise has only public fields. For enum variants, this is especially important, since their fields are always public.

## Compiler intrinsics

Compiler intrinsics, located in [`std::intrinsics`], provide special features that Rust would otherwise not be able to do normally. For example, atomic operations in [`std::sync::atomic`] are implemented using compiler intrinsics, since these otherwise have no way of being represented in Rust. Despite the name being *compiler* intrinsics, in general, intrinsics that are exposed by standard library APIs are guarantees of the *language* and have to be approved by the language team when stably exposed for the first time.

[`std::intrinsics`]: https://doc.rust-lang.org/nightly/std/intrinsics/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/nightly/std/sync/atomic/index.html

Although the exact naming of compiler intrinsics is left unstable, any implementation of a compiler for Rust has to have them to implement the standard library, and thus they represent a language-level guarantee. Additionally, the specific behavior of intrinsics may be of interest to the operational semantics (opsem) subteam of lang as well.

That said, some compiler intrinsics are just implementation details and *do not* have to be implemented by every compiler to work correctly. These intrinsics are tagged with the `#[miri::intrinsic_fallback_is_spec]` and have a relevant pure-Rust implementation that can be used by `miri` without changing any language-level guarantees. The language team did a blanket FCP allowing the stabilization of these intrinsics without their approval in [#161081].

[#161081]: https://github.com/rust-lang/rust/pull/161081#issuecomment-5344298013

A good rule of thumb for telling the difference between *internal* intrinsics and *language-level* intrinsics is that internal intrinsics will generally depend on target-specific behavior (for example, floating-point arithmetic) whereas internal intrinsics will not (for example, integer arithmetic).

In addition to the initial FCP of intrinsics from the lang team, an additional FCP is required for stabilizing the use of intrinsics in `const` context. The first FCP involves changing the `#[unstable]` attribute to `#[stable]`, and the second FCP involves changing the `#[rustc_const_unstable]` attribute to `#[rustc_const_stable]`. Sometimes, these two FCPs may be combined if `const`-stabilizing an intrinsic is uncontroversial.
