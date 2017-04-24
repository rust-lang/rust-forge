---
layout: default
title: The Rust Release Process &middot; The Rust Programming Language
---

# The Rust Release Process

The Rust release process is mostly in my head right now, so I took notes on how
it went this time (1.3.0).


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

* Update src/stage0.txt to bootstrap from the freshly minted stable
  compiler. The format is "X.Y.Z-YYYY-MM-DD", and the date is the
  archive date the stable build was uploaded.  Note that you'll need to update
  `src/bootstrap/bootstrap.py` to pull from `dev-static.rust-lang.org` instead
  of `static.rust-lang.org` as the stable release isn't published yet. Later
  you'll send a PR to bootstrap from the `static.rust-lang.org` location after
  the release.
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

This is on rust-central-station:

```
docker exec -d -it `docker ps -l -q` bash -c \
  'promote-release /tmp/stable stable /src/data/secrets.toml 2>&1 | logger --tag release-stable-realz'
```

That'll, in the background, schedule the `promote-release` binary to run on the
production secrets (not the dev secrets). That'll sign everything, upload it,
update the html index pages, and invalidate the CDN. Note that this takes about
30 minutes right now.

Next merge the website. It takes a while to deploy.

Merge blog post.

Locally, tag the new release and upload it. Use "x.y.z release" as the commit
message.

```sh
$ git tag -u FA1BE5FE 1.3.0 $COMMIT_SHA
$ git push rust-lang 1.3.0
```

Also tag Cargo the same way and then run `cargo publish` for the tag you just
created.

Also tag RLS the same way and then run `cargo publish` for the tag you just
created.

Bask in your success.
