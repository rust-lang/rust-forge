# Major Changes

Triagebot helps with automated processing of [Major Change Proposals](../compiler/proposals-and-stabilization.md#how-do-i-submit-an-mcp).

## Usage

The process starts when the appropriate label is set on an issue.
For example, the [rust-lang/compiler-team] repo has a [major change template] which will automatically set the `major-change` label.
Triagebot will detect this and create a new Zulip topic for hosting discussion, and post a comment to the issue with a link to Zulip stream.

If a team member writes a comment on the GitHub issue with `@rustbot second` (or `@rustbot seconded`), then triagebot will set the appropriate label, and post a comment to Zulip.

If a team member adds the `major-change-accepted` label, then triagebot will post a comment to Zulip to let people know that it has been accepted.

[rust-lang/compiler-team]: https://github.com/rust-lang/compiler-team/
[major change template]: https://github.com/rust-lang/compiler-team/issues/new?assignees=&labels=major-change%2C+T-compiler&projects=&template=major_change.md&title=%28My+major+change+proposal%29

## Configuration

This feature is enabled by the `[major-change]` table in `triagebot.toml`:

```toml
[major-change]
# Issues that have this label will start the MCP process.
# Defaults to "major-change".
enabling_label = "major-change"

# Label to apply once an MCP is seconded.
second_label = "final-comment-period"

# Label to apply when an MCP is created.
# Typically this is used to track what needs to be discussed at a meeting.
meeting_label = "to-announce"

# Label that indicates there are active concerns on the MCP
# Typically tracked by `@rustbot concern`
concerns_label = "has-concerns"

# When this label is added to an issue, that triggers acceptance of the proposal
# which sends an update to Zulip.
# Defaults to "major-change-accepted".
accept_label = "major-change-accepted"

# Optional extra text that is included in the GitHub comment when the issue is opened.
open_extra_text = "cc @rust-lang/compiler @rust-lang/compiler-contributors"

# The Zulip stream to automatically create topics about MCPs in
# Can be found by looking for the first number in URLs, e.g.
# https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler
zulip_stream = 233931

# An Zulip group or username to tag in the Zulip message when a
# proposal has been seconded.
zulip_ping = "T-compiler"
```

## Implementation

See [`src/handlers/major_change.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/major_change.rs).
