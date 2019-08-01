---
layout: default
title: The Rust Forge
---

This site contains supplementary documentation useful to the members of
[The Rust Project](https://www.rust-lang.org). To edit it submit PRs against
[rust-lang/rust-forge].

[rust-lang/rust-forge]: https://github.com/rust-lang/rust-forge

<hr>
<noscript>Please enable JavaScript to see release dates.</noscript>
<div class="releases"></div>
<div class="tools-no-breakages-header hidden">
    <hr>
    <h2>No tool breakages week</h2>
    <p>
        To ensure the beta release includes all the tools, no
        <a href="toolstate.html">tool breakages</a>
        are allowed in the week before the beta cutoff
        (except for nightly-only tools).
    </p>
</div>
<div class="tools-no-breakages"></div>
<hr>

### Interested in hacking the compiler?

- [The rustc-guide is a book about how rustc works](https://rust-lang.github.io/rustc-guide/)
- [The rustc API docs are hosted here](https://doc.rust-lang.org/nightly/nightly-rustc/rustc/)
- [Profile queries](profile-queries.html). Tips for tracking what the compiler
  does.
- [Rustc bug fix procedure](rustc-bug-fix-procedure.html): Describes the process
  for bug fixes that may cause existing code to stop compiling.
- [The list of FIXME comments in the compiler](https://oli-obk.github.io/fixmeh/)
  can be helpful to find cleanups that still need to be done

### Interested in infrastructure and the release process?

- [The current PR testing queue](https://buildbot2.rust-lang.org/homu/queue/rust)
- [The PR queue over time](https://rust-lang-nursery.github.io/rustc-pr-tracking/)
- [Current toolstate (rls, rustfmt, clippy)](https://rust-lang-nursery.github.io/rust-toolstate/)
  and [how the toolstate system works](toolstate.html).
- [Components availability history](https://rust-lang.github.io/rustup-components-history/index.html)
- [Release history](releases.html). Links to previous release artifacts.
- [Platform support](platform-support.html).
- [Release process](release-process.html). How to make releases of Rust.
- [How to prepare Rust release notes](release-notes.html).
- [How the channels look on `static.rust-lang.org`](channel-layout.html).
- [Beta backporting](beta-backporting.html). The mystery of the
  oft-misinterpreted `beta-nominated` / `beta-accepted` tags finally revealed.
- [Bots, websites and infrastructure](infrastructure.html). A catalog of the
  bots, websites and other infrastructure used by the project, what they do, and
  who maintains them (i.e. who to contact when they malfunction and go on a bot
  rampage).
- [Homu/Bors Syntax](https://buildbot2.rust-lang.org/homu/)
- [State Of Rust](https://github.com/rust-lang/rust/projects/8). A GitHub
  project board documenting the current status of unstable features.
- [`rustup-toolchain-install-master`](https://github.com/kennytm/rustup-toolchain-install-master)
  allows installing Rust from `master` before it even makes it to nightly.

### Meta-processes: managing the RFC repo, teams, etc

- [RFC merge procedure](rfc-merge-procedure.html)
- [Team maintenance](rustc-team-maintenance.html)
- [Issue & PR Triage Procedure](triage-procedure.html)

### Other things

- [Friend of the Tree archives](fott.html).
- [Bibliography](https://rust-lang.github.io/rustc-guide/appendix/bibliography.html).
  Research papers and other links to projects that influenced Rust. Papers about
  Rust.
- [Cross compilation resources](cross-compilation/index.html)
- [Other Rust Installation Methods](other-installation-methods.html)
- [Rust Pontoon](https://pontoon.rust-lang.org/). Translating the Rust website.

<script src="js/moment.min.js"></script>
<script src="js/index.js"></script>
