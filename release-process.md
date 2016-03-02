---
layout: default
title: The Rust Release Process &middot; The Rust Programming Language
---

# The Rust Release Process

The Rust release process is mostly in my head right now, so I took notes on how it went this time (1.3.0).

## Channel promotion and new beta/stable builds (T-2 days)

Tag a new Cargo release that will be included with the new beta build (same tagging instructions as for Rust below). Then edit the `cargo-revs.txt` file [in rust-packaging](https://github.com/rust-lang/rust-packaging) to update the Cargo version for the beta you're about to build.

Promote beta to stable, then nightly to beta as follows.
This is local, in your Rust repo, assuming you have push access to rust-lang. 

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/beta:stable -f
$ git push rust-lang rust-lang/master:beta -f
```

Manually start the `beta-dist-rustc-trigger` and `stable-dist-rustc-trigger` builds on buildbot.rust-lang.org.
You've got around 48 hours to ensure that both these succeed.

The beta will deploy automatically, the stable not. Precisely, the stable build will upload docs to
the s3 bin under `doc/$version`, but *not* `doc/stable`, and will not upload the bins anywhere,
instead leaving them staged in the `tmp/dist/packaging-stable/final` subdirectory of the buildbot master.

Make and submit a patch against master that bumps the version number in `mk/main.mk`.

## Prerelease testing (T-1 day)

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

Post a message to irlo asking for testing. The index is http://static-rust-lang-org.s3.amazonaws.com/dist/staging/dist/index.html

Test rustup.sh with

```sh
curl -sSf https://static.rust-lang.org/rustup.sh | RUSTUP_DIST_SERVER=https://static.rust-lang.org/dist/staging sh
```

or multirust.sh with

```sh
RUSTUP_DIST_SERVER=https://static.rust-lang.org/dist/staging multirust update stable
```

or multirust-rs with

```sh
MULTIRUST_DIST_ROOT=https://static.rust-lang.org/dist/staging/dist multirust update stable
```

## Release day

This is on the buildmaster again.

```sh
cd ~/rust-buildbot/master/tmp/dist/packaging-stable
# Sync docs from s3 versioned dir to local
mkdir ./doc-1.3.0
s3cmd sync -P s3://static-rust-lang-org/doc/1.3.0/ ./doc-1.3.0/
# Sync bins to s3 archives
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

Locally, tag the new release and upload it. Use "x.y.z release" as the commit message.

```sh
$ git tag -u FA1BE5FE 1.3.0 $COMMIT_SHA
$ git push rust-lang 1.3.0
```

Run `cargo publish` for the Cargo tag created in the first step.

Bask in your success.
