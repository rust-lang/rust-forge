# Mentions

Triagebot can leave a comment on PRs that touch certain files.
This can be useful to alert people who want to review any change to those files, or to provide a informational message to the author.

## Usage

Mentions are triggered automatically when a PR is opened (or new changes are pushed) based on the configuration in `triagebot.toml` of the repo.

## Configuration

To enable mentions, add entries to the `[mentions]` table in `triagebot.toml`.

Each key in the table should either be a path in the repo or should be a string (when `type="content"`). See the dedicated section below for more details.

There are three optional values that can be specified in the table:

* `type` --- Specifies the matching type that must be satisfied, either `filename` (the default) or `content`.
* `cc` --- A list of strings of users to ping.
  They should start with `@` like `@ehuss` or `@rust-lang/clippy`.
  If this is not specified, nobody will be pinged.
* `message` --- This is the message that will be included in the comment.
  If this is not specified, the comment will say `Some changes occurred in {path}`.

### Path-based mentions

By default triagebot checks for any file that **starts with**[^1] the given UNIX-style path. Can be explicitly requested with `type="filename"`.

[^1]: In order to achieve the *starts with* an implicit glob `*` is added an the end of path.

Glob matching is supported with the following syntax:
* `?` matches any single character.
* `*` matches zero or more characters.
* `**` recursively matches directories.
* `{a,b}` matches `a` or `b` where `a` and `b` are arbitrary glob patterns.
* `[ab]` matches `a` or `b` where `a` and `b` are characters.
  `[!ab]` to match any character except for `a` and `b`.

For example, `library/std` (or `library/std*`) would match anything under the `library/std` directory like `library/std/src/process.rs`.

### Content-based mentions

Optionally triagebot can check any added lines of a PR with `type="content"`.

In those cases the key is the content to be found.

### Example

```toml
[mentions."src/tools/cargo"]
cc = ["@ehuss"]

[mentions."src/rustdoc-json-types"]
message = """
rustdoc-json-types is a **public** (although nightly-only) API.
If possible, consider changing `src/librustdoc/json/conversions.rs`;
otherwise, make sure you bump the `FORMAT_VERSION` constant.
"""

[mentions."#[rustc_attr]"]
type = "content"
cc = ["@someone"]
```

## Implementation

See [`src/handlers/mentions.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/mentions.rs).
