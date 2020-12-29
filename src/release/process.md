# The Rust Release Process

Here's how Rust is currently released:

## Bump the stable version number (T-6 days, Friday the week before)

Open a PR bumping the version number in `src/version`. `r+ rollup=never` this
PR.

Mark it as `rollup=never`, because if it lands in a rollup as *not* the first
PR then other pull requests in that rollup will be incorrectly associated with
the prior release.

## Promote beta to stable (T-3 days, Monday)

Temporarily turn off GitHub branch protection for the `stable` branch in
rust-lang/rust repo. In your local Rust repo:

```sh
$ git fetch origin
$ git push origin origin/beta:stable -f
# make sure that the release notes file is as fresh as possible
$ git checkout origin/master -- RELEASES.md
```

Re-enable branch protection for the `stable` branch. Send a PR to rust-lang/rust
on the stable branch making the following changes:

- Update `src/ci/run.sh` to pass `channel=stable`, not `channel=beta`.

Once the PR is sent, r+ it and give it a high `p=1000`.

After the PR is merged you'll need to start a **dev** release. [Obtain AWS CLI
credentials][awscli] and run this command from the [simpleinfra] repository:

```
./start-release.py dev stable
```

As soon as this build is done create a blog post on Inside Rust asking for
testing. The index is
https://dev-static.rust-lang.org/dist/YYYY-MM-DD/index.html.

Test rustup with:

```sh
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup update stable
```

## Promote master to beta (T-2 days, Tuesday)

We need to find out the parent commit in which the PR opened last Monday merged.

Go to that PR, and find the "bors merged commit $SHA into rust-lang:master at the bottom.

Locally, run `` export BRANCH_POINT=`git rev-parse $SHA^` `` in the rust-lang/rust
checkout. This should be bors-authored merge into master of the PR before the
version bump merged.

Create a new branch on `rust-lang/cargo` for the new beta. Here, `rust-lang` is
the remote for https://github.com/rust-lang/rust.git. Replace `YY` with the
minor version of master. First determine the branch point for cargo in
`rust-lang/rust`, and then create a new branch:

```sh
cd rust
git fetch rust-lang
NEW_BETA_VERSION=`git show $BRANCH_POINT:src/version`
CARGO_SHA=`git rev-parse $BRANCH_POINT:src/tools/cargo`
cd src/tools/cargo
git branch rust-$NEW_BETA_VERSION $CARGO_SHA
git push origin rust-$NEW_BETA_VERSION
```

You'll need to temporarily disable branch protection on GitHub to push the new
branch.

Temporarily disable banch protection on GitHub for the `beta` branch of the Rust
repo. Promote rust-lang/rust's master branch to beta as with yesterday:

```sh
$ git fetch rust-lang
$ git push rust-lang $BRANCH_POINT:beta -f
```

Re-enable branch protection on GitHub. Send a PR to the freshly created beta
branch of rust-lang/rust which:

- Update src/stage0.txt
  - Change `date` to "YYYY-MM-DD" where the date is the archive date the stable
    build was uploaded
  - Change `rustc` to "X.Y.Z" where that's the version of rustc you just build
  - Comment `rustfmt: nightly-YYYY-MM-DD`
  - Uncomment `dev: 1`
- Update src/ci/run.sh to pass "--release-channel=beta".

## Master bootstrap update (T-1 day, Wednesday)

Send a PR to the master branch to:

- modify src/stage0.txt to bootstrap from yesterday's beta
- Remove `cfg(stage0)` annotated items
- Replace `cfg(not(stage0))` with nothing

## Release day (Thursday)

Decide on a time to do the release, T.

- **T-30m** - Run the following command in a shell with [AWS
  credentials][awscli] in the [simpleinfra] repository:

  ```
  ./start-release.py prod stable
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
  on GitHub Actions on the master branch.

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

  To publish Cargo you may have to bump the version numbers for the crates-io and Cargo crates; there's no need to do that in a formal commit though, so your tag and the published code may differentiate in that way.

- **T+1hr** Send a PR to the beta branch to comment out `dev: 1` again and
  update the date to download from (modifying `src/stage0.txt`).

[update-thanks]: https://github.com/rust-lang/thanks/actions?query=workflow%3ACI

Bask in your success.

## Rebuilding stable pre-releases

If something goes wrong and we need to rebuild the stable artifacts, merge the
PR on the `stable` branch of the [rust-lang/rust] repository. Once the commit
is merged, issue the following command in a shell with [AWS
credentials][awscli] on the [simpleinfra] repository:

```
./start-release.py dev stable --bypass-startup-checks
```

## Publishing a nightly based off a try build

Sometimes a PR requires testing how it behaves when downloaded from rustup, for
example after a manifest change. In those cases it's possible to publish a new
nightly based off that PR on dev-static.rust-lang.org.

Once the try build finishes grab the merge commit SHA and run the following
command in a shell with [AWS credentials][awscli] on the [simpleinfra]
repository:

```sh
./start-release.py dev nightly $MERGE_COMMIT_SHA
```

When the release process end you'll be able to install the new nightly with:

```sh
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup toolchain install nightly
```

[awscli]: /infra/docs/aws-access.md#using-the-aws-cli
[rust-lang/rust]: https://github.com/rust-lang/rust
[simpleinfra]: https://github.com/rust-lang/simpleinfra
