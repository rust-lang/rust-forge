# Notification groups

The compiler team has a number of notification groups that we use to
ping people and draw their attention to issues. Notification groups
are setup so that anyone can join them if they want.

## Creating a notification group

If you'd like to create a notification group, here are the steps.
First, you want to get approval from the compiler team:

* Propose the group by preparing a [Major Change Proposal][MCP]. If
  your group is not analogous to some existing group, it is probably
  a good idea to ping compiler team leads before-hand or as part of
  the MCP.
* The MCP should specify what GitHub label will be associated with the
  notification group. Often this is an existing label, such as
  `O-Windows`.

Once the MCP is accepted, here are the steps to actually create the group.
In some cases we include an example PR from some other group.

* File a tracking issue in the [rust-lang/compiler-team] repository to collect
  your progress.
* Create a PR against the [rust-lang/team] repository adding the notification
  group. [Example PR.](https://github.com/rust-lang/team/pull/347)
* Configure the [rust-lang/rust] repository to accept triagebot commands
  for this group. [Example PR.](https://github.com/rust-lang/rust/pull/72706)
* Create a PR for the rustc-dev-guide amending [the notification group
  section](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
  to mention your group.
* Create a sample PR for the [rust-lang/team] repository showing how one can add
  oneself. This will be referenced by your blog post to show people how to
  join. [Example PR.](https://github.com/rust-lang/team/pull/140)
* Create a Zulip stream for the notification group. If you don't have the permission
  to do, you can ask on [#t-compiler/wg-meta].
* Write an announcement blog post for Inside Rust and open a PR against
  [blog.rust-lang.org](https://github.com/rust-lang/blog.rust-lang.org).
  [Example PR.](https://github.com/rust-lang/blog.rust-lang.org/pull/615)

[rust-lang/compiler-team]: https://github.com/rust-lang/compiler-team
[rust-lang/team]: https://github.com/rust-lang/team
[rust-lang/rust]: https://github.com/rust-lang/rust
[#t-compiler/wg-meta]: https://rust-lang.zulipchat.com/#narrow/stream/185694-t-compiler.2Fwg-meta
[MCP]: ./mcp.md
