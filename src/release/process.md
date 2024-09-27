# The Rust Release Process

Here's how Rust is currently released:

## Bump the stable version number (Friday the week before)

Open a PR bumping the version number in `src/version`. `r+ rollup=never` this
PR (self-approve it).

Mark it as `rollup=never`, because if it lands in a rollup as *not* the first
PR then other pull requests in that rollup will be incorrectly associated with
the prior release.

This is effectively when the beta branch forks -- when beta is promoted, it will
be based off of the PR that landed just before this version number bump PR.

## Promote branches (Monday)

Both promotions should happen on Monday. You can open both PRs at the same
time, but prioritize landing the stable promotion first (to maximize the
pre-release testing period).

### Updating the base of the `beta` and `stable` branches

Run this command from the [rust-lang/release-team] repository[^auth]:

```
./scripts/start-release.py update-rust-branches
```

Remember that `start-release.py` starts a job in the background, and the script
will exit *before* the branches are updated. Watch the logs to see when the
background job finishes before proceeding.

### `stable` PR

Send a PR to [rust-lang/rust] targeting the new `stable` branch making the
following changes:

- Update release notes to the latest available copy:

  - If the release notes PR was merged:

    ```
    git checkout origin/master -- RELEASES.md
    ```

  - Otherwise, manually copy `RELEASES.md` from the pending release notes PR

- Update `src/ci/channel` to `stable`

Self-approve the PR with `r+ rollup=never p=1000`.

### `beta` PR

Send a PR to [rust-lang/rust] targeting the new `beta` branch with these
changes:

* Run this command and create a **separate commit** with just its output:

  ```
  ./x.py run replace-version-placeholder
  ```

* Update `src/ci/channel` to `beta`

Self-approve the PR with `r+ rollup=never p=10`.

### Publish the pre-release on the `dev-static` environment

After the `stable` PR is merged you'll need to start the pre-release. Run this command from the
[rust-lang/release-team] repository[^auth]:

```
./scripts/start-release.py publish-rust-dev-stable YYYY-MM-DD
```

You need to replace `YYYY-MM-DD` with the date of the release (Thursday).

## Master bootstrap update (Tuesday)

Send a PR to the master branch to:

- Cherry pick the commit that ran `replace-version-placeholder`
  from the now merged beta branch PR. Do not re-run the tool as there might
  have been other stabilizations on master which were not included in the
  branched beta, so may not be attributed to the current release.

- Run this to update the bootstrap compiler to the beta you created yesterday:

  ```
  ./x.py run src/tools/bump-stage0
  ```

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

Decide on a time to do the release. Let the Social Media coordinator (currently Mara)
know of the time, so that she can be ready to post the release on the project's social
media channels.

As of September 2024 a release takes between 75 and 90 minutes to complete, so
start the release process earlier enough to hit the time you planned.

To start the release, Run this command in the [rust-lang/release-team]
repository[^auth]:

```
./scripts/start-release.py publish-rust-prod-stable
```

The command will start a background job to invoke [`promote-release`] targeting
the production environment, and it will show the instructions to follow its
logs.

When the release process completes, merge the blog post PR and inform Mara to
announce the release on social media. Finally, bask in your success 🎉

## Beta stage0 update (Friday)

Send a PR to the beta branch updating the stage0 to the stable release you
published:

```
./x run src/tools/bump-stage0
```

## Appendix: Rebuilding stable pre-releases

If something goes wrong and we need to rebuild the stable artifacts, merge the
PR on the `stable` branch of the [rust-lang/rust] repository. Once the commit
is merged, [authenticate with AWS][awscli] and run this command in the
[rust-lang/release-team] repository:

```
./scripts/start-release.py publish-rust-dev-stable-rebuild
```

You'll also want to update the previously published pre-release announcement on
the blog and internals with the new information.

[^auth]: Publishing releases require authentication, and only authorized
  members of the release team can invoke it. The command will prompt you on how
  to setup your environment and how to authenticate with AWS the first time you
  execute it.

[rust-lang/rust]: https://github.com/[rust-lang/rust]
[rust-lang/release-team]: https://github.com/rust-lang/release-team
[ripgrep]: https://github.com/burntsushi/ripgrep
[`promote-release`]: https://github.com/rust-lang/promote-release
