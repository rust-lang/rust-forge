# Review queue tracking

Triagebot supports more advanced tracking of reviewers' workload in the `rust-lang/rust` repository. It tracks how many "relevant" pull requests are assigned to each reviewer, and allows reviewers to configure maximum capacity of such PRs, and also if they want to be automatically assigned or not.

This page describes how the review queue works and how you can interact with `triagebot` on Zulip to configure and examine the review queue.

## Configuration

To enable review queue tracking for a repository, include `[pr-tracking]` table in its `triagebot.toml`.

To take the review queue into account when assigning reviewers on PRs, add a `[assign.review_prefs]` table to `triagebot.toml`.

> Note that this functionality currently only works for the `rust-lang/rust` repository (it is hardcoded in `triagebot`). Enabling it for more repositories requires additional design and implementation work.

## Review queue design

The review queue remembers how many "relevant" `rust-lang/rust` PRs are assigned to each reviewer at any given point in time.
Currently, the heuristic for what makes a PR "relevant" works as follows:
- The PR must not be blocked (it must not have the `S-blocked` or `S-inactive` labels).
- The PR must not be a rollup.
- The PR must be waiting for a reviewer (must have the `S-waiting-on-review` label).
- The PR must be assigned to someone else than the PR author.
- The PR must be open and not a draft.

If a PR passes all these checks and it is assigned to reviewer `R`, it will be considered to be in `R`'s review queue.

See the implementation of the [`waits_for_a_review`](https://github.com/rust-lang/triagebot/blob/7044e5449aaf29eea939bc7db81c63b7c872b9b8/src/handlers/pr_tracking.rs#L277-L304) function in triagebot for more details.

## Review preferences

Reviewers can configure *review preferences* that are taken into account when determining who to assign on a PR:
- Review queue capacity (`C`) --- if the number of PRs in your review queue is at (or above) `C`, `triagebot` will not assign new pull requests to you.
- Rotation mode (`on` or `off` rotation) --- if you set your rotation mode to be `off`, `triagebot` will not assign new pull requests to you.
  - This is an alternative to setting yourself as being ["on vacation"](pr-assignment.md#vacation) which does not require sending a pull request to modify the `triagebot.toml` file. `triagebot` takes both `users_on_vacation` in `triagebot.toml` and the rotation mode into account; if you are marked as being on vacation in either of them, it will not assign PRs to you.

Note that the review preferences only affect assignment based on adhoc groups or teams. If someone directly requests your review (`r? <user>`), triagebot will currently always assign you. If you are off rotation or at your maximum review capacity, triagebot will send a comment to the PR where you were directly assigned to let the PR author know that you might not be available for a timely review.

## Usage

You can examine your review queue and configure your review preferences by sending a [Direct Message command](zulip-commands.md#direct-message-commands) to `triagebot`:

- `work show`: Show the contents of your review queue (in the `rust-lang/rust` repository) and your review preferences.
- `work set-pr-limit <number>|unlimited`: Set your review capacity to `<number>` or remove the capacity limit (`unlimited`).
- `work set-rotation-mode off|on`: Set your rotation mode to be `on` or `off`.

## Implementation

See [`src/handlers/pr_tracking.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/pr_tracking.rs).
