# Glacier

Triagebot can be used to automatically generate PRs on <https://github.com/rust-lang/glacier/> that contain code snippets that cause an ICE (Internal Compiler Error).

## Usage

Enter the code you want to post on the [Rust Playground](https://play.rust-lang.org).
Click the "Share" button and then copy the link for "Direct link to the gist".
Then post a comment on a GitHub issue with that link as:

```text
@rustbot glacier "https://gist.github.com/rust-play/3d9134282f880c93bfe65e7db6b0680f"
```

Note that the link must be in double quotes.

## Configuration

This feature is enabled on a repository by having a `[glacier]` table in `triagebot.toml`:

```toml
[glacier]
```

## Implementation

See [`parser/src/command/glacier.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/glacier.rs) and [`src/handlers/glacier.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/glacier.rs).
