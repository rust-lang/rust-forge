# Review Requested

This feature will automatically adjust the labels on a pull request when the PR author requests a review from an assignee.

## Usage

In the list of reviewers, click the "Re-request review" button near an assignee's name.
This will automatically remove the "waiting on the author" labels, and add a new labels to indicate that the PR is waiting on the review.

## Configuration

This feature is enabled on a repository by having a `[review-requested]` table in `triagebot.toml`:

```toml
[review-requested]
# Those labels are removed when PR author requests a review from an assignee
remove_labels = ["S-waiting-on-author"]
# Those labels are added when PR author requests a review from an assignee
add_labels = ["S-waiting-on-review"]
```

## Implementation

See [`src/handlers/review_requested.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/review_requested.rs).
