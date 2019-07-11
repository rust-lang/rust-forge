# [Rust Forge](https://forge.rust-lang.org)

This site contains supplementary documentation useful to the members of
[The Rust Project](https://www.rust-lang.org). To edit it submit PR's against
[rust-lang/rust-forge].

[rust-lang/rust-forge]: https://github.com/rust-lang/rust-forge

<hr/>

- [Release history](releases.md). Links to previous release artifacts.

- [Platform support](platform-support.md).

- [Friend of the Tree archives](fott.md).

- [Bibliography](bibliography.md). Research papers and other links to projects
  that influenced Rust. Papers about Rust.

- [Release process](release-process.md). How to make releases of Rust.

- [The Rust test suite](test-suite.md).

- [Bots, websites and infrastructure](infrastructure.md). A catalog of the IRC
  bots, websites and other infrastructure used by the project, what they do, and
  who maintains them (i.e. who to contact when they malfunction and go on a bot
  rampage).

## Building

Rust Forge uses the [Jekyll](https://jekyllrb.com) static site generator, which can be installed with:

```bash
$ gem install bundler jekyll
```

Next, fetch the list of targets and channels with:

```bash
$ cargo run
```

Then the site can be built with:

```bash
$ bundle exec jekyll build
```

Or it can be hosted locally at http://127.0.0.1:4000 with:

```bash
$ bundle exec jekyll serve
```
