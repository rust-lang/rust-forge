# Archive of Rust Stable Standalone Installers

The official Rust standalone installers contain a single release of Rust, and
are suitable for offline installation. They come in three forms: tarballs
(extension `.tar.gz`), that work in any Unix-like environment, Windows
installers (`.msi`), and Mac installers (`.pkg`). These installers come with
`rustc`, `cargo`, `rustdoc`, the standard library, and the standard
documentation, but do not provide access to additional cross-targets like
`rustup` does.

The most common reasons to use these are:

- Offline installation
- Preferring a more platform-integrated, graphical installer on Windows

Each of these binaries is signed with the [Rust signing key], which is
[available on keybase.io], by the Rust build infrastructure, with [GPG]. In the
tables below, the `.asc` files are the signatures.

{{#previous_stable_standalone_installers_tables}}
