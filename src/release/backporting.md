# Backporting

There's a steady trickle of patches that need to be ported to the beta and stable branch.
Only a few people are even aware of the process, but this is actually something anybody can do.

## Beta backporting in `rust-lang/rust`

Backports of PRs to the beta branch are usually only done to fix regressions.
Getting a PR backported to the beta branch involves the following process:

1. Add the label [`beta-nominated`](https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3Abeta-nominated+-label%3Abeta-accepted) to the PR to be backported.
   This marks the PR as in the state that it needs attention from the appropriate team to decide if it should be backported.
   Anybody with triage access is free to add this label.

2. If the team thinks it should be backported, then they should add the [`beta-accepted`](https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3Abeta-accepted) label.
   Otherwise they should remove the nominated label.

3. Occasionally someone will make a beta rollup PR.
   This is often done by the release team, but it can be done by anyone.
   The process here is:

   1. Create a local branch off the `beta` branch.
   2. Cherry-pick all of the PRs that have both [`beta-nominated` and `beta-accepted`][nominated-plus-accepted] labels.
      It is usually preferred to not include PRs that have not been merged in case there are any last minute changes, or it fails when running the full CI tests.
   3. (Recommended) Run some tests locally.
      It is not uncommon that the backports may not apply cleanly, or the UI tests need to be re-blessed if there are differences in the output.
   4. Open a PR **against the beta branch** with a title that starts with `[beta]` (so reviewers can see its specialness).
   5. List all of the PRs being backported in the PR description.
      [Here's an example](https://github.com/rust-lang/rust/pull/64097).
   6. Go through all of the PRs being backported and:

      * Change the milestone to the correct value for the beta release.
      * Remove the `beta-nominated` label.
        This indicates that the backport has been completed.

      If there are a lot of PRs, this can be done quickly by opening the [nominated + accepted][nominated-plus-accepted] query, check all the PRs being backported, and use the "Milestones" and "Label" drop-downs to modify multiple PRs in bulk.

      This last step can be done before or after the beta PR has been merged, though it can be easy to forget if you wait for it to be merged.

4. A reviewer (typically from the release team) needs to verify that the backport looks correct and that it's submitted to the beta branch.
   They will then approve with `@bors r+ rollup=never` (to avoid it being rolled up on accident).
   If the author of the PR has r+ rights, and has not made significant changes while backporting, they can also self-approve the PR.


In summary, there are three states that a PR can go through:
1. `beta-nominated`: Needs the team's attention.
2. `beta-nominated` + `beta-accepted`: Waiting to be backported.
3. `beta-accepted`: Backport complete.

[nominated-plus-accepted]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aclosed+label%3Abeta-accepted+label%3Abeta-nominated

## Stable backporting in `rust-lang/rust`

Backports to the stable branch work exactly the same as beta ones, labels have
just a slightly different name: `stable-nominated` identifies a PR to be
discussed for a backport and `stable-accepted` is a PR accepted for
backport. Declined stable nomination will have the `stable-nominated` label
removed.

The `T-release` will decide on a case by case basis if a stable backport will
warrant a point (.patch) release (f.e. release a `1.50.1` between `1.50` and `1.51`).

## Beta Backporting in `rust-lang/cargo`

The procedure for backporting fixes to Cargo is similar but slightly more
extended than the `rust-lang/rust` repo's procedure. Currently there aren't
backport tags in the Cargo repository, but you'll initiate the backport process
by commenting on an associated PR, requesting a backport. Once a Cargo team
member has approved the backport to happen you're good to start sending PRs!

- First you'll send a PR to the `rust-1.21.0` branch of Cargo (replace 1.21 with
  the current rustc beta version number). Like with `rust-lang/rust` you'll
  prefix the title of your PR with `[beta]` and ensure it's flagged as going to
  beta.

- Next a Cargo reviewer will `@bors: r+` the PR and put it into the queue.
  Eventually bors will automatically merge the PR (when tests are passing) to
  the appropriate Cargo branch.

- Finally you'll send a PR to the `rust-lang/rust` repository's `beta` branch,
  updating the Cargo submodule. The Cargo submodule should be updated to the tip
  of the `rust-1.21.0` branch (the branch your Cargo PR was merged to). As like
  before, ensure you've got `[beta]` in the PR title.

After that's all said and done the Cargo change is ready to get scheduled onto
the beta release!
