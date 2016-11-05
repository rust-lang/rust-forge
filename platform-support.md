---
layout: default
title: Rust Platform Support &middot; The Rust Programming Language
---

# Rust Platform Support

The Rust compiler runs on, and compiles to, a great number of platforms, though
not all platforms are equally supported. Rust's support levels are organized
into three tiers, each with a different set of guarantees.

Platforms are identified by their "target triple" which is the string to inform
the compiler what kind of output should be produced. The columns below indicate
whether the corresponding component works on the specified platform.

## Tier 1

Tier 1 platforms can be thought of as "guaranteed to work".
Specifically they will each satisfy the following requirements:

* Official binary releases are provided for the platform.
* Automated testing is set up to run tests for the platform.
* Landing changes to the `rust-lang/rust` repository's master branch is gated on
  tests passing.
* Documentation for how to use and how to build the platform is available.

|  Target                       | std |rustc|cargo| notes                      |
|-------------------------------|-----|-----|-----|----------------------------|
| `i686-apple-darwin`           |  ✓  |  ✓  |  ✓  | 32-bit OSX (10.7+, Lion+)  |
| `i686-pc-windows-gnu`         |  ✓  |  ✓  |  ✓  | 32-bit MinGW (Windows 7+)  |
| `i686-pc-windows-msvc`        |  ✓  |  ✓  |  ✓  | 32-bit MSVC (Windows 7+)   |
| `i686-unknown-linux-gnu`      |  ✓  |  ✓  |  ✓  | 32-bit Linux (2.6.18+)     |
| `x86_64-apple-darwin`         |  ✓  |  ✓  |  ✓  | 64-bit OSX (10.7+, Lion+)  |
| `x86_64-pc-windows-gnu`       |  ✓  |  ✓  |  ✓  | 64-bit MinGW (Windows 7+)  |
| `x86_64-pc-windows-msvc`      |  ✓  |  ✓  |  ✓  | 64-bit MSVC (Windows 7+)   |
| `x86_64-unknown-linux-gnu`    |  ✓  |  ✓  |  ✓  | 64-bit Linux (2.6.18+)     |

## Tier 2

Tier 2 platforms can be thought of as "guaranteed to build". Automated tests
are not run so it's not guaranteed to produce a working build, but platforms
often work to quite a good degree and patches are always welcome! Specifically,
these platforms are required to have each of the following:

* Official binary releases are provided for the platform.
* Automated building is set up, but may not be running tests.
* Landing changes to the `rust-lang/rust` repository's master branch is gated on
  platforms **building**. For some platforms only the standard library is
  compiled, but for others `rustc` and `cargo` are too.

|  Target                         | std |rustc|cargo| notes                      |
|---------------------------------|-----|-----|-----|----------------------------|
| `aarch64-apple-ios`             |  ✓  |     |     | ARM64 iOS                  |
| `aarch64-unknown-linux-gnu`     |  ✓  |  ✓  |  ✓  | ARM64 Linux (2.6.18+)      |
| `arm-linux-androideabi`         |  ✓  |     |     | ARM Android                |
| `arm-unknown-linux-gnueabi`     |  ✓  |  ✓  |  ✓  | ARM Linux (2.6.18+)        |
| `arm-unknown-linux-gnueabihf`   |  ✓  |  ✓  |  ✓  | ARM Linux (2.6.18+)        |
| `armv7-apple-ios`               |  ✓  |     |     | ARM iOS                    |
| `armv7-unknown-linux-gnueabihf` |  ✓  |  ✓  |  ✓  | ARM Linux (2.6.18+)        |
| `armv7s-apple-ios`              |  ✓  |     |     | ARM iOS                    |
| `i386-apple-ios`                |  ✓  |     |     | 32-bit x86 iOS             |
| `i586-pc-windows-msvc`          |  ✓  |     |     | 32-bit Windows w/o SSE     |
| `mips-unknown-linux-gnu`        |  ✓  |     |     | MIPS Linux (2.6.18+)       |
| `mips-unknown-linux-musl`       |  ✓  |     |     | MIPS Linux with MUSL       |
| `mipsel-unknown-linux-gnu`      |  ✓  |     |     | MIPS (LE) Linux (2.6.18+)  |
| `mipsel-unknown-linux-musl`     |  ✓  |     |     | MIPS (LE) Linux with MUSL  |
| `powerpc-unknown-linux-gnu`     |  ✓  |     |     | PowerPC Linux (2.6.18+)    |
| `powerpc64-unknown-linux-gnu`   |  ✓  |     |     | PPC64 Linux (2.6.18+)      |
| `powerpc64le-unknown-linux-gnu` |  ✓  |     |     | PPC64LE Linux (2.6.18+)    |
| `x86_64-apple-ios`              |  ✓  |     |     | 64-bit x86 iOS             |
| `x86_64-rumprun-netbsd`         |  ✓  |     |     | 64-bit NetBSD Rump Kernel  |
| `x86_64-unknown-freebsd`        |  ✓  |  ✓  |  ✓  | 64-bit FreeBSD             |
| `x86_64-unknown-linux-musl`     |  ✓  |     |     | 64-bit Linux with MUSL     |
| `x86_64-unknown-netbsd`         |  ✓  |  ✓  |  ✓  | 64-bit NetBSD              |

## Tier 3

Tier 3 platforms are those which the Rust codebase has support for, but
which are not built or tested automatically, and may not work.
Official builds are not available.

|  Target                       | std |rustc|cargo| notes                      |
|-------------------------------|-----|-----|-----|----------------------------|
| `aarch64-linux-android`       |  ✓  |     |     | ARM64 Android              |
| `armv7-linux-androideabi`     |  ✓  |     |     | ARM-v7a Android            |
| `i686-linux-android`          |  ✓  |     |     | 32-bit x86 Android         |
| `i686-pc-windows-msvc` (XP)   |  ✓  |     |     | Windows XP support         |
| `i686-unknown-freebsd`        |  ✓  |  ✓  |  ✓  | 32-bit FreeBSD             |
| `x86_64-pc-windows-msvc` (XP) |  ✓  |     |     | Windows XP support         |
| `x86_64-sun-solaris`          |  ✓  |  ✓  |     | 64-bit Solaris/SunOS       |
| `x86_64-unknown-bitrig`       |  ✓  |  ✓  |     | 64-bit Bitrig              |
| `x86_64-unknown-dragonfly`    |  ✓  |  ✓  |     | 64-bit DragonFlyBSD        |
| `x86_64-unknown-openbsd`      |  ✓  |  ✓  |     | 64-bit OpenBSD             |


