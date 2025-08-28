# Triagebot Zulip commands

You can send commands to triagebot on the [Rust Zulip](https://rust-lang.zulipchat.com) server using two separate mechanisms:

- Sending a direct message (DM) to the [triagebot][triagebot-dm] account.
- Sending a message in some stream by tagging `@triagebot`, followed by a command (e.g. `@triagebot end-meeting`).

Triagebot commands can only be sent by users that are in the [team](https://github.com/rust-lang/team) database.

## Direct message commands

You can send these commands as a direct message to the [triagebot][triagebot-dm] account.

- `whoami`: shows the Rust teams that you are a part of
- `lookup github <zulip-name>`: lookup the GitHub username of a user by their Zulip name
- `lookup zulip <github-username>`: lookup the Zulip name of a user by their GitHub username
- `team-stats <team-name>`: show the review workqueue statistics of all members of the given team
- Reviewer workqueue commands, which are documented [here](review-queue-tracking.md#usage).
- Notification commands, which are documented [here](notifications.md#usage).

You can use the `--help` flag or a `help` subcommand for any of these commands to find out more about their parameters.

### Impersonation

You can also run the above commands on behalf of other GitHub users with the following message:

```text
as <github-username> <command>
# e.g.
as MyFavouriteGitHubUser work show
```

If you execute a "sensitive" command in this way (i.e. a command that modifies something), `triagebot` will notify the user that you have impersonated via a direct message.

Note that the impersonation functionality is intended for inspecting the status (e.g. the reviewer workqueue or Rust teams) of other users or occasional debugging. Please do not use it maliciously.

## Stream commands

- *Meeting* commands serve for controlling the flow of Zulip meetings. They are documented [here](zulip-meeting.md).
- *Rust Project Goals* commands serve for controlling Rust Project Goal tracking.
  - `@triagebot ping-goals <threshold> <next-update>`: For use by the goals team to ping goal owners on Zulip to give an update on their goal. Will not ping if there has been a comment in `<threshold>` days. `<next-update>` is a string to say when the next blog update will start.
- `@triagebot docs-update`: Generates a Pull Request ([example](https://github.com/rust-lang/rust/pull/141923)) to update the documentation submodules. See [Documentation Updates](doc-updates.md).
- `@triagebot backport [stable | beta ] [approve | decline ] <PR>` (example: "@triagebot backport beta approve 123456") Will post a comment on GitHub to approve or decline a PR backport (see [Backports](../compiler/backports.md)).

## Implementation

See [`src/zulip.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/zulip.rs).

[triagebot-dm]: https://rust-lang.zulipchat.com/#narrow/dm/261224-triagebot
