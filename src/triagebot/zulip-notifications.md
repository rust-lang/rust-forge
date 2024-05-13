# Zulip Notifications

Triagebot can send messages to Zulip based on various triggers like issue labels.

## Usage

Zulip notifications are automated based on the configuration described below.
They can be triggered based on the addition or removal of labels, or when an issue is closed or reopened.

For example, the `rust-lang/rust` repository is configured to automatically post a message whenever an issue is tagged with the `A-edition-2021` label to the "Edition 2021" stream, which looks something like:

> triagebot
>
> Issue [#109298](https://github.com/rust-lang/rust/issues/109298) "ICE `Subslice unexpected because it isn't captured` --edition=2021" has been added.

## Configuration

This feature is enabled on a repository by having a `[notify-zulip]` table in `triagebot.toml`:

```toml
# Triggers a Zulip notification based on the given label name.
[notify-zulip."label-name"]
# The Zulip stream to post to.
# Can be found by looking for the first number in URLs, e.g. https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler
zulip_stream = 245100 # #t-compiler/wg-prioritization/alerts

# The Zulip topic to post to.
# {number} is replaced with the issue/PR number.
# {title} is replaced with the issue/PR title.
topic = "#{number} {title}"

# The message to post when the label is added.
# Supports {number} and {title} substitution.
message_on_add = "Issue #{number} \"{title}\" has been added."

# The message to post when the label is removed.
# Supports {number} and {title} substitution.
message_on_remove = "Issue #{number}'s nomination has been removed. Thanks all for participating!"

# The message to post when the issue/PR is closed and it has the label.
# Supports {number} and {title} substitution.
message_on_close = "Issue #{number} has been closed. Thanks for participating!"

# The message to post when the issue/PR is reopened and it has the label.
# Supports {number} and {title} substitution.
message_on_reopen = "Issue #{number} has been reopened. Pinging @*T-types*."

# The Zulip notification will not be posted unless the issue/PR has all of these labels.
# Please replace the `{team}` placeholder with the appropriate team to be notified for the nomination
# (ex. `I-compiler-nominated`, `I-lang-nominated`, ...)
required_labels = ["I-{team}-nominated"]
```

## Implementation

See [`src/handlers/notify_zulip.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/notify_zulip.rs).
