# Triagebot

Triaging on the rust-lang repository is an important step to take care of issues. The triagebot is the tool that allows *anyone* to help by assigning, self-assigning or labeling issues without being a member of the rust-lang organization.

To enable triagebot on a particular repository (currently only in the rust-lang organization), add a `triagebot.toml` file in the repository root. It should have a section per "feature". Please read this page to learn how to enable each feature and the options supported; if you spot something missing please let us know [by filing an issue](https://github.com/rust-lang/rust-forge/issues), thanks!

- [Issue assignment](#issue-assignment)
- [Issue notifications](#issue-notifications)
- [Ping a team](#ping-a-team)
- [Glacier](#glacier)
- [Triage](#triage)
- [Apply labels to issues](#apply-labels-to-issues)
- [Request prioritization](#request-prioritization)
- [Autolabel an issue](#autolabel-an-issue)
- [Notify Zulip](#notify-zulip)
- [Major Changes](#major-changes)

## Issue assignment

Any user belonging to the rust-lang organization can claim an issue via `@rustbot claim` or if the user is not part of the rust-lang organization `rustbot` will assign the issue to itself; then it will add a "claimed" message in the top-level comment, to signal who the current assignee is. It is possible to override someone else's claim (no warning/error is given).

You can drop your claim to the issue via `@rustbot release-assignment`; Rust team members can do the same if they want to release someone else's assignment.

`@rustbot assign @user` can be used only by Rust team members and will assign that user to the issue (with same rules as before -- either directly or indirectly).

Soon (when the "highfive" bot migration will be complete, see [rust-lang/highfive#258](https://github.com/rust-lang/highfive/pull/258)), `r?` will also assign reviewers to PRs, though unlike issues, non-team members cannot be assigned. Anyone can invoke the command.

To enable on a repository, add the following to a triagebot.toml file in the repository root.

```toml
[assign]
```

## Issue notifications

Each registered team member has a notifications page at:

`https://triage.rust-lang.org/notifications?user=<github-username>`

This page is populated from direct mentions (@user) and team mentions (@rust-lang/libs) across the rust-lang organization.

It can also be edited via Zulip by [private-messaging triagebot](https://rust-lang.zulipchat.com/#narrow/pm-with/261224-triage-rust-lang-bot). Any Rust organization member can edit their notifications page, or pages of other Rust organization team members. To do so, the editor must have a `zulip-id` listed in their people/username.toml file in the [team repository](https://github.com/rust-lang/team/). The bot will tell you which ID to use when talking to it for the first time; please `r? @Mark-Simulacrum` on PRs adding Zulip IDs.

The following commands are supported:

 * `acknowledge <url>` (or short form `ack <url>`)
 * `acknowledge <idx>` (or short form `ack <idx>`)

These both acknowledge (and remove) a notification from the list.

 * `add <url> <description... (multiple words)>`

This adds a new notification to the list.

 * `move <from> <to>`

This moves the notification at index `from` to the index `to`.

 * `meta <idx> <metadata...>`

This adds some text as a sub-bullet to the notification at `idx`. If the metadata is empty, the text is removed.

 * `as <github username> <command...>`

This executes any of the above commands as if you were the other GH user.

## Ping a team

The bot can be used to "ping" teams of people that do not have corresponding Github teams. This is useful because sometimes we want to keep groups of people that we can notify but we don't want to add all the members in those groups to the Github org, as that would imply that they are members of the Rust team (for example, Github would decorate their names with "member" and so forth). The compiler team uses this feature to reach the [notification groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html).

When a team is pinged, we will both post a message to the issue and add a label. The message will include a `cc` line that `@`-mentions all members of the team.


### Teams that can be pinged

To be pinged, teams have to be created in the [Rust team repository](https://github.com/rust-lang/team). Frequently those teams will be marked as `marker-team`, meaning that they do not appear on the website. The [LLVM team](https://github.com/rust-lang/team/blob/master/teams/icebreakers-llvm.toml#L2) is an example.

### Configuration

To enable the team (e.g. `TeamName`) to be pinged, you have to add section to the `triagebot.toml` file at the root of a repository, like so:

```toml
[ping.TeamName]
message = """\
Put your message here. It will be added as a Github comment,
so it can include Markdown and other markup.
"""
label = "help wanted"
```

This configuration would post the given message and also add the label `help wanted` to the issue.

You can also define aliases to add additional labels to refer to same target team. Aliases can be useful to add mnemonic labels or accomodate slight mispellings (such as "llvms" instead "llvm"), see the following example:

```toml
[ping.cleanup-crew]
alias = ["cleanup", "cleanups", "shrink", "reduce", "bisect"]
message = """\
message content...
"""
```

This will allow the command `@rustbot ping cleanup-crew` to be understood with all the aliased variants, ex.:

```text
@rustbot ping cleanup
@rustbot ping shrink
...
```

Check out [the rust-lang/rust configuration](https://github.com/rust-lang/rust/blob/master/triagebot.toml) for an up-to-date examples.


### Pinging teams

To ping the team `XXX`, simply leave a comment with the command:

```text
@rustbot ping XXX
```

### Related issues

* Requested in [https://github.com/rust-lang/triagebot/issues/169](https://github.com/rust-lang/triagebot/issues/169)

## Glacier

This adds the option to track ICEs (Internal Compiler Errors). Do note that the GitHub Gist must be from a [Rust Playground](https://play.rust-lang.org) link. The link must also be in quotes (`""`), example:

`@rustbot glacier "https://gist.github.com/rust-play/xxx"`

where `xxx` is the SHA1 hash of the GitHub gist generated by the Playground "share" button.

## Triage

This command can be used by people in charge of prioritizing issues, to assign either low or high priorities to issues. This is mostly done by the Compiler Prioritization WG for compiler bugs.

`@rustbot triage {high,medium,low}`

The configuration for this feature is:

```toml
[triage]
remove = ["I-prioritize"] # the set of labels to remove when this command is invoked
high = "P-high"
medium = "P-medium"
low = "P-low"
```

## Apply labels to issues

This command lets anyone apply labels to issues. This is most useful when opening an issue. In general, labels get applied to issues by the Triage WG. If you are interested in helping triaging issues, see the [Triage WG procedure](../../release/triage-procedure.md).

The specific grammar can be found [here](https://github.com/rust-lang/triagebot/blob/master/parser/src/command/relabel.rs), but some examples are listed below. The grammar is intended to be fairly intuitive for people, to prevent needing to reach for documentation when using the bot.

```text
@rustbot modify labels to +T-lang, -T-compiler
```

This will remove the `T-compiler` label and add the `T-lang` label. You can also omit the `+` sign, if you want, and it'll be implied.

You can also write the same command in a few other ways:

```text
@rustbot modify labels to +T-lang and -T-compiler
@rustbot modify labels: +T-lang and -T-compiler
@rustbot modify labels to +T-lang -T-compiler
```

Note that the command can either terminate with a `.` or a newline, otherwise the bot will not parse the command successfully.

### Errors

The bot currently restricts the labels that can be applied by people outside the Rust teams. For example, they can't add the I-unsound label. Most of the time, you shouldn't hit this. Feel free to ping the release team if you feel that a label should be added to the set of allowed labels!

### Enabling
```toml
[relabel]
# any label is allowed to be set by team members (anyone on a team in rust-lang/team)
# but these can be set by anyone in the world
allow-unauthenticated = [
    "C-*", # any C- prefixed label will be allowed for anyone
           # independent of authorization with rust-lang/team
    "!C-bug", # but not C-bug (order does not matter)
]
```

## Request prioritization

Users can request an issue to be prioritized by the Prioritization WG.

To do so, you can invoke the following command:
```text
@rustbot prioritize
```

This will simply add the `I-prioritize` label to the issue.

### Errors
The command fails if the issue has already been requested for prioritization (i.e. already has the `I-prioritize` label).

### Enabling
```toml
[prioritize]
# Name of the label used for requesting prioritization on issues
label = "I-prioritize"
```

## Autolabel an issue

When certain labels are added to an issue, this command will trigger adding a set of additional prioritization labels to the issue. In the following example adding the "I-prioritize" label will automatically add the labels in `trigger_labels` but only if the issue is not already labeled with those in `exclude_labels` (this is to avoid applying unrelated labels to issues).

```toml
[autolabel."I-prioritize"]
trigger_labels = [
    "regression-from-stable-to-stable",
    "regression-from-stable-to-beta",
    "regression-from-stable-to-nightly"
]
exclude_labels = [
    "P-*",
    "T-infra",
    "T-release"
]
```

## Notify Zulip

When a prioritization label is added to an issue, this command will create a new topic on Zulip, in the designated stream ("245100" in the following example), replacing `{number}` and `{title}` with the issue GitHub ID and title:

```toml
[notify-zulip."I-prioritize"]
zulip_stream = 245100 # t-compiler/wg-prioritization/alerts
topic = "I-prioritize #{number} {title}"
message_on_add = "@**WG-prioritization** issue #{number} has been requested for prioritization."
message_on_remove = "Issue #{number}'s prioritization request has been removed."
```

The subscribers of that Zulip stream will receive a notification and can discuss the prioritization of the issue.

## Major Changes

A major change is an issue that will have a big impact on users. See [this page on the MCP process](../../compiler/mcp.md) for detailed explanations.

The compiler team uses the major change process, which requires:
 * An issue
 * A "second", who is an expert from the Compiler team who thinks the proposal is a good idea

We have supporting automation for both parts.

First, on the rust-lang/compiler-team repository, an issue with the "major-change" label (MCP = Major Change Proposal) is created via the template. Once that's opened, it automatically gains the "to-announce" label which should be removed when it's announced at a compiler team meeting.

For seconds, you tell rustbot `@rustbot seconded` or `@rustbot second` and it will apply the relevant label. Only team members can do so.

Configuration:
```toml
[major-change]
# Label to apply once an MCP is seconded
second_label = "final-comment-period"
# Label to apply when an MCP is created
meeting_label = "to-announce"
# The Zulip stream to automatically create topics about MCPs in
# Can be found by looking for the first number in URLs, e.g. https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler
zulip_stream = 131828
```
