---
layout: default
title: The Rust Release Channel Layout
---

**NOTE** This document should be considered incomplete and descriptive rather
than normative. Do not rely on anything described herein to be fully correct
or a definition of how things _should_ be done.

A lot of the content herein is derived from [a posting made to the Rust internals
forum by Brian Anderson back in 2016](https://internals.rust-lang.org/t/future-updates-to-the-rustup-distribution-format/4196#the-static-rust-lang-org-layout).

# The Rust Release Channel Layout

Rust releases are deployed onto `static.rust-lang.org` where they are served
via `https`. There are several parts to a release channel (`stable`, `beta`,
`nightly`) but they all key off a manifest file and then go from there.

## Channel manifests

There is a top level directory `/dist/` which contains the channel manifests. The
manifests are named `channel-rust-[channelname].toml`. Each channel manifest
is accompanied by a `.sha256` file which is a checksum of the manifest file and
can be used to check integrity of the downloaded data. In addition each channel's
manifest is also accompanied by a `.asc` file which is a detached GPG signature
which can be used to check not only the integrity but also the authenticity of
the channel manifest.

In addition to the `stable`, `beta`, and `nightly` channels, there is also a manifest
for each release which will be called `channel-rust-x.yy.z.toml` with its
associated `.sha256` and `.asc` files.

To support date-based channels, there is an archive folder for each day (labelled
`YYYY-MM-DD`) which contains copies of the requisite channel files on that day.
So, for example, if you installed `nightly-2019-02-16` then the channel file
would be <https://static.rust-lang.org/dist/2019-02-16/channel-rust-nightly.toml>.

### Content of channel manifests

Channel manifests are `toml` files. These are known as _v2_ manifests. The _v1_
manifests are simply lists of the files associated with a release and are not
generated for every channel all of the time. Currently it is recommended to
work only with the _v2_ manifests and these are the topic of this section.

The top level of the `.toml` file consists of two important key/value pairs.
Firstly the `manifest-version` which is, at this time, `"2"`, and secondly
the date of the manifest (`date`) whose value is of the form `"YYYY-MM-DD"`.

There are then a number of top level sections (tables) which are:

- `pkg` - This contains the bulk of the manifest and lists the packages which
  are part of the release. Typically this will be things like `rust`, `rustc`,
  `cargo` etc. The `rust` package is semi-special and currently is used to
  specify the subset of other packages which will be installed by default.

  Within packages are `components` and `extensions`. Currently `components` are
  installed by default by `rustup`, `extensions` are optional components and
  are available via `rustup component add` and friends.

- `renames` - This contains a set of package renames which can be used to determine
  the correct package to fetch when the user enters an alias for it.

  Typically renames are used when a package leaves its preview state and is considered
  to be release quality. For example, the actual package for `rustfmt` is called
  `rustfmt-preview` but since its release there has been a `renames.rustfmt`
  table whose `to` field is `rustfmt`. When the user runs `rustup component add rustfmt`
  the name is automatically translated to `rustfmt-preview` and when the user
  runs `rustup component list` then `rustfmt-preview` is automatically renamed
  back to `rustfmt` for display to the user.

- `profiles` - This is part of the future setup for deciding the default component
  set to install. Instead of choosing the `components` of `pkg.rust` instead
  `rustup` will honour one of the entries in the `profiles` table. Usually this
  will be the `default` entry which _essentially_ (though not exactly) boils down
  to `["rustc", "cargo", "rust-std", "rust-docs", "rustfmt", "clippy"]`.

  Other profiles include `minimal` (`["rustc", "cargo", "rust-std"]`) and
  `complete` which adds in additional tools such as the `rls`, a copy of
  the standard library source (`rust-src`), `miri`, `lldb`, `llvm-tools`, and
  `rust-analysis`.

### Package entries in the channel manifest

As stated above, packages list their components and extensions (mostly just the
`rust` package) and they can provide per-target tarball and sha256 data.

For example, a package might be:

```toml
[pkg.cargo.target.powerpc64-unknown-linux-gnu]
available = true
url = "https://static.rust-lang.org/dist/2019-05-23/cargo-0.36.0-powerpc64-unknown-linux-gnu.tar.gz"
hash = "279f3a84f40e3547a8532c64643f38068accb91c21f04cd16e46579c893f5a06"
xz_url = "https://static.rust-lang.org/dist/2019-05-23/cargo-0.36.0-powerpc64-unknown-linux-gnu.tar.xz"
xz_hash = "cf93b387508f4aea4e64f8b4887d70cc07a00906b981dc0c143e92e918682e4a"
```

Here you can see that this is for the `cargo` package, and for the `powerpc64-unknown-linux-gnu` target.
The `url`/`hash` combo is for a `.tar.gz` and the `xz_url`/`xz_hash` pair for the same
tarball compressed with `xz`.
Either pair of url and hash could be present, both may be present, but it is not
useful for neither to be present unless `available` is set to `false` to indicate
that that particular combiantion of package and target is unavailable in this channel
at this time.

In addition, there will be a single entry providing the version for a package
in the form:

```toml
[pkg.cargo]
version = "0.36.0 (6f3e9c367 2019-04-04)"
```

Here `version` will be effectively the `$tool --version` output, minus the tool's name.

## Targets

Targets are the same triples you might use when building something with `cargo build --target=$target`
and you can add them to your installation using `rustup target add $target`.
When you do that, what `rustup` actually does is to find the `rust-std` package
for the target in question and installs that. Essentially like an imaginary
`rustup component add rust-std.$target`.

If a `rust-std` package for a target is not `available = true` then that target
cannot be installed via `rustup`. This can happen for lower tier targets from
time to time.

Since components and extensions are target-specific in the `pkg` tables, you
will be able to see that `rust-std` for every target is specified in every
`rust` target's extensions. This allows for cross-compilation by installation
of any `rust-std` on any build system.
