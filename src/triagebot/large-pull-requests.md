# Large pull requests

Pull requests have diminishing returns on length. While it might seem better to bundle multiple changes in the same PR to deliver a full feature all at once, this is almost never the correct choice.

There are several studies showing that pull requests go expontentially unmerged the larger they are, and the optimal sweet spot is usually in between 150 to 250 lines.

We can use the `[large-pull-requests]` configuration option to automatically warn users if they go over this threshold. The warning looks like this:

> Seems that this commit is larger than expected (threshold: {threshold} lines of code changed)
> Big pull requests are reviewed much slower than small ones, consider splitting it.

## Configuration

```toml
[large-pull-requests]
# We should only warn if the pull request is over 300 additions / deletions
threshold = 300

# These are automated, and thus should not emit a warning
exclude-titles = ["subtree sync"]

# We don't want to penalize testing, doing as much testing
# as possible is very beneficial!
exclude-files = ["tests/*"]
```
