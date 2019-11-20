# Rust Forge

Welcome to the [Rust Forge]! Rust Forge serves as a repository of supplementary
documentation useful for members of [The Rust Programming Language].

[the rust programming language]: https://rust-lang.org
[rust forge]: https://forge.rust-lang.org

# Building

```
$ mdbook build
```

# Development

When developing it's recommended to use the `serve` command to launch a local
server to allow you to easily see and update changes you make.

```
$ mdbook serve
```

## JavaScript

Forge uses JavaScript to display dates for releases and "no tools breakage
week". When making modifications to the JavaScript make sure it matches the
[standard] style. You can install `standard` and automatically format the code
using the following commands.

[standard]: https://standardjs.com/index.html

### Install

```bash
# With Yarn
yarn global add standard
# With NPM
npm install --global standard
```

### Formatting

```bash
standard --fix js/
```
