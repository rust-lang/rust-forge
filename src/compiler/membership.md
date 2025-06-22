# Membership
This team discusses membership in the compiler team. There are currently two levels of membership:

* members: regular contributors with r+ rights, bot privileges, and access to [infrastructure]
* maintainers: members who have committed themselves to invest in the quality of the compiler and
  health of the compiler team

[infrastructure]: ../infra/index.md

## The path to membership
People who are looking to contribute to the compiler typically start in one of two ways. They may 
tackle "one off" issues, or they may get involved in some kind of existing working group. They
don't know much about the compiler yet and have no particular privileges. They are assigned to
issues using the triagebot and (typically) work with a mentor or mentoring instructions.

## Compiler team member  
Once an individual has been contributing regularly for some time, they can be promoted to the
level of a **compiler team member** (see the section on [how decisions are made][hdam] below).
This title indicates that they are someone who contributes regularly.

It is hard to define the precise conditions when such a promotion is appropriate. Being promoted
to member is not just a function of checking various boxes. But the general sense is that someone
is ready when they have demonstrated three things:

- "Staying power" -- the person should be contributing on a regular basis in some way. This might
  for example mean that they have completed a few projects.
- "Independence and familiarity" -- they should be acting somewhat independently when taking on
  tasks, at least within the scope of the working group. They should plausibly be able to mentor
  others on simple PRs.
- "Cordiality" -- compiler team members will be part of the Rust organization and are held to a
  higher standard with respect to the [Code of Conduct][CoC]. They should not only obey the
  letter of the CoC but also its spirit.
  
[CoC]: https://www.rust-lang.org/policies/code-of-conduct

Being promoted to member implies a number of privileges:

- Members have `r+` (approve a pull request) privileges and can do reviews (they are expected to
  use those powers appropriately, as discussed previously). They also have access to control
  perf/rustc-timer and other similar bots. See the documentation for `bors` and `r+`
  [here](https://rustc-dev-guide.rust-lang.org/compiler-team.html#team-membership).

  Tip: some baseline rules around bors permissions are: don't do a `try` build unless you have
  done a check for malicious code first and don't `r+` unless you are reasonably confident that
  you can effectively review the code in question.
- Compiler team members are members of the Rust organization so they can modify labels and be
  assigned to issues.
- Members become a part of the `rust-lang/compiler` team on GitHub, so that they receive pings
  when people are looking to address the team as a whole.
- Members are [listed] on the rust-lang.org web page.
  
It also implies some obligations (in some cases, optional obligations):

- Members will be asked if they wish to be added to the reviewer rotation.
- Members may take part in various other maintainer activities to help the team. 
- Members are held to a higher standard than ordinary folk when it comes to the [Code of Conduct][CoC].

[listed]: https://www.rust-lang.org/governance/teams/compiler

## What it means to be a compiler member
Once you're a member of the compiler team, a number of events will happen:

- You will gain access to a private Zulip stream, where internal discussions happen or ideas in
  very draft state are shared. Come and say hello to your new team members!
- You will be subscribed and gain write access to a number of Github repositories. Check [this
  GitHub page](https://github.com/orgs/rust-lang/teams/compiler/repositories) to see which
  repositories you have now access to. Some of them are pretty quiet or obsolete, so don't worry
  about all of them.

  Tip: Github automatically adds you as subscriber to every repo you get write permission too. You
  can disable this in the settings ([here](https://github.com/settings/notifications)).

- You will also be subscribed to the `all@rust-lang.org` mailing list. See
  [this file](https://github.com/rust-lang/team/blob/HEAD/teams/all.toml) to check how subscriptions
  to mailing lists work. It's a very low-volume mailing list (maybe a few emails per year), it's a
  way to communicate things to all contributors. We will not send you spam from this address.

## Maintainers
After being a compiler team member for a year, members can request or be asked to become a
**compiler team maintainer**. This implies that they are not only a regular contributor, but are
actively helping to shape the direction of the team or some part of the compiler (or multiple
parts).

- Compiler team maintainers are expected to participate in at least one maintenance activities.
- Compiler team maintainers are identified with the "Maintainer" role on the rust-lang website.

## How promotion decisions are made
[hdam]: #how-promotion-decisions-are-made

After an individual has been contributing to the compiler for a while, they may be nominated by an
existing compiler team member or they may ask the compiler team leads if their contribution history
is sufficient for team membership.

The compiler team leads will check with the rest of the compiler team to see if there are concerns
with extending a membership invitation to the individual, and after a week (barring no objections),
an invitation will be extended.

If the invitation is accepted by the individual, the compiler team leads will update the [team]
repository to reflect their new role.

[team]: https://github.com/rust-lang/team

## Not just code
It is worth emphasizing that becoming a member of the compiler team does not necessarily imply
writing PRs. There are a wide variety of tasks that need to be done to support the compiler and
which should make one eligible for membership. Such tasks would include organizing meetings,
participating in meetings, bisecting and triaging issues, writing documentation, and working on the 
rustc-dev-guide.

The most important criterion for elevation to compiler team member, in particular,
is **regular and consistent** participation. The most important criterion for elevation to
maintainer is **actively shaping the direction of the team or compiler**.

## Alumni status
If at any time a compiler team member or maintainer wishes to take a break from participating,
they can opt to put themselves into alumni status. When in alumni status, they will be removed from
GitHub aliases and the like, so that they need not be bothered with pings and messages. They will
also not have r+ privileges. **Alumni members will however still remain members of the GitHub
org overall.**

People in alumni status can ask to return to "active" status at any time. This request would
ordinarily be granted automatically barring extraordinary circumstances.

People in alumni status are still members of the team at the level they previously attained and
they may publicly indicate that, though they should indicate the time period for which they were
active as well.

### Entering or leaving the Maintainer role
After a compiler team member has committed to actively maintaining the compiler by becoming a
Maintainer, they may wish to take a break from these ongoing responsibilities either temporarily
or indefinitely. In either case, the Maintainer can let the compiler team leads know or open a PR
themselves to the [team] repo, removing themselves from the Maintainer marker team and placing
themselves in the alumni list.

In the future, if the former Maintainer would like to resume maintenance duties, they can request
re-instatement from the compiler team leads. This request would ordinarily be granted automatically
barring extraordinary circumstances.

### Compiler team alumni
Likewise, if any member of the compiler team would like to take an extended break from contribution
and interaction with the team, they can let the compiler team leads know or open a PR themselves
to the [team] repo, moving themselves to alumni status.

If an alumni member would like to resume compiler team membership in the future, they can request
re-instatement from the compiler team leads and this will normally be granted.

### Automatic alumni status after 6 months of inactivity
If a member or maintainer has been inactive in the compiler for 6 months, then we will ask them if
they would like to go to alumni status. If they respond yes or do not respond, they can be placed on
alumni status.  If they would prefer to remain active, that is also fine, but they will get asked
again periodically if they continue to be inactive.

### Process: Adding a new team member
When a potential team member has been nominated by existing members, there is a standard process
that can be followed by team leads to add the new team member:

1. Contact the nominees asking if they are interested in joining the team:

```quote
Hey $name, you've been nominated for compiler team membership by a few people on the compiler
team! The [compiler team re-org RFC][rfc] has the full details as to what this means. This would
grant you permission to resources like bors and such.

This would not require you to take on additional work or responsibilities (though joining the
review queue is encouraged), and is just public recognition of the great work you've already been
doing around the compiler!

If you would like to accept, please let me know and I can update the teams repo accordingly.

[rfc]: https://rust-lang.github.io/rfcs/3599-compiler-team-reorganisation.html#team-members
```

2. Add the new nominee to the teams repository and to the [compiler team][team]. This will sync
   with Zulip, GitHub, etc. to give the new team member access and permissions.

3. Draft a Inside Rust blog post introducing the new team members. See [previous][p1] [examples][p2]
   for a template.

[team]: https://github.com/rust-lang/team/blob/master/teams/compiler.toml
[p1]: https://blog.rust-lang.org/inside-rust/2024/11/12/compiler-team-new-members/
[p2]: https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg/
