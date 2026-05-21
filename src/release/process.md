# The Rust Release Process

Here's how Rust is currently released:

## A note about the `start-release.py` script

Steps of the release process that require interacting with our production
environment are executed through the `start-release.py` script. The script
requires you to install programs and configure your local environment, and it
will guide you through the setup.

The first time you run the script (or when the pre-requisites change), you will
need to invoke the script *multiple times* until everything is setup correctly.

`start-release.py` will always start a CI job in the background. To know when
it finishes, you have to watch the logs. When the build finishes, a line like
this will appear in the logs:

```console
Phase complete: UPLOAD_ARTIFACTS State: SUCCEEDED
```

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

```console
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

    ```console
    git checkout origin/HEAD -- RELEASES.md
    ```

  - Otherwise, manually copy `RELEASES.md` from the pending release notes PR

- Update `src/ci/channel` to `stable`

You should also check whether there are beta backports that weren't merged
before the branch update, and cherry-pick them into the `stable` PR:

* [List of PRs targeting the beta branch.][target-beta]
* [List of PRs approved for beta backport.][approved-beta]
* [List of PRs *nominated* for beta backport.][nominated-beta] Note that PRs in
  this list are not approved: you should follow up with the relevant teams to
  decide what to do with them.

Self-approve the PR with `r+ rollup=never p=1000`.

Note that we need to merge this PR as soon as possible, to maximise the
pre-release testing time. If another PR is being tested by bors, and CI is not
going to finish soon (use your judgement here), you can "yield"
priority to the stable release PR by going into that PR and typing this
comment:

> @bors yield
> Yield priority to the stable release.

### `beta` PR

Send a PR to [rust-lang/rust] targeting the new `beta` branch with these
changes:

* Run this command and create a **separate commit** with just its output:

  ```console
  ./x.py run replace-version-placeholder
  ```

* Update `src/ci/channel` to `beta`

Self-approve the PR with `r+ rollup=never p=10`.

### Publish the pre-release on the `dev-static` environment

After the `stable` PR is merged you'll need to start the pre-release. Run this command from the
[rust-lang/release-team] repository[^auth]:

```console
./scripts/start-release.py publish-rust-dev-stable YYYY-MM-DD
```

You need to replace `YYYY-MM-DD` with the date of the release (Thursday).

## Default branch bootstrap update and Crater (Tuesday)

This step can only be done after the new beta has been released. The release
process for the beta happens automatically at 00:00 UTC every day, so if the
beta PR landed after that you will have to wait another day. You can check
whether beta has been released by installing it with rustup.

Send a PR to the default branch to:

- Cherry pick the commit that ran `replace-version-placeholder`
  from the now merged beta branch PR. Do not re-run the tool as there might
  have been other stabilizations on the default branch which were not included in the
  branched beta, so may not be attributed to the current release.

- Run this to update the bootstrap compiler to the beta you created yesterday:

  ```console
  ./x.py run src/tools/bump-stage0
  ```

- Remove references to the `bootstrap` and `not(bootstrap)` conditional
  compilation attributes. You can find all of them by installing [ripgrep] and
  running this command:

  ```console
  rg '#!?\[.*\(bootstrap' -t rust -t toml
  ```

  The general guidelines (both for `#[]` and `#![]`) are:

  - Remove any item annotated with `#[cfg(bootstrap)]`.
  - Remove any `#[cfg(not(bootstrap))]` attribute while keeping the item.
  - Remove any `#[cfg_attr(bootstrap, $attr)]` attribute while keeping the item.
  - Replace any `#[cfg_attr(not(bootstrap), doc="$doc")]` with `$doc` in the
    relevant documentation block (or in a new documentation block).
  - Replace any `#[cfg_attr(not(bootstrap), $attr)]` with `#[$attr]`.

  Note that if a PR adds `cfg(bootstrap)` and is merged between the beta PR and
  the default branch bootstrap update, the `rg` invocation will show them even though
  they won't have to be removed. The easiest way to handle this is to change
  them anyway and let CI show you the failure.

- Ensure there are no new warnings or Clippy lints affecting the codebase:

  ```console
  ./x clippy ci
  ```

It's also a good idea to kick off the crater runs for the next release cycle at this point now that the new beta is out. This can reduce the latency on getting results triaged from a few days up to a couple weeks depending on how long it would otherwise take for the runs to be started as part of the next cycle.

Note that this is only *starting* the crater runs, actually triaging and handling the crater runs is not the responsibility of whoever is running the release. If you're short on time this step can be skipped and whoever is responsible for the crater runs for the next cycle will start them.

- See the chapter on [release crater runs][crater] for how to start the crater runs

[crater]: ./crater.md

## Release day (Thursday)

Decide on a time to do the release. You are fully in charge of deciding when
the release happens, pick the time that works best for you. The only constraint
is, the release process must start and finish within the release day (in UTC).

Let the Social Media coordinator (currently Mara) know of the time, so that she
can be ready to post the release on the project's social media channels.

As of September 2024 a release takes between 75 and 90 minutes to complete, so
start the release process earlier enough to hit the time you planned.

To start the release, Run this command in the [rust-lang/release-team]
repository[^auth]:

```console
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

```console
./x run src/tools/bump-stage0
```

## Appendix: Rebuilding stable pre-releases

If something goes wrong and we need to rebuild the stable artifacts, merge the
PR on the `stable` branch of the [rust-lang/rust] repository. Once the commit
is merged, [authenticate with AWS][awscli] and run this command in the
[rust-lang/release-team] repository:

```console
./scripts/start-release.py publish-rust-dev-stable-rebuild
```

You'll also want to update the previously published pre-release announcement on
the blog and internals with the new information.

[^auth]: Publishing releases require authentication, and only authorized
  members of the release team can invoke it. The command will prompt you on how
  to setup your environment and how to authenticate with AWS the first time you
  execute it.

[awscli]: https://forge.rust-lang.org/infra/docs/aws-access.html#using-the-aws-console
[rust-lang/rust]: https://github.com/rust-lang/rust
[rust-lang/release-team]: https://github.com/rust-lang/release-team
[ripgrep]: https://github.com/burntsushi/ripgrep
[`promote-release`]: https://github.com/rust-lang/promote-release
[target-beta]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+base%3Abeta
[approved-beta]: https://github.com/rust-lang/rust/pulls?q=label%3Abeta-nominated+label%3Abeta-accepted
[nominated-beta]: https://github.com/rust-lang/rust/pulls?q=label%3Abeta-nominated
