# Requesting Prioritization

Users can request an issue to be prioritized by the Prioritization WG.

## Usage

On repositories configured for prioritization, any user can post a comment with:

```text
@rustbot prioritize
```

which will add the `I-prioritize` label to the issue to notify the [Prioritization WG] that the issue needs prioritization.

[Prioritization WG]: https://www.rust-lang.org/governance/teams/compiler#Prioritization%20working%20group

## Configuration

This feature is enabled on a repository by the `[prioritize]` table in `triagebot.toml`:

```toml
[prioritize]
# Name of the label used for requesting prioritization on issues
label = "I-prioritize"
```

## Implementation

See [`parser/src/command/prioritize.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/prioritize.rs) and
[`src/handlers/prioritize.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/prioritize.rs).
