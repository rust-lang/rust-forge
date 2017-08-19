---
layout: default
title: The Rust Forge
---

This site contains supplementary documentation useful to the members
of [The Rust Project](https://www.rust-lang.org). To edit it submit
PRs against [rust-lang-nursery/rust-forge].

[rust-lang-nursery/rust-forge]: https://github.com/rust-lang-nursery/rust-forge

<div id="release_info"></div>

<hr/>

### Pages

* [Building rustc with x.py](x-py.html).

* [Release history](releases.html). Links to previous release
  artifacts.

* [Platform support](platform-support.html).

* [Friend of the Tree archives](fott.html).

* [IDE initiative](ides.html). Ongoing work into improving IDE support
  for Rust.

* [Bibliography](bibliography.html). Research papers and other links
  to projects that influenced Rust. Papers about Rust.

* [Release process](release-process.html). How to make releases of
  Rust.

* [How to prepare Rust release notes](release-notes.html).

* [Beta backporting](beta-backporting.html). The mystery of the
  oft-misinterpreted `beta-nominated` / `beta-accepted` tags finally
  revealed.

* [The Rust test suite](test-suite.html).

* [Rustc bug fix procedure](rustc-bug-fix-procedure.html): Describes the
  process for bug fixes that may cause existing code to stop
  compiling.

* [So you want to stabilize a feature?](stabilization-guide.html): Describes
  the procedure for stabilizing features in rustc.

* [Bots, websites and infrastructure](infrastructure.html). A catalog
  of the IRC bots, websites and other infrastructure used by the
  project, what they do, and who maintains them (i.e. who to contact
  when they malfunction and go on a bot rampage).

* [Cross compilation resources](cross-compilation/index.html)

* [RFC merge procedure](rfc-merge-procedure.html)

* [Team maintenance](rustc-team-maintenance.html)

* [PR Triage Procedure](pr-triage-procedure.html)

* [Homu/Bors Syntax](https://buildbot2.rust-lang.org/homu/)

<script>

document.addEventListener("DOMContentLoaded", function(event) {

  var previousDate = new Date('2015-12-11');
  var nextDate = new Date('2016-01-22');
  var nextNextDate = new Date('2016-03-04');

  var prevRelease = 5;
  var nextRelease = 6;
  var nextNextRelease = 7;

  while (Date.now() > nextDate) {
    previousDate = new Date(nextDate);
    nextDate.setDate(nextDate.getDate() + (7 * 6));
    nextNextDate.setDate(nextNextDate.getDate() + (7 * 6));

    prevRelease += 1;
    nextRelease += 1;
    nextNextRelease += 1;
  }

  previousDate = previousDate.toDateString();
  nextDate = nextDate.toDateString();
  nextNextDate = nextNextDate.toDateString();

  var toWrite = "<hr/><h3>Release Dates</h3>";

  toWrite += "<p>Rust 1." + prevRelease + " stable was released on " + previousDate + ".</p>";
  toWrite += "<p><h4>Rust 1." + nextRelease + " stable will release on " + nextDate + ".</h4></p>";
  toWrite += "<p>Rust 1." + nextNextRelease + " stable will release on " + nextNextDate + ".</p>";

  document.getElementById('release_info').innerHTML = toWrite;
});
</script>
