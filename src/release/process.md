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

[Obtain AWS CLI credentials][awscli] and run this command from the [simpleinfra] repository:

```
./release-scripts/promote-release.py branches
```

Once that's done, send a PR to the freshly created beta branch of rust-lang/rust
with two commits:

* The changes caused by running `./x.py run replace-version-placeholder`
* An update of `src/ci/channel` to `beta`

The version placeholder replacement changes must be in a separate commit so
that they can be cherry picked to the master branch.

Also send a PR to rust-lang/rust targeting the new stable branch making the
following changes:

- Update `src/ci/channel` to `stable`
- Update release notes to the latest available copy
  * e.g., `git checkout origin/master -- RELEASES.md`

Once the PRs are sent, r+ both and give them a high `p=1000` (for stable) and
`p=10` for beta.

After the PR is merged you'll need to start a **dev** release. [Obtain AWS CLI
credentials][awscli] and run this command from the [simpleinfra] repository:

```
# The date here is of the actual, production, stable release. Used for the blog post.
./release-scripts/promote-release.py release dev stable --release-date YYYY-MM-DD
```

## Master bootstrap update (T-2 day, Tuesday)

Send a PR to the master branch to:

- Cherry pick the commit that ran `./x.py run replace-version-placeholder`
  from the now merged beta branch PR. Do not re-run the tool as there might
  have been other stabilizations on master which were not included in the
  branched beta, so may not be attributed to the current release.
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
  ./release-scripts/promote-release.py release prod stable
  ```

  That'll, in the background, schedule the `promote-release` binary to run on
  the production secrets (not the dev secrets). That'll sign everything, upload
  it, update the html index pages, and invalidate the CDN. Note that this takes
  about 30 minutes right now. This will also push a signed tag to rust-lang/rust.

- **T-2m** - Merge blog post.

- **T** - Tweet and post everything!

  - Twitter [@rustlang](https://twitter.com/rustlang)
  - [Users forum](https://users.rust-lang.org/)

- **T+5m** - Release and tag Cargo. From a rust-lang/rust checkout (script will
  checkout the stable branch automatically), run the following script from
  [simpleinfra].

  ```sh
  ../simpleinfra/release-scripts/tag-cargo.sh
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
./release-scripts/promote-release.py release dev stable --bypass-startup-checks
```

You'll also want to update the previously published blog post and internals post
with the new information.

## Publishing a nightly based off a try build

Sometimes a PR requires testing how it behaves when downloaded from rustup, for
example after a manifest change. In those cases it's possible to publish a new
nightly based off that PR on dev-static.rust-lang.org.

Once the try build finishes grab the merge commit SHA and run the following
command in a shell with [AWS credentials][awscli] on the [simpleinfra]
repository:

```sh
./release-scripts/promote-release.py release dev nightly $MERGE_COMMIT_SHA
```

When the release process end you'll be able to install the new nightly with:

```sh
RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup toolchain install nightly
```

[awscli]: /infra/docs/aws-access.md#using-the-aws-cli
[rust-lang/rust]: https://github.com/rust-lang/rust
[simpleinfra]: https://github.com/rust-lang/simpleinfra
[ripgrep]: https://github.com/burntsushi/ripgrep
