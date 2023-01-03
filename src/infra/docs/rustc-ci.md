# How the Rust CI works

Rust CI ensures that the master branch of rust-lang/rust is always in a valid state.

A developer submitting a pull request to rust-lang/rust, experiences the following:

- A small subset of tests and checks are run on each commit to catch common errors.
- When the PR is ready and approved, the "bors" tool enqueues a full CI run.
- The full run is either performed or the PR is "rolled up" with other changes.
- Eventually a CI run containing the changes from the PR is performed and either passes or fails with an error the developer must address.

## Which jobs we run

The `rust-lang/rust` repository uses GitHub Actions to test [all the
platforms][platforms] we support. We currently have two kinds of jobs running
for each commit we want to merge to master:

- Dist jobs build a full release of the compiler for that platform, including
  all the tools we ship through rustup; Those builds are then uploaded to the
  `rust-lang-ci2` S3 bucket and are available to be locally installed with the
  [rustup-toolchain-install-master] tool; The same builds are also used for
  actual releases: our release process basically consists of copying those
  artifacts from `rust-lang-ci2` to the production endpoint and signing them.
- Non-dist jobs run our full test suite on the platform, and the test suite of
  all the tools we ship through rustup; The amount of stuff we test depends on
  the platform (for example some tests are run only on Tier 1 platforms), and
  some quicker platforms are grouped together on the same builder to avoid
  wasting CI resources.

All the builds except those on macOS and Windows are executed inside that
platform’s custom [Docker container]. This has a lot of advantages for us:

- The build environment is consistent regardless of the changes of the
  underlying image (switching from the trusty image to xenial was painless for
  us).
- We can use ancient build environments to ensure maximum binary compatibility,
  for example [using older CentOS releases][dist-x86_64-linux] on our Linux builders.
- We can avoid reinstalling tools (like QEMU or the Android emulator) every
  time thanks to Docker image caching.
- Users can run the same tests in the same environment locally by just running
  `src/ci/docker/run.sh image-name`, which is awesome to debug failures.
- Forces the use of portable scripts to drive the CI process which keeps the CI fairly platform independent (i.e., we are not overly reliant on GitHub Actions).

The docker images prefixed with `dist-` are used for building artifacts while those without that prefix run tests and checks.

We also run tests for less common architectures (mainly Tier 2 and Tier 3
platforms) in CI. Since those platforms are not x86 we either run
everything inside QEMU or just cross-compile if we don’t want to run the tests
for that platform.

These builders are running on a special pool of builders set up and maintained for us by GitHub.

[platforms]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[rustup-toolchain-install-master]: https://github.com/kennytm/rustup-toolchain-install-master
[Docker container]: https://github.com/rust-lang/rust/tree/master/src/ci/docker
[dist-x86_64-linux]: https://github.com/rust-lang/rust/blob/master/src/ci/docker/host-x86_64/dist-x86_64-linux/Dockerfile

## Merging PRs serially with bors

CI services usually test the last commit of a branch merged with the last
commit in master, and while that’s great to check if the feature works in
isolation it doesn’t provide any guarantee the code is going to work once it’s
merged. Breakages like these usually happen when another, incompatible PR is
merged after the build happened.

To ensure a master that works all the time we forbid manual merges: instead all
PRs have to be approved through our bot, [bors] (the software behind it is
called [homu]). All the approved PRs are put [in a queue][homu-queue] (sorted
by priority and creation date) and are automatically tested one at the time. If
all the builders are green the PR is merged, otherwise the failure is recorded
and the PR will have to be re-approved again.

Bors doesn’t interact with CI services directly, but it works by pushing the
merge commit it wants to test to a branch called `auto`, and detecting the
outcome of the build by listening for either Commit Statuses or Check Runs.
Since the merge commit is based on the latest master and only one can be tested
at the same time, when the results are green master is fast-forwarded to that
merge commit.

The `auto` branch and other branches used by bors live on a fork of the rust-lang/rust: [rust-lang-ci/rust]. This as originally done to some security limitations in GitHub Actions. These limitations have been addressed, but we've not yet done the work of removing the use of a fork.

Unfortunately testing a single PR at the time, combined with our long CI (~3
hours for a full run)[^1], means we can’t merge too many PRs in a single day, and a
single failure greatly impacts our throughput for the day. The maximum number
of PRs we can merge in a day is around 8.

The large CI run times and requirement for a large builder pool is worth it because it: 

- allows perf testing even at a later date 
- allows bisection when bugs are discover later
- ensures release quality since if we're always releasing, we catch problems very early

Bors [runs on ecs](https://github.com/rust-lang/simpleinfra/blob/master/terraform/bors/app.tf) and uses a sqlite database running in a volume as storage.

[^1]: As of December 2022, the bottleneck are the macOS builders. Hopefully faster macOS builders will be coming soon!

[bors]: https://github.com/bors
[homu]: https://github.com/rust-lang/homu
[homu-queue]: https://bors.rust-lang.org/queue/rust
[rust-lang-ci/rust]: https://github.com/rust-lang-ci/rust

### Rollups

Some PRs don’t need the full test suite to be executed: trivial changes like
typo fixes or README improvements *shouldn’t* break the build, and testing
every single one of them for 2 to 3 hours is a big waste of time. To solve this
we do a "rollup", a PR where we merge all the trivial PRs so they can be tested
together. Rollups are created manually by a team member using the "create a rollup" button on the [bors queue]. The team member uses their
judgment to decide if a PR is risky or not, and are the best tool we have at
the moment to keep the queue in a manageable state.

[bors queue]: https://bors.rust-lang.org/queue/rust

### Try builds

Sometimes we need a working compiler build before approving a PR, usually for
[benchmarking][perf] or [checking the impact of the PR across the
ecosystem][crater]. Bors supports creating them by pushing the merge commit on
a separate branch (`try`), and they basically work the same as normal builds,
without the actual merge at the end. Any number of try builds can happen at the
same time, even if there is a normal PR in progress.

You can see the CI configuration for try builds [here](https://github.com/rust-lang/rust/blob/9d46c7a3e69966782e163877151c1f0cea8b630a/src/ci/github-actions/ci.yml#L728-L741).

[perf]: https://perf.rust-lang.org
[crater]: https://github.com/rust-lang/crater

## Which branches we test

Our builders are defined in [`src/ci/github-actions/ci.yml`].

[`src/ci/github-actions/ci.yml`]: https://github.com/rust-lang/rust/blob/master/src/ci/github-actions/ci.yml

### PR builds

All the commits pushed in a PR run a limited set of tests: a job containing a
bunch of lints plus a cross-compile check build to Windows mingw (without
producing any artifacts) and the `x86_64-gnu-llvm-##` non-dist builder (where
`##` is the *system* LLVM version we are currently testing). Those two
builders are enough to catch most of the common errors introduced in a PR, but
they don’t cover other platforms at all. Unfortunately it would take too many
resources to run the full test suite for each commit on every PR.

Additionally, if the PR changes certain tools (or certain platform-specific
parts of std to check for miri breakage), the `x86_64-gnu-tools` non-dist
builder is run.

### The `try` branch

On the main rust repo, `try` builds produce just a Linux toolchain using the
`dist-x86_64-linux` image.

### The `auto` branch

This branch is used by bors to run all the tests on a PR before merging it, so
all the builders are enabled for it. bors will repeatedly force-push on it
(every time a new commit is tested).

### The `master` branch

Since all the commits to `master` are fast-forwarded from the `auto` branch (if
they pass all the tests there) we don’t need to build or test anything. A quick
job is executed on each push to update toolstate (see the toolstate description
below).

### Other branches

Other branches are just disabled and don’t run any kind of builds, since all
the in-progress branches will eventually be tested in a PR.

## Caching

The main rust repository doesn’t use the native GitHub Actions caching tools.
All our caching is uploaded to an S3 bucket we control
(`rust-lang-ci-sccache2`), and it’s used mainly for two things:

### Docker images caching

The Docker images we use to run most of the Linux-based builders take a *long*
time to fully build. To speed up the build, we cache the exported images on the
S3 bucket (with `docker save`/`docker load`).

Since we test multiple, diverged branches (`master`, `beta` and `stable`) we
can’t rely on a single cache for the images, otherwise builds on a branch would
override the cache for the others. Instead we store the images identifying them
with a custom hash, made from the host’s Docker version and the contents of all
the Dockerfiles and related scripts.

### LLVM caching with sccache

We build some C/C++ stuff during the build and we rely on [sccache] to cache
intermediate LLVM artifacts. Sccache is a distributed ccache developed by
Mozilla, and it can use an object storage bucket as the storage backend, like
we do with our S3 bucket.

[sccache]: https://github.com/mozilla/sccache

## Custom tooling around CI

During the years we developed some custom tooling to improve our CI experience.

### Rust Log Analyzer to show the error message in PRs

The build logs for `rust-lang/rust` are huge, and it’s not practical to find
what caused the build to fail by looking at the logs. To improve the
developers’ experience we developed a bot called [Rust Log Analyzer][rla] (RLA)
that receives the build logs on failure and extracts the error message
automatically, posting it on the PR.

The bot is not hardcoded to look for error strings, but was trained with a
bunch of build failures to recognize which lines are common between builds and
which are not. While the generated snippets can be weird sometimes, the bot is
pretty good at identifying the relevant lines even if it’s an error we've never
seen before.

[rla]: https://github.com/rust-lang/rust-log-analyzer

### Toolstate to support allowed failures

The `rust-lang/rust` repo doesn’t only test the compiler on its CI, but also a
variety of tools and documentation. Some documentation is pulled in via git
submodules. If we blocked merging rustc PRs on the documentation being fixed,
we would be stuck in a chicken-and-egg problem, because the documentation's CI
would not pass since updating it would need the not-yet-merged version of
rustc to test against (and we usually require CI to be passing).

To avoid the problem, submodules are allowed to fail, and their status is
recorded in [rust-toolstate]. When a submodule breaks, a bot automatically
pings the maintainers so they know about the breakage, and it records the
failure on the toolstate repository. The release process will then ignore
broken tools on nightly, removing them from the shipped nightlies.

While tool failures are allowed most of the time, they’re automatically
forbidden a week before a release: we don’t care if tools are broken on nightly
but they must work on beta and stable, so they also need to work on nightly a
few days before we promote nightly to beta.

More information is available in the [toolstate documentation].

### GitHub Actions Templating

GitHub Actions does not natively support templating which can cause configurations to be large and difficult to change. We use YAML anchors for templating and a custom tool, [`expand-yaml-anchors`], to expand [the template] into the CI configuration that [GitHub uses][ci config].

This templating language is fairly straightforward:

- `&` indicates a template section
- `*` expands the indicated template in place
- `<<` merges yaml dictionaries

[rust-toolstate]: https://rust-lang-nursery.github.io/rust-toolstate
[toolstate documentation]: ../toolstate.md
[`expand-yaml-anchors`]: https://github.com/rust-lang/rust/tree/master/src/tools/expand-yaml-anchors
[the template]: https://github.com/rust-lang/rust/blob/736c675d2ab65bcde6554e1b73340c2dbc27c85a/src/ci/github-actions/ci.yml
[ci config]: https://github.com/rust-lang/rust/blob/master/.github/workflows/ci.yml