# Concern

The `concern` command is used to formally register a concern at the top comment of a GitHub issue/PR.

## Usage

A concern can be registered by writing a comment with the command:

```text
@rustbot concern my concern title
```

The concern is then added in the top comment of the GitHub issue to a special section: `Concerns` (which should *not* be edited by hand).

*Concerns can only be registered by member of the organization and cannot be registered on active [rfcbot FCPs](https://rfcbot.rs).*

### Resolving a concern

Concerns can be resolved by writing a comment with the command:

```text
@rustbot resolve my concern title
```

The concern is then strikethrough and a link to the comment resolving the concern is added next to it.

## Configuration

This feature is enabled by having a `[concern]` table in `triagebot.toml`:

```toml
[concern]
labels = ["has-concerns"] # optional, list of labels to be added to the issue/PR when there are un-resolved concerns
```

## Implementation

See [`parser/src/command/concern.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/concern.rs) and [`src/handlers/concern.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/concern.rs).
