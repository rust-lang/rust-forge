# Triagebot

Triagebot (AKA rustbot) is a general-purpose bot used for a wide variety of tasks in the rust-lang organization, usually involving sending commands via GitHub or Zulip comments.
The following pages explain the available features.

Commands are usually issued by writing comments starting with the text `@rustbot`.
The commands that are available depends on which repository you are using.
Each repository has a `triagebot.toml` where you can see which features are enabled.

For example, the following comment:

```text
@rustbot label A-diagnostics A-macros
```

will set the given labels on a GitHub issue or pull request, even for people who don't have direct permissions to do that in the GitHub UI.

## GitHub commands

Commands on GitHub issues or pull requests are usually issued by writing `@rustbot` followed by the command anywhere in the comment.
`@rustbot` will ignore commands in markdown code blocks, inline code spans, HTML blocks, or blockquotes.
Multiple rustbot commands can be entered in a single comment.

Triagebot also allows editing of a comment.
If you don't modify the text of the command, then triagebot will ignore the edit.
However, if you modify an existing command, or add new ones, then those commands will be processed.

## Configuration

Individual GitHub repositories can configure triagebot features via a file called `triagebot.toml` in the root of the default branch.
The following pages explain the syntax needed for each feature.

For example, the `rust-lang/rust` configuration file is at <https://github.com/rust-lang/rust/blob/master/triagebot.toml>.

When first adding `triagebot.toml` to a new repository, you will need to enable permissions for the bot to operate.
This can be done by posting a PR to the [`rust-lang/team`](https://github.com/rust-lang/team) database to add `bots = ["rustbot"]` to the repository in the `repos/rust-lang` directory.

## Common command summary

The following are some common commands you may see on [rust-lang/rust](https://github.com/rust-lang/rust/).

<style>
table td:nth-child(1) {
    white-space: nowrap;
}
</style>

| Command | Description | Docs |
|---------|-------------|------|
| `@rustbot claim` | Assigns an issue to yourself. | [Issue Assignment](issue-assignment.md) |
| `@rustbot release-assignment` | Removes your assignment to an issue. | [Issue Assignment](issue-assignment.md) |
| `@rustbot assign @octocat` | Assigns an issue to a specific user. | [Issue Assignment](issue-assignment.md) |
| `@rustbot ready` | Indicates a PR is ready for review. | [Shortcuts](shortcuts.md) |
| `@rustbot author` | Indicates a PR is waiting on the author. | [Shortcuts](shortcuts.md) |
| `@rustbot blocked` | Indicates a PR is blocked on something. | [Shortcuts](shortcuts.md) |
| `@rustbot label A-diagnostics A-macros` | Adds two labels to an issue or PR. | [Labeling](labeling.md) |
| `@rustbot label -P-high` | Removes a label from an issue or PR. | [Labeling](labeling.md) |
| `@rustbot ping windows` | Posts a comment pinging the Windows ping group. | [Pinging](pinging.md) |
| `@rustbot prioritize` | Requests prioritization from the Prioritization WG. | [Prioritization](requesting-prioritization.md) |
| `r? @octocat` | Assigns a PR to a user. | [PR Assignment](pr-assignment.md) |
| `r? libs` | Assigns to a random person in the libs review group. | [PR Assignment](pr-assignment.md) |
| `r? rust-lang/cargo` | Assigns a random person from the cargo team. | [PR Assignment](pr-assignment.md) |

The following are some common commands you may see on Zulip:

| Command | Description | Docs |
|---------|-------------|------|
| `@triagebot read` | Waits for people to read a document in a meeting. | [Zulip Meeting Management](zulip-meeting.md) |
| `@triagebot end-topic` | Checks if everyone is done discussing a topic in a meeting. | [Zulip Meeting Management](zulip-meeting.md) |
| `@triagebot end-meeting` | Checks if everyone is ready to finish a meeting. | [Zulip Meeting Management](zulip-meeting.md) |

## Implementation

The source code for triagebot can be found at <https://github.com/rust-lang/triagebot>.
If you are interested in extending triagebot, the documentation there should provide some guidance on how to get started.
