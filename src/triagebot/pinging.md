# Pinging

Triagebot can be used to "ping" teams of people that do not have corresponding GitHub teams.
This is useful because sometimes we want to keep groups of people that we can notify but we don't want to add all the members in those groups to the GitHub org, as that would imply that they are members of the Rust team (for example, GitHub would decorate their names with "member" and so forth).
The compiler team uses this feature to reach the [notification groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html).

When a team is pinged, we will both post a message to the issue and add a label.
The message will include a `cc` line that `@`-mentions all members of the team.

## Usage

On repositories with a ping group configured, any Rust team member (and wg-triage, wg-async) can write a GitHub comment such as:

```text
@rustbot ping windows
```

which would cause triagebot to post a comment notifying the members of the `windows` ping group.

### Teams that can be pinged

To be pinged, teams have to be created in the [Rust team repository](https://github.com/rust-lang/team).
Frequently those teams will be marked as `marker-team`, meaning that they do not appear on the website.
The [WASM team](https://github.com/rust-lang/team/blob/master/teams/wasm.toml#L2) is an example.

Additionally, the team needs to be configured in the repository's `triagebot.toml` file.

## Configuration

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

You can also define aliases to add additional labels to refer to same target team.
Aliases can be useful to add mnemonic labels or accommodate slight misspellings (such as "llvms" instead "llvm"), see the following example:

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


## Implementation

See [`parser/src/command/ping.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/ping.rs) and
[`src/handlers/ping.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/ping.rs).
