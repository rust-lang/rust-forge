# Review Changes Requested

This feature will automatically adjust the labels on a pull request when a reviewer[^1] sends a review with changes requested.

[^1]: For the purpose of this feature, a reviewer can either be one of the assignees or anyone who has "write" permissions on the repository.

## Usage

When creating a pull request review, click the "Request Changes" option when finishing the review.

This will automatically remove the review labels, and add a new label to indicate that the PR is waiting on the author.

## Configuration

This feature is enabled on a repository by having a `[review-submitted]` table in `triagebot.toml`:

```toml
[review-submitted]
# These labels are removed when a review is submitted.
review_labels = ["S-waiting-on-review"]
# This label is added when a review is submitted.
reviewed_label = "S-waiting-on-author"
```

## Implementation

See [`src/handlers/review_submitted.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/review_submitted.rs).
