# Membership
This section discusses membership in the mdBook team.

## The path to membership

People who are looking to contribute on the mdBook tool generally start on either fixing bugs,
implementing a new feature or reviewing open pull requests. If you need guidance or help, don't
hesitate to ask on the [t-mdbook channel on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/507422-t-mdbook)!

## mdBook member

Once an individual has been contributing regularly for some time, they can be promoted to the
level of a **mdBook team member** (see the section on [how decisions are made][hdam] below).
This title indicates that they are someone who contributes regularly.

It is hard to define the precise conditions when such a promotion is appropriate. Being promoted
to member is not just a function of checking various boxes. But the general sense is that someone
is ready when they have demonstrated three things:

- "Staying power" -- the person should be contributing on a regular basis in some way. This might
  for example mean that they have completed a few projects.
- "Independence and familiarity" -- they should be acting somewhat independently when taking on
  tasks, at least within the scope of their "mdBook area". They should plausibly be able to mentor
  others on simple PRs.
- "Cordiality" -- mdBook team members will be part of the Rust organization and are held to a
  higher standard with respect to the [Code of Conduct][CoC]. They should not only obey the
  letter of the CoC but also its spirit.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct

Being promoted to member implies a number of privileges:

- Members can add pull requests to the merge queue and can do reviews (they are expected to
  use those powers appropriately, as discussed previously).
- mdBook team members are members of the Rust organization so they can modify labels and be
  assigned to issues.
- Members become a part of the `rust-lang/mdBook` team on GitHub, so that they receive pings
  when people are looking to address the team as a whole.
- Members are listed on the [rust-lang.org web page].

It also implies some obligations (in some cases, optional obligations):

- Members are expected to respond to FCPs in maximum 4 weeks (28 days).
- Members may take part in various other maintainer activities to help the team.
- Members are held to a higher standard than ordinary folk when it comes to the [Code of
  Conduct][CoC].

[rust-lang.org web page]: https://www.rust-lang.org/governance/teams/dev-tools#team-rustdoc

## What it means to be a mdBook team member

Once you're a member of the mdBook team, a number of events will happen:

- You will gain access to a private Zulip stream, where internal discussions happen.
- You will also be subscribed to the `all@rust-lang.org` and `mdbook@rust-lang.org` mailing lists.
  See [this file](https://github.com/rust-lang/team/blob/HEAD/teams/all.toml) to check how
  subscriptions to mailing lists work. Both are very low-volume mailing list (maybe a few emails per
  year). About `all@rust-lang.org`: it's a way to communicate things to all contributors. We will
  not send you spam from this address.

## How promotion decisions are made

[hdam]: #how-promotion-decisions-are-made

After an individual has been contributing to mdBook for a while, they may be nominated in the
private Zulip mdBook team channel by an existing team member. All nominations **must** be done in
the private Zulip mdBook team channel.

The mdBook team members will check to see if there are concerns with extending a membership
invitation to the individual and after 10 days (barring no objections), an invitation will be
extended.

If the invitation is accepted by the individual, the mdBook team leads will update the [team]
repository to reflect their new role.

## Alumni status

If at any time a mdBook team member wishes to take a break from participating, they can opt to put
themselves into alumni status. When in alumni status, they will be removed from
GitHub aliases and the like, so that they need not be bothered with pings and messages. They will
also not have the possibility to add pull requests to the merge queue anymore. **Alumni members
will however still remain members of the GitHub org overall.**

People in alumni status can ask to return to "active" status at any time. This request would
ordinarily be granted automatically barring extraordinary circumstances.

People in alumni status are still members of the team at the level they previously attained and
they may publicly indicate that, though they should indicate the time period for which they were
active as well.

### Automatic alumni status after 6 months of inactivity

If a member or maintainer has been inactive in mdBook for 6 months, they will be moved to the
alumni status.

[team]: https://github.com/rust-lang/team
