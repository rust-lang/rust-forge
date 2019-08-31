# The Rust Release Process

Here's how Rust is currently released:

## Promote beta to stable (T-3 days, Monday)

Promote beta to stable. Temporarily turn off GitHub branch protection for the
`stable` branch in rust-lang/rust repo. In your local Rust repo:

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/beta:stable -f
```

Re-enable branch protection for the `stable` branch. Send a PR to rust-lang/rust
on the stable branch making the following changes:

- Update `src/ci/run.sh` to pass `channel=stable`, not `channel=beta`.

Once the PR is sent, r+ it and give it a high `p=1000`.

The stable build will **not** deploy automatically to prod. The
rust-central-station repository is configured to upload to **dev** every hour if
it detects a change. You should be able to browse changes in dev.

As soon as this build is done post a message to irlo asking for testing. The
index is
https://dev-static-rust-lang-org.s3.amazonaws.com/dist/2015-09-17/index.html and
our URL is then https://dev-static.rust-lang.org/dist/2015-09-17/index.html.

Test rustup with

```sh
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup update stable
```

If something goes wrong, and we rebuild stable artifacts, you'll need to
invalidate the dev-static bucket for RCS to re-release it.

1.  Download https://dev-static.rust-lang/dist/channel-rust-1.35.0.toml The
    version number must be less than the current release, but otherwise doesn't
    matter.
1.  Rename the file locally to channel-rust-stable.toml
1.  Upload the file to the dev-static bucket into the dist folder, replacing
    channel-rust-stable.toml.
1.  Go to CloudFront in AWS, to the dev-static bucket, and issue an invalidation
    for "/dist/channel-rust-stable.toml". This is necessary until
    https://github.com/rust-lang/rust-central-station/issues/49 is fixed.
1.  (optional) login to central station, and run the following. This starts the
    dev-static promotion immediately, vs. waiting till the next hour.

```bash
docker exec -d -it rcs bash -c 'promote-release /tmp/stable stable /data/secrets-dev.toml 2>&1 | logger --tag release-stable'
```

## Promote master to beta (T-2 days, Tuesday)

Create a new branch on `rust-lang/cargo` for the new beta. Here, `rust-lang` is
the remote for https://github.com/rust-lang/rust.git. Replace `YY` with the
minor version of master. First determine the branch point for cargo in
`rust-lang/rust`, and then create a new branch:

```sh
$ cd rust
$ git fetch rust-lang
$ CARGO_SHA=`git rev-parse rust-lang/master:src/tools/cargo`
$ cd src/tools/cargo
$ git branch rust-1.YY.0 $CARGO_SHA
$ git push origin rust-1.YY.0
```

You'll need to temporarily disable branch protection on GitHub to push the new
branch.

In theory one day we'll do the same for rust-lang/rls, but for now we haven't
done this yet.

Temporarily disable banch protection on GitHub for the `beta` branch of the Rust
repo. Promote rust-lang/rust's master branch to beta as with yesterday:

```sh
$ git fetch rust-lang
$ git push rust-lang rust-lang/master:beta -f
```

Re-enable branch protection on GitHub. Send a PR to the freshly created beta
branch of rust-lang/rust which:

- Update src/stage0.txt
  - Change `date` to "YYYY-MM-DD" where the date is the archive date the stable
    build was uploaded
  - Change `rustc` to "X.Y.Z" where that's the version of rustc you just build
  - Change `cargo` to "A.B.C" where it's Cargo's version. That's typically
    "0.(Y+1).0" wrt the rustc version.
  - Uncomment `dev: 1`
- Update src/ci/run.sh to pass "--release-channel=beta".

Note that you probably don't want to update the RLS if it's working, but if it's
not working beta won't land and it'll need to get updated. After this PR merges
(through @bors) the beta should be automatically released.

# Master bootstrap update (T-1 day, Wednesday)

Write a new blog post, update rust-www, and update rust-forge. Submit PRs for
tomorrow.

Send a PR to the master branch to:

- modify src/stage0.txt to bootstrap from yesterday's beta
- modify src/bootstrap/channel.rs with the new version number

## Release day (Thursday)

Decide on a time to do the release, T.

- **T-30m** - This is on rust-central-station:

  ```
  docker exec -d -it rcs bash -c 'promote-release /tmp/stable stable /data/secrets.toml 2>&1 | logger --tag release-stable-realz'
  ```

  That'll, in the background, schedule the `promote-release` binary to run on
  the production secrets (not the dev secrets). That'll sign everything, upload
  it, update the html index pages, and invalidate the CDN. Note that this takes
  about 30 minutes right now. Logs are in `/opt/rcs/logs`.

- **T-10m** - Locally, tag the new release and upload it. Use "x.y.z release" as
  the commit message.

  ```sh
  $ git tag -u FA1BE5FE 1.3.0 $COMMIT_SHA
  $ git push rust-lang 1.3.0
  ```

  After this [Update thanks.rust-lang.org][update-thanks] by triggering a build
  on Travis.

- **T-5m** - Merge blog post.

- **T** - Tweet and post everything!

  - Twitter [@rustlang](https://twitter.com/rustlang)
  - Reddit [/r/rust](https://www.reddit.com/r/rust/)
  - [Hacker News](https://news.ycombinator.com/)
  - [Users forum](https://users.rust-lang.org/)

- **T+5m** - Tag Cargo the same way as rust-lang/rust and then run
  `cargo publish` for the tag you just created. You'll first need to comment
  out `cargo-test-macro` from Cargo.toml, then publish `crates-io` (in
  `crates/crates-io`) and finally publish `cargo` itself.

  Also tag RLS the same way and then run `cargo publish` for the tag you just
  created.

- **T+1hr** Send a PR to the beta branch to comment out `dev: 1` again and
  update the date to download from (modifying `src/stage0.txt`).

[update-thanks]: https://travis-ci.com/rust-lang/thanks

Bask in your success.

# Update dependencies (T+1 day, Friday)

In the repo:

```bash
$ cd src
$ cargo update
```

The very ambitious can use https://crates.io/crates/cargo-outdated and update
through breaking changes.

