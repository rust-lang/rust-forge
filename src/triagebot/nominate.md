# Nominate

The nominate commands are used for nominating issues for [backporting](../release/backporting.md).

## Usage

There are multiple commands that can be issued in a GitHub comment to handle nomination:

* `@rustbot beta-nominate <team>` --- Adds the `beta-nominated` and the given team's label.
  This indicates that the issue is nominated for beta backport, and the team should decide whether to accept or reject it.
* `@rustbot nominate <team>` --- Adds the `I-nominated` and the given team's label.
  This is used to nominate an issue for the team to discuss.
* `@rustbot beta-accept` --- Adds the `beta-accepted` label.
  This indicates that it has been approved for beta backport, and someone (usually the release team) will take care of applying the backport.
    * `@rustbot beta-approve` --- An alias for `beta-accept`.

Only rust-lang team members may use the nominate commands.

Only teams that are listed in the [configuration](#configuration) can be nominated.

If you need to nominate multiple teams, add each one in a separate command.
This is to encourage descriptions of what to do targeted at each team, rather than a general summary.

## Configuration

This feature is enabled on a repository by having a `[nominate]` table in `triagebot.toml`.
The `nominate.teams` table lists the team names, and the associated labels that should be used for that team.

```toml
[nominate.teams]
compiler = "T-compiler"
release = "T-release"
core = "T-core"
infra = "T-infra"
```

## Implementation

See [`src/handlers/nominate.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/nominate.rs) and
[`parser/src/command/nominate.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/nominate.rs).
