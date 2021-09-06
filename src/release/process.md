# The Rust Release Process

Here's how Rust is currently released:

## Bump the stable version number (T-6 days, Friday the week before)

Open a PR bumping the version number in `src/version`. `r+ rollup=never` this
PR.

Mark it as `rollup=never`, because if it lands in a rollup as *not* the first
PR then other pull requests in that rollup will be incorrectly associated with
the prior release.

## Promote branches (T-3 days, Monday)

Both promotions should happen on Monday. You can open both PRs at the same
time, but make sure the stable promotion lands first.

### Beta to stable

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

- Update `src/ci/channel` to `stable`

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

### Master to beta

Gather the relevant information and push the new Cargo branch:

```sh
git fetch git@github.com:rust-lang/rust
BRANCH_POINT=`git log --merges --first-parent --format="%P" -1 master -- src/version | awk '{print($1)}'`
NEW_BETA_VERSION=`git show $BRANCH_POINT:src/version`
CARGO_SHA=`git rev-parse $BRANCH_POINT:src/tools/cargo`

cd src/tools/cargo
git branch rust-$NEW_BETA_VERSION $CARGO_SHA
git push git@github.com:rust-lang/cargo rust-$NEW_BETA_VERSION
```

Temporarily disable banch protection on GitHub for the `beta` branch of the Rust
repo. Promote rust-lang/rust's master branch to beta as you did for stable:

```sh
git push git@github.com:rust-lang/rust $BRANCH_POINT:beta -f
```

Re-enable branch protection on GitHub. Send a PR to the freshly created beta
branch of rust-lang/rust which updates `src/ci/channel` to `beta`.

## Master bootstrap update (T-2 day, Tuesday)

Send a PR to the master branch to:

- Run `./x.py run src/tools/bump-stage0` to update the bootstrap compiler to
  the beta you created yesterday.

- Remove references to the `bootstrap` and `not(bootstrap)` conditional
  compilation attributes. You can find all of them by installing [ripgrep] and
  running this command:

  ```
  rg '#!?\[.*\(bootstrap' -t rust
  ```

  The general guidelines (both for `#[]` and `#![]`) are:

  - Remove any item annotated with `#[cfg(bootstrap)]`.
  - Remove any `#[cfg(not(bootstrap))]` attribute while keeping the item.
  - Remove any `#[cfg_attr(bootstrap, $attr)]` attribute while keeping the item.
  - Replace any `#[cfg_attr(not(bootstrap), doc="$doc")]` with `$doc` in the
    relevant documentation block (or in a new documentation block).
  - Replace any `#[cfg_attr(not(bootstrap), $attr)]` with `#[$attr]`.

## Release day (Thursday)

Decide on a time to do the release, T.

- **T-50m** - Run the following command in a shell with [AWS
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

- **T-2m** - Merge blog post.

- **T** - Tweet and post everything!

  - Twitter [@rustlang](https://twitter.com/rustlang)
  - [Users forum](https://users.rust-lang.org/)

- **T+5m** - Release and tag Cargo. In the rust-lang/rust repository on the
  **stable branch**:

  ```sh
  # Remote "rust-lang" is github.com/rust-lang/rust.git
  git fetch rust-lang
  git checkout rust-lang/stable
  # Make sure submodules are at the correct revision.
  git submodule update
  cd src/tools/cargo
  # Publish to crates.io. This will publish internal dependencies first (if
  # necessary), then publish Cargo itself.
  ./publish.py
  # Where YY is the Rust minor release, add one to it (Rust 1.49.0 = Cargo 0.50.0).
  CARGO_VERSION="0.YY+1.0"
  git tag -u FA1BE5FE $CARGO_VERSION
  git push git@github.com:rust-lang/cargo.git $CARGO_VERSION
  ```

- **T+1hr** Send a PR to the beta branch running `./x.py run
  src/tools/bump-stage0` to bump the boostrap compiler to the stable you
  just released.

[update-thanks]: https://github.com/rust-lang/thanks/actions/workflows/ci.yml

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
[ripgrep]: https://github.com/burntsushi/ripgrep
