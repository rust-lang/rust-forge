# Membership
This section discusses membership in the rustdoc team.

## The path to membership

People who are looking to contribute on the rustdoc tool generally start on either fixing bugs
or implementing a new feature. If you need guidance or help, don't hesitate to ask on the
[t-rustdoc channel on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc)!

## Rustdoc member
Once an individual has been contributing regularly for some time, they can be promoted to the
level of a **rustdoc team member** (see the section on [how decisions are made][hdam] below).

It is hard to define the precise conditions when such a promotion is appropriate. Being promoted
to member is not just a function of checking various boxes. But the general sense is that someone
is ready when they have demonstrated three things:

- "Staying power" -- the person should be contributing on a regular basis in some way. This might
  for example mean that they have completed a few projects.
- "Independence and familiarity" -- they should be acting somewhat independently when taking on
  tasks, at least within the scope of their "rustdoc area". They should plausibly be able to mentor
  others on simple PRs.
- "Cordiality" -- rustdoc team members will be part of the Rust organization and are held to a
  higher standard with respect to the [Code of Conduct][CoC]. They should not only obey the
  letter of the CoC but also its spirit.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct

Being promoted to member implies a number of privileges:
<!-- FIXME(fmease): Not only r+ but also GH approval/merge rights in certain other repos -->

- Members have `r+` (approve a pull request) privileges and can do reviews (they are expected to
  use those powers appropriately, as discussed previously). They also have access to control
  perf/rustc-timer and other similar bots. See the documentation for `bors` and `r+`
  [here](https://rustc-dev-guide.rust-lang.org/contributing.html#r-1).

  Tip: some baseline rules around bors permissions are: don't do a `try` build unless you have
  done a check for malicious code first and don't `r+` unless you are reasonably confident that
  you can effectively review the code in question.
- Rustdoc team members are members of the Rust organization so they can modify labels and be
  assigned to issues.
- Members become a part of the `rust-lang/rustdoc` team on GitHub, so that they receive pings
  when people are looking to address the team as a whole.
- Members are listed on the [rust-lang.org web page].

It also implies some obligations (in some cases, optional obligations):

- Members are expected to respond to FCPs in maximum 4 weeks (28 days).
- Members may take part in various other maintainer activities to help the team.
- Members are held to a higher standard than ordinary folk when it comes to the [Code of
  Conduct][CoC].

[rust-lang.org web page]: https://www.rust-lang.org/governance/teams/dev-tools#team-rustdoc

## What it means to be a rustdoc member
Once you're a member of the rustdoc team, a number of events will happen:

- You will gain access to a private Zulip stream, where internal discussions happen.
- You will also be subscribed to the `all@rust-lang.org` and `rustdoc@rust-lang.org` mailing lists.
  See [this file](https://github.com/rust-lang/team/blob/HEAD/teams/all.toml) to check how
  subscriptions to mailing lists work. Both are very low-volume mailing list (maybe a few emails per
  year). About `all@rust-lang.org`: it's a way to communicate things to all contributors. We will
  not send you spam from this address.

## Roles in the rustdoc team

Rustdoc has multiple different areas and team members are not working on all of them. Currently
we have three main areas:

- Front-end: Everything related to HTML/CSS/JS
- JSON backend: Work on the `--output-format=json` backend.
- Internals: The internals of rustdoc: interacting with the compiler, doctests, generating
  rustdoc internal code representation, parsing command line arguments, lints, etc.

These groups are NOT full-fledged teams, and as such, to be part of any of these groups, you need to
be a member of the rustdoc team.

For now, only the front-end group has an official status in the
[team repository](https://github.com/rust-lang/team) and is called `rustdoc-frontend`. If a rustdoc
team member is interested to be part of this group, they can ask to be added into it.

Let's take the front-end group as an example. It is a part of the rustdoc team, so you need to be a
member of the rustdoc team to be able to join this group. Being part of the front-end group means
you are encouraged to be part of the review rotations for front-end pull requests and you will need
to respond on front-end FCPs.

## How promotion decisions are made
[hdam]: #how-promotion-decisions-are-made

After an individual has been contributing to rustdoc for a while, they may be nominated in the
private Zulip rustdoc team channel by an existing team member. All nominations **must** be done in
the private Zulip rustdoc team channel.

The rustdoc team members will check to see if there are concerns with extending a membership
invitation to the individual and after 10 days (barring no objections), an invitation will be
extended.

If the invitation is accepted by the individual, the rustdoc team leads will update the [team]
repository to reflect their new role.

<!-- FIXME(fmease): Need to be manually added to private rustdoc team channel because it's
not tracked by sync-team at the time of writing -->

## Alumni status
If at any time a rustdoc team member wishes to take a break from participating, they can opt to put
themselves into alumni status. When in alumni status, they will be removed from
GitHub aliases and the like, so that they need not be bothered with pings and messages. They will
also not have r+ privileges. **Alumni members will however still remain members of the GitHub
org overall.** <!-- FIXME(fmease): No longer accurate! -->

<!-- FIXME(fmease): Need to be manually removed from private rustdoc team channel because it's
not tracked by sync-team at the time of writing -->

People in alumni status can ask to return to "active" status at any time. This request would
ordinarily be granted automatically barring extraordinary circumstances.

People in alumni status are still members of the team at the level they previously attained and
they may publicly indicate that, though they should indicate the time period for which they were
active as well.

### Automatic alumni status after 6 months of inactivity
If a member or maintainer has been inactive in rustdoc for 6 months, they will be moved to the
alumni status.

[team]: https://github.com/rust-lang/team
