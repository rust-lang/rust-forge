# Mentions

Triagebot can leave a comment on PRs that touch certain files.
This can be useful to alert people who want to review any change to those files, or to provide a informational message to the author.

## Usage

Mentions are triggered automatically when a PR is opened (or new changes are pushed) based on the configuration in `triagebot.toml` of the repo.

## Configuration

To enable mentions, add entries to the `[mentions]` table in `triagebot.toml`.
Each key in the table should be a path in the repo.
Triagebot will check for modifications to any file that **starts with** the given path.
For example, `library/std` would match anything under the `library/std` directory like `library/std/src/process.rs`.

There are two optional values that can be specified in the table:

* `cc` --- A list of strings of users to ping.
  They should start with `@` like `@ehuss` or `@rust-lang/clippy`.
  If this is not specified, nobody will be pinged.
* `message` --- This is the message that will be included in the comment.
  If this is not specified, the comment will say `Some changes occurred in {path}`.

Example:

```toml
[mentions."src/tools/cargo"]
cc = ["@ehuss"]

[mentions."src/rustdoc-json-types"]
message = """
rustdoc-json-types is a **public** (although nightly-only) API.
If possible, consider changing `src/librustdoc/json/conversions.rs`;
otherwise, make sure you bump the `FORMAT_VERSION` constant.
"""
```

## Implementation

See [`parser/src/mentions.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/mentions.rs) and [`src/handlers/mentions.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/mentions.rs)
