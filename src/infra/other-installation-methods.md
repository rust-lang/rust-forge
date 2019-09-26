# Other Rust Installation Methods

- [Which installer should you use?](#which)
- [Other ways to install `rustup`](#more-rustup)
- [Standalone installers](#standalone)
- [Source code](#source)

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
  macOS with [Homebrew], and Windows with [Chocolatey] or [Scoop], developers sometimes
  prefer to install Rust with their platform's package manager.
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
binary builds for all tier 1 and tier 2 platforms, and they are all installable
via `rustup`. Some tier 2 platforms though have only the standard library
available, not the compiler itself; that is, they are cross-compilation targets
only; Rust code can run on those platforms, but they do not run the compiler
itself. Such targets can be installed with the `rustup target add` command.

## Other ways to install `rustup`

<span id="rustup"></span>

The way to install `rustup` differs by platform:

- On Unix, run `curl https://sh.rustup.rs -sSf | sh` in your shell. This
  downloads and runs [`rustup-init.sh`], which in turn downloads and runs the
  correct version of the `rustup-init` executable for your platform.
- On Windows, download and run [`rustup-init.exe`].

`rustup-init` can be configured interactively, and all options can additionally
be controlled by command-line arguments, which can be passed through the shell
script. Pass `--help` to `rustup-init` as follows to display the arguments
`rustup-init` accepts:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

If you prefer not to use the shell script, you may directly download
`rustup-init` for the platform of your choice:

<!-- `{{#rustup_init_list}}`, `{{#installer_table}}`, and `{{#source_code_table}}`
are generated at build time. Please refer to the `blacksmith` preprocessor for
what they each specifically generate.
-->

{{#rustup_init_list}}

## Standalone installers

<span id="standalone"></span>

The official Rust standalone installers contain a single release of Rust, and
are suitable for offline installation. They come in three forms: tarballs
(extension `.tar.gz`), that work in any Unix-like environment, Windows
installers (`.msi`), and Mac installers (`.pkg`). These installers come with
`rustc`, `cargo`, `rustdoc`, the standard library, and the standard
documentation, but do not provide access to additional cross-targets like
`rustup` does.

The most common reasons to use these are:

- Offline installation
- Prefering a more platform-integrated, graphical installer on Windows

Each of these binaries is signed with the [Rust signing key], which is
[available on keybase.io], by the Rust build infrastructure, with [GPG]. In the
tables below, the `.asc` files are the signatures.

<!-- FIXME: Show this sentence again once we've found a quick way to display the archives.
Past releases can be found in [the archives].
-->

{{#installer_table}}

## Source code

{{#source_code_table}}

[installation page]: https://www.rust-lang.org/tools/install
[`rustup`]: https://github.com/rust-lang/rustup.rs
[other-rustup]: https://github.com/rust-lang/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[homebrew]: http://brew.sh/
[chocolatey]: http://chocolatey.org/
[scoop]: https://scoop.sh/
[three tiers]: https://forge.rust-lang.org/platform-support.html
[rust signing key]: https://static.rust-lang.org/rust-key.gpg.ascii
[gpg]: https://gnupg.org/
[available on keybase.io]: https://keybase.io/rust
[the archives]: https://static.rust-lang.org/dist/index.html

