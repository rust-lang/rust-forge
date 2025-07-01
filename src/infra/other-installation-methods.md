# Other Rust Installation Methods

- [Which installer should you use?](#which)
- [Other ways to install `rustup`](#rustup)
- [Standalone installers](#standalone)
- [Source code](#source-code)

## Which installer should you use?

<span id="which"></span>

Rust runs on many platforms, and there are many ways to install Rust. If you
want to install Rust in the most straightforward, recommended way, then follow
the instructions on the main [installation page].

That page describes installation via [`rustup`], a tool that manages multiple
Rust toolchains in a consistent way across all platforms Rust supports. Why
might one _not_ want to install using those instructions?

- Offline installation. `rustup` downloads components from the internet on
  demand. If you need to install Rust without access to the internet, `rustup`
  is not suitable.
- Preference for the system package manager. On Linux in particular, but also on
  macOS with [Homebrew], [MacPorts] or [pkgsrc], and Windows with [Chocolatey]
  or [Scoop], developers sometimes prefer to install Rust with their platform's
  package manager.
- Preference against `curl | sh`. On Unix, we usually install `rustup` by
  running a shell script via `curl`. Some have concerns about the security of
  this arrangement and would prefer to download and run the installer
  themselves.
- Validating signatures. Although `rustup` performs its downloads over HTTPS,
  the only way to verify the signatures of Rust installers today is to do so
  manually with the standalone installers.
- GUI installation and integration with "Add/Remove Programs" on Windows.
  `rustup` runs in the console and does not register its installation like
  typical Windows programs. If you prefer a more typical GUI installation on
  Windows there are standalone `.msi` installers. In the future `rustup` will
  also have a GUI installer on Windows.

Rust's platform support is defined in [three tiers], which correspond closely
with the installation methods available: in general, the Rust project provides
binary builds for all Tier 1 and Tier 2 platforms, and they are all installable
via `rustup`. Some Tier 2 platforms though have only the standard library
available, not the compiler itself; that is, they are cross-compilation targets
only; Rust code can run on those platforms, but they do not run the compiler
itself. Such targets can be installed with the `rustup target add` command.

## Other ways to install `rustup`

<span id="rustup"></span>

Please refer to `rustup`'s [Other installation
methods](https://rust-lang.github.io/rustup/installation/other.html).

## Standalone installers

<span id="standalone"></span>

{{#include shared-standalone-installers.md}}

Past releases can be found in [the archive].

<!--
`{{#installer_table}}` and `{{#source_code_table}}`
are generated at build time. Please refer to the `blacksmith` preprocessor for
what they each specifically generate.
-->

{{#installer_table}}

## Source code

If you want to build the Rust toolchain from source code, you can use the following
links to download source code tarballs.

{{#source_code_table}}

If you want to make sure that the published source tarball matches what is in the
`rust` git repository, you can use the following script as a template:

<details>
<summary>Script for reproducing source tarball contents</summary>

```bash
#!/bin/bash

set -e

# You can use either a commit SHA or a stable release version (e.g. 1.XY.Z)
TAG=a8cfc83801301c2b4f0fd030192e268eeb15d473
# TAG=1.77.1

# Clone Rust toolchain repository from GitHub
git clone https://github.com/rust-lang/rust
cd rust
git reset --hard ${TAG}

cat >config.toml << EOF
[rust]
# Use for a commit SHA
channel = "nightly"

# Use for a stable release
# channel = "stable"

[dist]
compression-formats = ["xz"]
compression-profile = "fast"
EOF

# Build the source tarball from git into build/dist/
./x dist rustc-src

# Download source tarball for a commit SHA
wget https://ci-artifacts.rust-lang.org/rustc-builds/${TAG}/rustc-nightly-src.tar.xz

# Download a source tarball for a stable release
# wget https://static.rust-lang.org/dist/rustc-${TAG}-src.tar.xz

# Decompress the tarballs and check if they're the same
xz --decompress rustc-*-src.tar.xz
xz --decompress build/dist/rustc-*-src.tar.xz
diff rustc-*-src.tar build/dist/rustc-*-src.tar
```

</details>

[installation page]: https://www.rust-lang.org/tools/install
[`rustup`]: https://github.com/rust-lang/rustup.rs
[other-rustup]: https://github.com/rust-lang/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[homebrew]: http://brew.sh/
[macports]: https://www.macports.org/
[pkgsrc]: https://pkgsrc.joyent.com/install-on-osx/
[chocolatey]: http://chocolatey.org/
[scoop]: https://scoop.sh/
[three tiers]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[the archive]: ../infra/archive-stable-version-installers.md
