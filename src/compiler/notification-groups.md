# Notification groups
The compiler team has a number of notification groups that used to ping people and draw their
attention to issues. Notification groups are setup so that anyone can join them if they want.

Please keep in mind that only members of a Rust project GitHub team can use these notification
groups. Non-team members will trigger an error from our automation bot.

## Creating a notification group
If you'd like to create a notification group, here are the steps. First, you want to get approval
from the compiler team:

* File a tracking issue in the [rust-lang/compiler-team] repository to collect your progress.
* Create a PR against the [rust-lang/team] repository adding the notification
  group [(Example PR)](https://github.com/rust-lang/team/pull/347)
* Configure the [rust-lang/rust] repository to accept triagebot commands
  for this group. [(Example PR.)))](https://github.com/rust-lang/rust/pull/72706)
* Create a PR for the rustc-dev-guide amending [the notification group
  section](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
  to mention your group.
* Create a sample PR for the [rust-lang/team] repository showing how one can add
  oneself. This will be referenced by your blog post to show people how to
  join. [(Example PR)](https://github.com/rust-lang/team/pull/140)
* Write an announcement blog post for Inside Rust and open a PR against
  [blog.rust-lang.org](https://github.com/rust-lang/blog.rust-lang.org).
  [(Example PR)](https://github.com/rust-lang/blog.rust-lang.org/pull/615)

[rust-lang/compiler-team]: https://github.com/rust-lang/compiler-team
[rust-lang/team]: https://github.com/rust-lang/team
[rust-lang/rust]: https://github.com/rust-lang/rust
[MCP]: ./mcp.md
