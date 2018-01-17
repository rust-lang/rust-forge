---
layout: default
title: The Rust Release Process &middot; The Rust Programming Language
---

# The Rust Release Process

Here's how Rust is currently released:

## Update books (T-7 days, Friday)

From within the repo:

```bash
$ cd src/doc/book
$ git fetch origin
$ git reset --hard origin/master
$ cd ../rust-by-example
$ git fetch origin
$ git reset --hard origin/master
$ cd ../reference
$ git fetch origin
$ git reset --hard origin/master
$ cd ../nomicon
$ git fetch origin
$ git reset --hard origin/master
$ cd ../../..
$ git add .
$ git commit -m "update books for next release"
```

This can be done a day earlier too, if you'd like: the books are only tested fully once they're in
this repo, and so updating them may cause tests to fail. If they do, you need to send in a PR to
the right book, ping @steveklabnik to get a quick merge, and then update that particular book again
to fix it. This should be relatively rare.

## Promote beta to stable (T-3 days, Monday)

Promote beta to stable.  This is local, in your Rust repo, assuming you have
push access to rust-lang.

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/beta:stable -f
```

Send a PR to rust-lang/rust on the stable branch making the following changes:

* Update the cargo submodule to the latest of the branch for this rustc version.
  Cargo branches are named `rust-1.17.0` and such.
* Update the rls submodule to the latest of the branch for this rustc version
  (as with Cargo)
* Update `src/ci/run.sh` to pass `channel=stable`, not `channel=beta`.

Once the PR is sent, r+ it and give it a high `p=10000`.

The stable build will **not** deploy automatically to prod. The
rust-central-station repository is configured to upload to **dev** every hour if
it detects a change. You should be able to browse changes in dev.

## Prerelease testing (T-2 days, Tuesday)

Post a message to irlo asking for testing. The index is
https://dev-static-rust-lang-org.s3.amazonaws.com/dist/2015-09-17/index.html and
our URL is then https://dev-static.rust-lang.org/dist/2015-09-17/index.html.

Test rustup with

```sh
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup update stable
```

## Promote master to beta (T-2 days, Tuesday)

Branch a rust-lang/cargo commit for the new beta:

```sh
$ cd cargo
$ git fetch rust-lang
$ git push rust-lang rust-lang/master:rust-1.14.0
```

Do the same for rust-lang-nursery/rls

Promote rust-lang/rust's master branch to beta as with yesterday:

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/master:beta -f
```

Send a PR to the freshly created beta branch of rust-lang/rust
which:

* Update src/stage0.txt
  * Change `date` to "YYYY-MM-DD" where the date is the archive date the stable
    build was uploaded
  * Change `rustc` to "X.Y.Z" where that's the version of rustc you just build
  * Change `cargo` to "A.B.C" where it's Cargo's version. That's typically
    "0.(Y+1).0" wrt the rustc version.
  * Uncomment `dev: 1`
* Update src/ci/run.sh to pass "--release-channel=beta".
* Update the cargo submodule to the head of the versioned branch
* Update the rls submodule to the head of the versioned branch

After this PR merges (through @bors) the beta should be automatically released.

# Master bootstrap update (T-1 day, Wednesday)

Write a new blog post and update rust-www. Submit PRs for tomorrow.

Send a PR to the master branch to:

* modify src/stage0.txt to bootstrap from yesterday's beta
* modify src/bootstrap/channel.rs with the new version number
* Update the cargo submodule to the master branch
* Update the rls submodule to the master branch

## Release day (Thursday)

Decide on a time to do the release, T.

* **T-30m** - This is on rust-central-station:

  ```
  docker exec -d -it `docker ps -l -q` bash -c \
    'promote-release /tmp/stable stable /data/secrets.toml 2>&1 | logger --tag release-stable-realz'
  ```

  That'll, in the background, schedule the `promote-release` binary to run on the
  production secrets (not the dev secrets). That'll sign everything, upload it,
  update the html index pages, and invalidate the CDN. Note that this takes about
  30 minutes right now. Logs are in `/opt/rcs/logs`.

* **T-20m** - Merge the website. Travis may have a big backlog, cancel
  rust-lang/rust PR builds or other builds until this build is scheduled.
  This'll then involve a CloudFront invalidation that takes awhile.

* **T-10m** - Locally, tag the new release and upload it. Use "x.y.z release" as
  the commit message.

  ```sh
  $ git tag -u FA1BE5FE 1.3.0 $COMMIT_SHA
  $ git push rust-lang 1.3.0
  ```
  After this [Update thanks.rust-lang.org][update-thanks].

* **T-5m** - Merge blog post.

* **T** - Tweet and post everything!

* **T+5m** - Tag Cargo the same way as rust-lang/rust and then run `cargo
  publish` for the tag you just created.

  Also tag RLS the same way and then run `cargo publish` for the tag you just
  created.

* **T+1hr** Send a PR to the beta branch to comment out `dev: 1` again and
  update the date to download from (modifying `src/stage0.txt`).

[update-thanks]: https://github.com/rust-lang-nursery/thanks#thanks

Bask in your success.

# Update dependencies (T+1 day, Friday)

In the repo:

```bash
$ cd src
$ cargo update
```

The very ambitious can use https://crates.io/crates/cargo-outdated and update through breaking changes.
