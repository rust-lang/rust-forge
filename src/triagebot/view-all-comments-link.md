# View all comments link

"View all comments" links are simple hyperlinks automatically added by triagebot to a PR or issue description. They link to triagebot comments viewer for GitHub issues and PRs.

As its name implies, these links load all the comments (and review comments) from GitHub and display them all at once. This is in contrast to the GitHub UI, where one has to click on "Load More" multiple times to see the full conversation.

## Configuration

This feature is enabled on a repository by having a `[view-all-comments-link]` table in `triagebot.toml`:

```toml
[view-all-comments-link]
threshold = 25 # 20 comments by default
exclude-prs = true # false by default
exclude-issues = false # false by default
```

All the options are optional.

The "View all comments" links will point to triagebot's `/gh-comments` endpoint - our GitHub's comments viewer.

## Implementation

See [`src/handlers/view_all_comments_link.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/view_all_comments_link.rs).
