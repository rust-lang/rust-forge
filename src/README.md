# Rust Forge
Welcome to the Rust Forge! Rust Forge serves as a repository of supplementary
documentation useful for members of [The Rust Programming Language]. If
you find any mistakes, typos, or want to add to the Rust Forge, feel free to
file an issue or PR on the [Rust Forge GitHub].

[The Rust Programming Language]: https://rust-lang.org
[Rust Forge GitHub]: https://github.com/rust-lang/rust-forge

<!-- All `<span id="..."></span>` elements are filled at run time when a reader
visits the website. Please refer to `js/index.js` for how these values
are generated.

Avoid changing the "Current Release Versions" without also updating the selector
in `js/index.js.
-->

### Current Release Versions

Channel | Version | Release Date
--------|---------|-------------
Stable  | <span id="stable-version"></span>  | <span id="stable-release-date"></span>
Beta    | <span id="beta-version"></span>    | <span id="beta-release-date"></span>
Nightly | <span id="nightly-version"></span> | <span id="nightly-release-date"></span>


### No Tools Breakage Week
To ensure the beta release includes all the tools, no [tool breakages] are
allowed in the week before the beta cutoff (except for nightly-only tools).

Channel | Version | No Breakage Week
--------|---------|-------------
Beta    | <span id="beta-cycle"></span>    | <span id="beta-timespan"></span>
Nightly | <span id="nightly-cycle"></span> | <span id="nightly-timespan"></span>

[tool breakages]: ./infra/toolstate.md

### External Links

* [Bibliography] of research papers and other projects that influenced Rust.
* [Rust Pontoon] is a translation management system used to localize the Rust
  website.

[Bibliography]: https://rust-lang.github.io/rustc-dev-guide/appendix/bibliography.html
[Rust Pontoon]: https://pontoon.rust-lang.org/
