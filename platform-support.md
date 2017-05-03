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

|  Target                           | std |rustc|cargo| notes                        |
|-----------------------------------|-----|-----|-----|------------------------------|
| `aarch64-apple-ios`               |  ✓  |     |     | ARM64 iOS                    |
| `aarch64-unknown-linux-gnu`       |  ✓  |  ✓  |  ✓  | ARM64 Linux                  |
| `arm-linux-androideabi`           |  ✓  |     |     | ARMv5TE Android              |
| `arm-unknown-linux-gnueabi`       |  ✓  |  ✓  |  ✓  | ARMv6 Linux                  |
| `arm-unknown-linux-musleabi`      |  ✓  |     |     | ARMv6 Linux with MUSL        |
| `arm-unknown-linux-gnueabihf`     |  ✓  |  ✓  |  ✓  | ARMv6 Linux, hardfloat       |
| `arm-unknown-linux-musleabihf`    |  ✓  |     |     | ARMv6 Linux, MUSL, hardfloat |
| `armv7-apple-ios`                 |  ✓  |     |     | ARMv7 iOS, Cortex-a8         |
| `armv7-unknown-linux-gnueabihf`   |  ✓  |  ✓  |  ✓  | ARMv7 Linux                  |
| `armv7-unknown-linux-musleabihf`  |  ✓  |     |     | ARMv7 Linux with MUSL        |
| `armv7s-apple-ios`                |  ✓  |     |     | ARMv7 iOS, Cortex-a9         |
| `asmjs-unknown-emscripten`        |  ✓  |     |     | asm.js via Emscripten        |
| `i386-apple-ios`                  |  ✓  |     |     | 32-bit x86 iOS               |
| `i586-pc-windows-msvc`            |  ✓  |     |     | 32-bit Windows w/o SSE       |
| `i686-unknown-linux-musl`         |  ✓  |     |     | 32-bit Linux with MUSL       |
| `mips-unknown-linux-gnu`          |  ✓  |  ✓  |  ✓  | MIPS Linux                   |
| `mips-unknown-linux-musl`         |  ✓  |     |     | MIPS Linux with MUSL         |
| `mipsel-unknown-linux-gnu`        |  ✓  |  ✓  |  ✓  | MIPS (LE) Linux              |
| `mipsel-unknown-linux-musl`       |  ✓  |     |     | MIPS (LE) Linux with MUSL    |
| `mips64-unknown-linux-gnuabi64`   |  ✓  |  ✓  |  ✓  | MIPS64 Linux, n64 ABI        |
| `mips64el-unknown-linux-gnuabi64` |  ✓  |  ✓  |  ✓  | MIPS64 (LE) Linux, n64 ABI   |
| `powerpc-unknown-linux-gnu`       |  ✓  |  ✓  |  ✓  | PowerPC Linux                |
| `powerpc64-unknown-linux-gnu`     |  ✓  |  ✓  |  ✓  | PPC64 Linux                  |
| `powerpc64le-unknown-linux-gnu`   |  ✓  |  ✓  |  ✓  | PPC64LE Linux                |
| `s390x-unknown-linux-gnu`         |  ✓  |  ✓  |  ✓  | S390x Linux                  |
| `wasm32-unknown-emscripten`       |  ✓  |     |     | WebAssembly via Emscripten   |
| `x86_64-apple-ios`                |  ✓  |     |     | 64-bit x86 iOS               |
| `x86_64-rumprun-netbsd`           |  ✓  |     |     | 64-bit NetBSD Rump Kernel    |
| `x86_64-unknown-freebsd`          |  ✓  |  ✓  |  ✓  | 64-bit FreeBSD               |
| `x86_64-unknown-linux-musl`       |  ✓  |     |     | 64-bit Linux with MUSL       |
| `x86_64-unknown-netbsd`           |  ✓  |  ✓  |  ✓  | 64-bit NetBSD                |

## Tier 3

Tier 3 platforms are those which the Rust codebase has support for, but
which are not built or tested automatically, and may not work.
Official builds are not available.

|  Target                         | std |rustc|cargo| notes                                                    |
|---------------------------------|-----|-----|-----|----------------------------------------------------------|
| `aarch64-linux-android`         |  ✓  |     |     | ARM64 Android                                            |
| `aarch64-unknown-fuchsia`       |  ✓  |     |     | Fuchsia OS                                               |
| `armv5te-unknown-linux-gnueabi` |  ✓  |     |     | ARMv5TE                                                  |
| `armv7-linux-androideabi`       |  ✓  |     |     | ARMv7a Android                                           |
| `i586-unknown-linux-gnu`        |  ✓  |     |     | 32-bit Linux w/o SSE                                     |
| `i686-linux-android`            |  ✓  |     |     | 32-bit x86 Android                                       |
| `i686-pc-windows-msvc` (XP)     |  ✓  |     |     | Windows XP support                                       |
| `i686-unknown-freebsd`          |  ✓  |  ✓  |  ✓  | 32-bit FreeBSD                                           |
| `i686-unknown-haiku`            |  ✓  |     |     | 32-bit Haiku                                             |
| `le32-unknown-nacl`             |  ✓  |     |     | PNaCl sandbox                                            |
| `mips-unknown-linux-uclibc`     |  ✓  |     |     | MIPS Linux with uClibc                                   |
| `mipsel-unknown-linux-uclibc`   |  ✓  |     |     | MIPS (LE) Linux with uClibc                              |
| `sparc64-unknown-linux-gnu`     |  ✓  |     |     | SPARC Linux                                              |
| `sparc64-unknown-netbsd`        |  ✓  |     |     | SPARC NetBSD                                             |
| `thumbv6m-none-eabi`            |  *  |     |     | Bare Cortex-M0, M0+, M1                                  |
| `thumbv7em-none-eabi`           |  *  |     |     | Bare Cortex-M4, M7                                       |
| `thumbv7em-none-eabihf`         |  *  |     |     | Bare Cortex-M4F, M7F, FPU, hardfloat                     |
| `thumbv7m-none-eabi`            |  *  |     |     | Bare Cortex-M3                                           |
| `x86_64-linux-android`          |  ✓  |     |     | 64-bit x86 Android                                       |
| `x86_64-pc-windows-msvc` (XP)   |  ✓  |     |     | Windows XP support                                       |
| `x86_64-sun-solaris`            |  ✓  |  ✓  |     | 64-bit Solaris/SunOS                                     |
| `x86_64-unknown-bitrig`         |  ✓  |  ✓  |     | 64-bit Bitrig                                            |
| `x86_64-unknown-dragonfly`      |  ✓  |  ✓  |     | 64-bit DragonFlyBSD                                      |
| `x86_64-unknown-fuchsia`        |  ✓  |     |     | Fuchsia OS                                               |
| `x86_64-unknown-haiku`          |  ✓  |     |     | 64-bit Haiku                                             |
| `x86_64-unknown-openbsd`        |  ✓  |  ✓  |     | 64-bit OpenBSD                                           |
| `x86_64-unknown-redox`          |  ✓  |     |     | Redox OS                                                 |
| [MSP430]                        |  ** |     |     | 16-bit MSP430 microcontroller                            |
| [NVPTX]                         |  ** |     |     | `--emit=asm` generates PTX code that runs on NVIDIA GPUs |

[MSP430]: https://github.com/pftbest/rust_on_msp/blob/master/msp430.json
[NVPTX]: https://github.com/japaric/nvptx#targets

<em>*</em> These are bare-metal microcontroller targets that only have access to
the core library, not std.

<em>**</em> There's backend support for these targets but no target built into
`rustc` (yet). You'll have to write your own target specification file (see the
links in the table). These targets only support the core library.

But those aren't the only platforms Rust can compile to! Those are the ones with
built-in target definitions and/or standard library support. When linking only
to the core library, Rust can also target "bare metal" in the x86, ARM, MIPS, and
PowerPC families, though it may require defining custom target specifications to
do so. All such scenarios are tier 3.
