# Triagebot

Triaging on the rust-lang repository is an important step to prepare the pipeline for the rust-lang teams that will take care of issues and regressions. The triagebot is the assistant for the rust-lang organization.

To enable triagebot on a particular repository (currently only in the rust-lang organization), add a `triagebot.toml` file in the repository root. It should have a section per "feature". Please read this page to learn how to enable each feature and the options supported; if you spot something missing please let us know [by filing an issue](https://github.com/rust-lang/rust-forge/issues), thanks!

- [Issue assignment](#issue-assignment)
- [Issue notifications](#issue-notifications)
- [Ping a team](#ping-a-team)
- [Glacier](#glacier)
- [Triage](#triage)
- [Applying labels to issues](#applying-labels-to-issues)
- [Requesting prioritization](#requesting-prioritization)
- [Major Changes](#major-changes)

## Issue assignment

Any user can claim an issue via `@rustbot claim`. Issues will be assigned to any user allowed to be an assignee. Users who don't will receive a "claimed" message in the top-level comment (and `rustbot` will be assigned). It is possible to override someone else's claim (no warning/error is given).

You can drop your claim to the issue via `@rustbot release-assignment`; Rust team members can do the same if they want to release someone else's assignment.

`@rustbot assign @user` can be used only by Rust team members and will assign that user to the issue (with same rules as before -- either directly or indirectly).

Soon (when the "highfive" bot migration will be complete, see [rust-lang/highfive#258](https://github.com/rust-lang/highfive/pull/258)), `r?` will also assign users to PRs, though unlike issues, non-team members cannot be assigned. Anyone can invoke the command.

To enable on a repository, add the following to a triagebot.toml file in the repository root.

```toml
[assign]

```

## Issue notifications

Each registered team member has a notifications page at:

`https://triagebot.infra.rust-lang.org/notifications?user=<gh-username>`

This page is populated from direct mentions (@user) and team mentions (@rust-lang/libs) across the rust-lang organization.

It can also be edited via Zulip by [private-messaging triagebot](https://rust-lang.zulipchat.com/#narrow/pm-with/261224-triage-rust-lang-bot). Any Rust organization member can edit their notifications page, or pages of other Rust organization team members. To do so, the editor must have a `zulip-id` listed in their people/username.toml file in the [team repository](https://github.com/rust-lang/team/). The bot will tell you which ID to use when talking to it for the first time; please `r? @Mark-Simulacrum` on PRs adding Zulip IDs.

The following commands are supported:

 * `acknowledge <url>`
 * `acknowledge <idx>`

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

The bot can be used to "ping" teams of people that do not have corresponding Github teams. This is useful because sometimes we want to keep groups of people that we can notify but we don't want to add all the members in those groups to the Github org, as that would imply that they are members of the Rust team (for example, Github would decorate their names with "member" and so forth). The compiler team uses this feature for their [ICE-breaker teams](https://rust-lang.github.io/rustc-guide/ice-breaker/about.html).

TODO: isn't ICE-breakers now called [notification groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)?

When a team is pinged, we will both post a message to the issue and add a label. The message will include a `cc` line that `@`-mentions all members of the team.


### Teams that can be pinged

To be pinged, teams have to be created in the [Rust team repository]. Frequently those teams will be marked as `marker-team`, meaning that they do not appear on the website. The [LLVM ICE-breaker team] is an example.

[Rust team repository]: https://github.com/rust-lang/team
[LLVM ICE-breaker team]: https://github.com/rust-lang/team/blob/master/teams/icebreakers-llvm.toml#L2

### Configuration

To enable the team `XXX` (or "someothername") to be pinged, you have to add section to the `triagebot.toml` file at the root of a repository, like so:

```
[ping.XXX]
alias = ["someothername"]
message = """\
Put your message here. It will be added as a Github comment,
so it can include Markdown and other markup.
"""
label = "help wanted"
```

This configuration would post the given message and also add the label `help wanted` to the issue.

Check out [the rust-lang/rust configuration] for an up-to-date example.

[the rust-lang/rust configuration]: https://github.com/rust-lang/rust/blob/master/triagebot.toml

### Pinging teams

To ping the team `XXX`, simply leave a comment with the command:

```
@rustbot ping XXX
```

### Related issues

* Requested in [https://github.com/rust-lang/triagebot/issues/169](https://github.com/rust-lang/triagebot/issues/169)

## Glacier

This adds the option to track ICEs (Internal Compiler Errors). Do note that the GitHub Gist must be from a [Rust Playground](https://play.rust-lang.org) link. The link must also be in quotes (`""`), example:

`@bot glacier "https://gist.github.com/rust-play/xxx"`

## Triage

This command can be used by people in charge of prioritizing issues, to assign either low or high priorities to issues. This is mostly done by the Compiler Prioritization WG for compiler bugs.

`@bot triage {high,medium,low,P-high,P-medium,P-low}`

The configuration for this feature is:

```toml
[triage]
remove = ["I-nominated"] # the set of labels to remove when this command is invoked
high = "P-high"
medium = "P-medium"
low = "P-low"
```

## Applying labels to issues

Anyone can apply a label to issues.

TODO: any updates [after this PR merge](https://github.com/rust-lang/triagebot/issues/591)?

The specific grammar can be found [here](https://github.com/rust-lang/triagebot/blob/master/parser/src/command/relabel.rs), but some examples are listed below. The grammar is intended to be fairly intuitive for people, to prevent needing to reach for documentation when using the bot.

```
@rustbot modify labels to +T-lang, -T-compiler
```

This will remove the `T-compiler` label and add the `T-lang` label. You can also omit the `+` sign, if you want, and it'll be implied.

You can also write the same command in a few other ways:

```
@rustbot modify labels to +T-lang and -T-compiler
@rustbot modify labels: +T-lang and -T-compiler
@rustbot modify labels to +T-lang and -T-compiler
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

## Requesting prioritization

Users can request an issue to be prioritized by the Prioritization WG.

To do so, you can invoke the following command:
```text
@rustbot prioritize
```

This will add the `I-prioritize` label to the issue, as well as creating a Zulip thread for the WG-prioritization members to discuss.

### Errors
The command fails if the issue has already been requested for prioritization (i.e. already has the `I-prioritize` label).

### Enabling
```toml
[prioritize]
# Name of the label used for requesting prioritization on issues
label = "I-prioritize"
# ID of the stream used by the Prioritization WG
zulip_stream = 227806
```

## Major Changes

A major change is an issue that will have a big impact on users. (TODO: improve description)

The compiler team uses the major change process, which requires:
 * An issue
 * A second (TODO: replace term? The meaning could not be obvious to non-native English speakers)

We have supporting automation for both parts.

First, on the rust-lang/compiler-team repository, an issue with the "major-change" label (MCP = Major Change Proposal) is created via the template. Once that's opened, it automatically gains the "to-announce" label which should be removed when it's announced at a compiler team meeting.

For seconds, you tell rustbot `@rustbot seconded` or `@rustbot second` and it will apply the relevant label. Only team members can do so.

Configuration:
```toml
[major-change]
second_label = "final-comment-period"
meeting_label = "to-announce"
# can be found by looking for the first number in URLs, e.g. https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler
zulip_stream = 131828
```
