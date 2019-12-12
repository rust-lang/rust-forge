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

target | std | rustc | cargo | notes
-------|-----|-------|-------|-------
`i686-apple-darwin` | ✓ | ✓ | ✓ | 32-bit OSX (10.7+, Lion+)
`i686-pc-windows-gnu` | ✓ | ✓ | ✓ | 32-bit MinGW (Windows 7+)
`i686-pc-windows-msvc` | ✓ | ✓ | ✓ | 32-bit MSVC (Windows 7+)
`i686-unknown-linux-gnu` | ✓ | ✓ | ✓ | 32-bit Linux (2.6.18+)
`x86_64-apple-darwin` | ✓ | ✓ | ✓ | 64-bit OSX (10.7+, Lion+)
`x86_64-pc-windows-gnu` | ✓ | ✓ | ✓ | 64-bit MinGW (Windows 7+)
`x86_64-pc-windows-msvc` | ✓ | ✓ | ✓ | 64-bit MSVC (Windows 7+)
`x86_64-unknown-linux-gnu` | ✓ | ✓ | ✓ | 64-bit Linux (2.6.18+)

## Tier 2
Tier 2 platforms can be thought of as "guaranteed to build". Automated tests
are not run so it's not guaranteed to produce a working build, but platforms
often work to quite a good degree and patches are always welcome!
Specifically, these platforms are required to have each of the following:

* Official binary releases are provided for the platform.
* Automated building is set up, but may not be running tests.
* Landing changes to the `rust-lang/rust` repository's master branch is gated on
    platforms **building**. For some platforms only the standard library is
    compiled, but for others `rustc` and `cargo` are too.

target | std | rustc | cargo | notes
-------|-----|-------|-------|-------
`aarch64-apple-ios` | ✓ |  |  | ARM64 iOS
`aarch64-fuchsia` | ✓ |  |  | ARM64 Fuchsia
`aarch64-linux-android` | ✓ |  |  | ARM64 Android
`aarch64-pc-windows-msvc` | ✓ |  |  | ARM64 Windows MSVC
`aarch64-unknown-linux-gnu` | ✓ | ✓ | ✓ | ARM64 Linux
`aarch64-unknown-linux-musl` | ✓ |  |  | ARM64 Linux with MUSL
`arm-linux-androideabi` | ✓ |  |  | ARMv7 Android
`arm-unknown-linux-gnueabi` | ✓ | ✓ | ✓ | ARMv6 Linux
`arm-unknown-linux-gnueabihf` | ✓ | ✓ | ✓ | ARMv6 Linux, hardfloat
`arm-unknown-linux-musleabi` | ✓ |  |  | ARMv6 Linux with MUSL
`arm-unknown-linux-musleabihf` | ✓ |  |  | ARMv6 Linux with MUSL, hardfloat
`armebv7r-none-eabi` | * |  |  | Bare ARMv7-R, Big Endian
`armebv7r-none-eabihf` | * |  |  | Bare ARMv7-R, Big Endian, hardfloat
`armv5te-unknown-linux-gnueabi` | ✓ |  |  | ARMv5TE Linux
`armv5te-unknown-linux-musleabi` | ✓ |  |  | ARMv5TE Linux with MUSL
`armv7-apple-ios` | ✓ |  |  | ARMv7 iOS, Cortex-a8
`armv7-linux-androideabi` | ✓ |  |  | ARMv7a Android
`armv7-none-eabi` | * |  |  | Bare ARMv7-R
`armv7-none-eabihf` | * |  |  | Bare ARMv7-R, hardfloat
`armv7-unknown-linux-gnueabihf` | ✓ | ✓ | ✓ | ARMv7 Linux
`armv7-unknown-linux-musleabihf` | ✓ |  |  | ARMv7 Linux with MUSL
`asmjs-unknown-emscripten` | ✓ |  |  | asm.js via Emscripten
`i386-apple-ios` | ✓ |  |  | 32-bit x86 iOS
`i586-pc-windows-msvc` | ✓ |  |  | 32-bit Windows w/o SSE
`i586-unknown-linux-gnu` | ✓ |  |  | 32-bit Linux w/o SSE
`i586-unknown-linux-musl` | ✓ |  |  | 32-bit Linux w/o SSE, MUSL
`i686-linux-android` | ✓ |  |  | 32-bit x86 Android
`i686-unknown-freebsd` | ✓ | ✓ | ✓ | 32-bit FreeBSD
`i686-unknown-linux-musl` | ✓ |  |  | 32-bit Linux with MUSL
`mips-unknown-linux-gnu` | ✓ | ✓ | ✓ | MIPS Linux
`mips-unknown-linux-musl` | ✓ |  |  | MIPS Linux with MUSL
`mips64-unknown-linux-gnuabi64` | ✓ | ✓ | ✓ | MIPS64 Linux, n64 ABI
`mips64el-unknown-linux-gnuabi64` | ✓ | ✓ | ✓ | MIPS64 (LE) Linux, n64 ABI
`mipsel-unknown-linux-gnu` | ✓ | ✓ | ✓ | MIPS (LE) Linux
`mipsel-unknown-linux-musl` | ✓ |  |  | MIPS (LE) Linux with MUSL
`powerpc-unknown-linux-gnu` | ✓ | ✓ | ✓ | PowerPC Linux
`powerpc64-unknown-linux-gnu` | ✓ | ✓ | ✓ | PPC64 Linux
`powerpc64le-unknown-linux-gnu` | ✓ | ✓ | ✓ | PPC64LE Linux
`riscv32imac-unknown-none-elf` | * |  |  | Bare RISC-V (RV32IMAC ISA)
`riscv32imc-unknown-none-elf` | * |  |  | Bare RISC-V (RV32IMC ISA)
`riscv64gc-unknown-none-elf` | * |  |  | Bare RISC-V (RV64IMAFDC ISA)
`riscv64imac-unknown-none-elf` | * |  |  | Bare RISC-V (RV64IMAC ISA)
`s390x-unknown-linux-gnu` | ✓ | ✓ | ✓ | S390x Linux
`sparc64-unknown-linux-gnu` | ✓ |  |  | SPARC Linux
`sparcv9-sun-solaris` | ✓ |  |  | SPARC Solaris 10/11, illumos
`thumbv6m-none-eabi` | * |  |  | Bare Cortex-M0, M0+, M1
`thumbv7em-none-eabi` | * |  |  | Bare Cortex-M4, M7
`thumbv7em-none-eabihf` | * |  |  | Bare Cortex-M4F, M7F, FPU, hardfloat
`thumbv7m-none-eabi` | * |  |  | Bare Cortex-M3
`thumbv7neon-linux-androideabi` | ✓ |  |  | Thumb2-mode ARMv7a Android with NEON
`thumbv7neon-unknown-linux-gnueabihf` | ✓ |  |  | Thumb2-mode ARMv7a Linux with NEON
`wasm32-unknown-emscripten` | ✓ |  |  | WebAssembly via Emscripten
`wasm32-unknown-unknown` | ✓ |  |  | WebAssembly
`wasm32-wasi` | ✓ |  |  | WebAssembly with WASI
`x86_64-apple-ios` | ✓ |  |  | 64-bit x86 iOS
`x86_64-fortanix-unknown-sgx` | ✓ |  |  | [Fortanix ABI] for 64-bit Intel SGX
`x86_64-fuchsia` | ✓ |  |  | 64-bit Fuchsia
`x86_64-linux-android` | ✓ |  |  | 64-bit x86 Android
`x86_64-rumprun-netbsd` | ✓ |  |  | 64-bit NetBSD Rump Kernel
`x86_64-sun-solaris` | ✓ |  |  | 64-bit Solaris 10/11, illumos
`x86_64-unknown-cloudabi` | ✓ |  |  | 64-bit CloudABI
`x86_64-unknown-freebsd` | ✓ | ✓ | ✓ | 64-bit FreeBSD
`x86_64-unknown-linux-gnux32` | ✓ |  |  | 64-bit Linux (x32 ABI)
`x86_64-unknown-linux-musl` | ✓ | ✓ | ✓ | 64-bit Linux with MUSL
`x86_64-unknown-netbsd` | ✓ | ✓ | ✓ | NetBSD/amd64
`x86_64-unknown-redox` | ✓ |  |  | Redox OS

[Fortanix ABI]: https://edp.fortanix.com/

## Tier 2.5
Tier 2.5 platforms can be thought of as "guaranteed to build", but without
builds available through `rustup`. Automated tests are not run so it's not
guaranteed to produce a working build, but platforms often work to quite a good
degree and patches are always welcome! Specifically, these platforms are
required to have each of the following:

* Automated building is set up, but may not be running tests.
* Landing changes to the `rust-lang/rust` repository's master branch is gated on
  platforms **building**. For some platforms only the standard library is
  compiled, but for others `rustc` and `cargo` are too.

**This status is accidental: no new platforms should reach this state**

target | std | rustc | cargo | notes
-------|-----|-------|-------|-------
`aarch64-unknown-cloudabi` | ✓ |  |  | ARM64 CloudABI
`armv7-unknown-cloudabi-eabihf` | ✓ |  |  | ARMv7 CloudABI, hardfloat
`i686-unknown-cloudabi` | ✓ |  |  | 32-bit CloudABI
`powerpc-unknown-linux-gnuspe` | ✓ |  |  | PowerPC SPE Linux
`sparc-unknown-linux-gnu` | ✓ |  |  | 32-bit SPARC Linux

## Tier 3
Tier 3 platforms are those which the Rust codebase has support for, but which
are not built or tested automatically, and may not work. Official builds are
not available.

target | std | rustc | cargo | notes
-------|-----|-------|-------|-------
`aarch64-unknown-freebsd` | ? |  |  |
`aarch64-unknown-hermit` | ? |  |  |
`aarch64-unknown-netbsd` | ? |  |  |
`aarch64-unknown-none` | ? |  |  |
`aarch64-unknown-openbsd` | ✓ | ✓ | ✓ | ARM64 OpenBSD
`aarch64-unknown-redox` | ? |  |  |
`aarch64-uwp-windows-msvc` | ? |  |  |
`aarch64-wrs-vxworks` | ? |  |  |
`armv4t-unknown-linux-gnueabi` | ? |  |  |
`armv6-unknown-freebsd` | ? |  |  |
`armv6-unknown-netbsd-eabihf` | ? |  |  |
`armv7-unknown-freebsd` | ? |  |  |
`armv7-unknown-netbsd-eabihf` | ? |  |  |
`armv7-wrs-vxworks-eabihf` | ? |  |  |
`hexagon-unknown-linux-musl` | ? |  |  |
`i686-pc-windows-msvc` | ✓ |  |  | 32-bit Windows XP support
`i686-unknown-haiku` | ✓ | ✓ | ✓ | 32-bit Haiku
`i686-unknown-netbsd` | ✓ |  |  | NetBSD/i386 with SSE2
`i686-unknown-openbsd` | ✓ | ✓ | ✓ | 32-bit OpenBSD
`i686-uwp-windows-gnu` | ? |  |  |
`i686-uwp-windows-msvc` | ? |  |  |
`i686-wrs-vxworks` | ? |  |  |
`mips-unknown-linux-uclibc` | ✓ |  |  | MIPS Linux with uClibc
`mips64-unknown-linux-muslabi64` | ? |  |  |
`mips64el-unknown-linux-muslabi64` | ? |  |  |
`mipsel-unknown-linux-uclibc` | ✓ |  |  | MIPS (LE) Linux with uClibc
`mipsisa32r6-unknown-linux-gnu` | ? |  |  |
`mipsisa32r6el-unknown-linux-gnu` | ? |  |  |
`mipsisa64r6-unknown-linux-gnuabi64` | ? |  |  |
`mipsisa64r6el-unknown-linux-gnuabi64` | ? |  |  |
`msp430-none-elf` | * |  |  | 16-bit MSP430 microcontrollers
`nvptx64-nvidia-cuda` | ** |  |  | --emit=asm generates PTX code that [runs on NVIDIA GPUs]
`nvptx64-nvidia-cuda` | ** |  |  | --emit=asm generates PTX code that [runs on NVIDIA GPUs]
`powerpc-unknown-linux-musl` | ? |  |  |
`powerpc-unknown-netbsd` | ? |  |  |
`powerpc-wrs-vxworks` | ? |  |  |
`powerpc-wrs-vxworks-spe` | ? |  |  |
`powerpc64-unknown-freebsd` | ? |  |  |
`powerpc64-unknown-linux-musl` | ? |  |  |
`powerpc64-wrs-vxworks` | ? |  |  |
`powerpc64le-unknown-linux-musl` | ? |  |  |
`riscv32i-unknown-none-elf` | ? |  |  |
`sparc64-unknown-netbsd` | ✓ | ✓ |  | NetBSD/sparc64
`sparc64-unknown-openbsd` | ? |  |  |
`thumbv7a-pc-windows-msvc` | ? |  |  |
`thumbv8m.base-none-eabi` | ? |  |  |
`thumbv8m.main-none-eabi` | ? |  |  |
`thumbv8m.main-none-eabihf` | ? |  |  |
`wasm32-experimental-emscripten` | ? |  |  |
`x86_64-pc-solaris` | ? |  |  |
`x86_64-pc-windows-msvc` | ✓ |  |  | 64-bit Windows XP support
`x86_64-unknown-bitrig` | ✓ | ✓ |  | 64-bit Bitrig
`x86_64-unknown-dragonfly` | ✓ | ✓ | ✓ | 64-bit DragonFlyBSD
`x86_64-unknown-haiku` | ✓ | ✓ | ✓ | 64-bit Haiku
`x86_64-unknown-hermit` | ? |  |  |
`x86_64-unknown-l4re-uclibc` | ? |  |  |
`x86_64-unknown-openbsd` | ✓ | ✓ | ✓ | 64-bit OpenBSD
`x86_64-unknown-uefi` | ? |  |  |
`x86_64-uwp-windows-gnu` | ✓ |  |  |
`x86_64-uwp-windows-msvc` | ✓ |  |  |
`x86_64-wrs-vxworks` | ? |  |  |

[runs on NVIDIA GPUs]: https://github.com/japaric-archived/nvptx#targets

\* These are bare-metal microcontroller targets that only have access to the
   core library, not std.

\*\* There’s backend support for these targets but no target built into rustc
     (yet). You’ll have to write your own target specification file (see the
     links in the table). These targets only support the core library.

? These are targets that haven't yet been documented here. If you can shed some
  light on these platforms support, please create an issue or PR on the [Rust
  Forge repo].

But those aren't the only platforms Rust can compile to! Those are the ones with
built-in target definitions and/or standard library support. When linking only
to the core library, Rust can also target additional "bare metal" platforms in
the x86, ARM, MIPS, and PowerPC families, though it may require defining custom
target specifications to do so.

[Rust Forge repo]: https://github.com/rust-lang/rust-forge
