---
layout: default
title: The Rust Forge
---

This site contains supplementary documentation useful to the members
of [The Rust Project](https://www.rust-lang.org). To edit it submit
PRs against [rust-lang/rust-forge].

[rust-lang/rust-forge]: https://github.com/rust-lang/rust-forge

<hr>
<div class="releases">
    <noscript>Please enable JavaScript to see release dates.</noscript>
</div>
<hr>

### Interested in hacking the compiler?

* [The rustc-guide is a book about how rustc works](https://rust-lang.github.io/rustc-guide/)
* [The rustc API docs are hosted here](https://doc.rust-lang.org/nightly/nightly-rustc/rustc/)
* [Profile queries](profile-queries.html). Tips for tracking what the compiler does.
* [Rustc bug fix procedure](rustc-bug-fix-procedure.html): Describes the process for bug fixes that may cause existing code to stop
  compiling.

### Interested in infrastructure and the release process?

* [The current PR testing queue](https://buildbot2.rust-lang.org/homu/queue/rust)
* [The PR queue over time](https://rust-lang-nursery.github.io/rustc-pr-tracking/)
* [Toolstate (rls, rustfmt, clippy)](https://rust-lang-nursery.github.io/rust-toolstate/)
* [Release history](releases.html). Links to previous release artifacts.
* [Platform support](platform-support.html).
* [Release process](release-process.html). How to make releases of Rust.
* [How to prepare Rust release notes](release-notes.html).
* [Beta backporting](beta-backporting.html). The mystery of the oft-misinterpreted `beta-nominated` / `beta-accepted` tags finally revealed.
* [Bots, websites and infrastructure](infrastructure.html). A catalog of the IRC bots, websites and other infrastructure used by the project, what they do, and who maintains them (i.e. who to contact when they malfunction and go on a bot rampage).
* [Homu/Bors Syntax](https://buildbot2.rust-lang.org/homu/)
* [State Of Rust](state-of-rust.html). Links and information about the current status of unstable features.
* [`rustup-toolchain-install-master`](https://github.com/kennytm/rustup-toolchain-install-master) allows installing Rust from `master` before it even makes it to nightly.

### Meta-processes: managing the RFC repo, teams, etc

* [RFC merge procedure](rfc-merge-procedure.html)
* [Team maintenance](rustc-team-maintenance.html)
* [Issue & PR Triage Procedure](triage-procedure.html)

### Other things

* [Friend of the Tree archives](fott.html).
* [Bibliography](bibliography.html). Research papers and other links to projects that influenced Rust. Papers about Rust.
* [Cross compilation resources](cross-compilation/index.html)
* [Other Rust Installation Methods](other-installation-methods.html)

<script>

function addRelease(kind, num, when) {
    var out = "";
    out += '<div class="release">';
    out += '<div class="release-kind">Current ' + kind + '</div>';
    out += '<div class="release-number">' + num + '</div>';
    out += '<div class="release-date">' + when.toDateString() + '</div>';
    out += '</div>';
    document.querySelector(".releases").innerHTML += out;
}

document.addEventListener("DOMContentLoaded", function() {

  // rust 1.5's release date
  var epochDate = new Date('2015-12-11');
  // #nevertwopointoh -- we render "1." in the string literals below, this is easier to increment
  var epochRelease = 5;
  // there are 6 weeks in between releases
  var releaseDuration = 7 * 6 * 86400 * 1000;

  var today = new Date();
  var releases = (today - epochDate) / releaseDuration | 0;

  var prevDate = new Date(epochDate.getTime() + releases * releaseDuration);
  var prevRelease = epochRelease + releases;

  var nextDate = new Date(prevDate.getTime() + releaseDuration);
  var nextRelease = prevRelease + 1;

  var nextNextDate = new Date(nextDate.getTime() + releaseDuration);
  var nextNextRelease = nextRelease + 1;

  addRelease("stable", "1." + prevRelease, prevDate);
  addRelease("beta", "1." + nextRelease, nextDate);
  addRelease("nightly", "1." + nextNextRelease, nextNextDate);
});
</script>
