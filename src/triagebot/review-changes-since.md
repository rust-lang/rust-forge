# Review Changes Since

This feature will automatically adjust the body of a GitHub review to include a link to view the changes that happened since the review.

## Usage

When creating a pull request review, the bot will automatically append at the end of the review body a link to view the changes that happened since the review.

## Configuration

This feature is enabled on a repository by having a `[review-changes-since]` table in `triagebot.toml`:

```toml
[review-changes-since]
```

## Implementation

See [`src/handlers/review_changes_since.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/review_changes_since.rs).
