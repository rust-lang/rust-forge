# Tracking PR assignment

If you contribute in some capacity to the Rust compiler development, you might also be assigned pull requests to be reviewed.

You can check your current review assignment in two ways:
- by visiting this [GitHub URL](https://github.com/pulls?q=org%3Arust-lang+is%3Aopen+is%3Apr+assignee%3A%40me+archived%3Afalse)
- by interacting with the `triagebot` on the [Zulip chat](/platforms/zulip.md) in a DM (Direct Messages) thread. You can open a direct message session with the `triagebot` clicking on [this link](https://rust-lang.zulipchat.com/#narrow/dm/261224-triagebot) (requires Zulip login).

This chapter will describe how to interact with the `triagebot` on Zulip.

## Configuration

Tracking the PR assignment is enabled on the git repository by having a `[pr-tracking]` table in `triagebot.toml`. No additional configuration is needed.

## Usage

Open a Direct Message session with the `triagebot` and send a message with one of these commands:

* `work` --- Will emit an error and show the available commands
* `work show` --- Will show your Github username and a list of pull requests assigned to you for review (on the `rust-lang/rust` git repository)

## Implementation

See [`parser/src/handlers/pr_tracking.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/handlers/pr_tracking.rs).
