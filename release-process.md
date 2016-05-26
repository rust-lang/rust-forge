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

Move any new targets we added support for in `rust-lang/rust-buildbot`'s
`master.cfg` from beta to stable. This is currently primarily found in the
`*_cross_targets` arrays near the top of the file.

Manually start the `stable-dist-rustc-trigger` builds on
buildbot.rust-lang.org. You'll have around 48 hours to ensure that both these
succeed.

The stable build will not deploy automatically. Precisely, the stable build
will upload docs to the s3 bin under `doc/$version`, but *not* `doc/stable`, and
will not upload the bins anywhere, instead leaving them staged in the
`tmp/dist/packaging-stable/final` subdirectory of the buildbot master.

## Promote master to beta (T-2 days, Tuesday)

Edit the `cargo-revs.txt` file [in
rust-packaging](https://github.com/rust-lang/rust-packaging) to update the Cargo
version for the beta you're about to build.

Promote master to beta as with yesterday:

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/master:beta -f
```

Like yesterday, promote any nightly-only `*_cross_targets` arrays to also be
included on the beta channel.

Send a commit to the freshly created beta branch of rust-lang/rust which updates
src/stage0.txt to bootstrap from this new stable compiler. This should involve
essentially no changes, so you shouldn't need a PR. Be sure to update the
bootstrap key as well, printed out as follows:

```
echo -n 1.9.0 | md5sum | head -c 8
```

Manually start the `beta-dist-rustc-trigger`. The beta will deploy
automatically.

Make and submit a patch against master that bumps the version number in
`mk/main.mk`.

## Prerelease testing (T-1 day, Wednesday)

Write a new blog post and update rust-www. Submit PRs for tomorrow.

Now upload the bins to an s3 staging directory for testing as follows.
This happens on the buildmaster under the rustbuild user.

```sh
cd ~/rust-buildbot/master/tmp/dist/packaging-stable
# Move final staged bins to another directory. This clears out 'final' for the next release.
mv final final-1.3.0
# Upload final to s3 staging for testing
s3cmd del s3://static-rust-lang-org/dist/staging/ --recursive
s3cmd put -P ./final-1.3.0/* s3://static-rust-lang-org/dist/staging/dist/
```

Regenerate the index.

```sh
(cd && sh update-rust-dist-index.sh)
```

Post a message to irlo asking for testing. The index is
http://static-rust-lang-org.s3.amazonaws.com/dist/staging/dist/index.html

Test rustup.sh with

```sh
curl -sSf https://static.rust-lang.org/rustup.sh | RUSTUP_DIST_SERVER=https://static.rust-lang.org/dist/staging sh
```

or multirust.sh with

```sh
RUSTUP_DIST_SERVER=https://static.rust-lang.org/dist/staging multirust update stable
```

or rustup with

```sh
RUSTUP_DIST_ROOT=https://static.rust-lang.org/dist/staging/dist rustup update stable
RUSTUP_DIST_ROOT=https://static.rust-lang.org/dist/staging/dist rustup update 1.8.0
```

Send a PR to the master branch to start bootstrapping from the new beta produced
yesterday.

## Release day (Thursday)

This is on the buildmaster again.

```sh
cd ~/rust-buildbot/master/tmp/dist/packaging-stable
# Sync docs from s3 versioned dir to local
mkdir ./doc-1.3.0
s3cmd sync -P s3://static-rust-lang-org/doc/1.3.0/ ./doc-1.3.0/
# Sync bins to s3 archives
# NOTE! This date is *not* the release date but the date mentioned in the manifest!
s3cmd put -P ./final-1.3.0/* s3://static-rust-lang-org/dist/2015-09-17/
# Sync docs to s3 stable/
# Do this in a screen session in case you lose network access to AWS!
s3cmd sync -P --delete-removed ./doc-1.3.0/ s3://static-rust-lang-org/doc/stable/
# Sync bins to release channel
# Do this in a screen session in case you lose network access to AWS!
s3cmd put -P ./final-1.3.0/* s3://static-rust-lang-org/dist/
# Invalidate the CDN
sh ~/invalidate.sh
```

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

Bask in your success.
