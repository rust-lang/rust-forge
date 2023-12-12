# Membership

This team discusses membership in the compiler team. There are currently two levels of membership:

* [contributors]: regular contributors with r+ rights, bot privileges, and access to [infrastructure]
* [full members]: full members who vote on RFCs

[full members]: https://www.rust-lang.org/governance/teams/compiler
[contributors]: https://www.rust-lang.org/governance/teams/compiler#compiler-contributors
[infrastructure]: ../infra/index.html

## The path to membership

People who are looking to contribute to the compiler typically start
in one of two ways. They may tackle "one off" issues, or they may get
involved in some kind of existing working group. They don't know much
about the compiler yet and have no particular privileges. They are
assigned to issues using the triagebot and (typically) work with a
mentor or mentoring instructions.

## Compiler team contributors
  
Once a working group participant has been contributing regularly for
some time, they can be promoted to the level of a **compiler team
contributor** (see the section on [how decisions are made][hdam]
below). This title indicates that they are someone who contributes
regularly.

It is hard to define the precise conditions when such a promotion is
appropriate. Being promoted to contributor is not just a function of
checking various boxes. But the general sense is that someone is ready
when they have demonstrated three things:

- "Staying power" -- the person should be contributing on a regular
  basis in some way. This might for example mean that they have
  completed a few projects.
- "Independence and familiarity" -- they should be acting somewhat
  independently when taking on tasks, at least within the scope of the
  working group. They should plausibly be able to mentor others on simple
  PRs.
- "Cordiality" -- contributors will be members of the organization and
  are held to a higher standard with respect to the [Code of
  Conduct][CoC]. They should not only obey the letter of the CoC but
  also its spirit.
  
[CoC]: https://www.rust-lang.org/policies/code-of-conduct

Being promoted to contributor implies a number of privileges:

- Contributors have `r+` (approve a pull request) privileges and can do reviews
  (they are expected to use those powers appropriately, as discussed
  previously). They also have access to control perf/rustc-timer and other
  similar bots. See the documentation for `bors` and `r+`
  [here](https://rustc-dev-guide.rust-lang.org/compiler-team.html#team-membership).

  Tip: some baseline rules around bors permissions are: don't do a `try` build
  unless you have done a check for malicious code first and don't `r+` unless
  you are reasonably confident that you can effectively review the code in
  question.
- Contributors are members of the organization so they can modify
  labels and be assigned to issues.
- Contributors are a member of the rust-lang/compiler team on GitHub,
  so that they receive pings when people are looking to address the
  team as a whole.
- Contributors are listed on the rust-lang.org web page.
  
It also implies some obligations (in some cases, optional obligations):

- Contributors will be asked if they wish to be added to the reviewer rotation.
- Contributors are held to a higher standard than ordinary folk when
  it comes to the [Code of Conduct][CoC].

## What it means to be a compiler contributor

Once you're a member of the compiler team contributors, a number of events will
happen:
- You will gain access to a private Zulip stream, where internal discussions
happen or ideas in very draft state are shared. Come and say hello to your new
team members!
- You will be subscribed and gain write access to the following Github repositories:
  - [rust-lang/rustc_apfloat](https://github.com/rust-lang/rustc_apfloat)
  - [rust-lang/ar_archive_writer](https://github.com/rust-lang/ar_archive_writer)
  - [rust-lang/project-thir-unsafeck](https://github.com/rust-lang/project-thir-unsafeck)
  - [rust-lang/odht](https://github.com/rust-lang/odht)
  - [rust-lang/wg-incr-comp](https://github.com/rust-lang/wg-incr-comp)
  - [rust-lang/project-rfc-2229](https://github.com/rust-lang/project-rfc-2229)
  - [rust-lang/rustc-demangle](https://github.com/rust-lang/rustc-demangle)
  - [rust-lang/surveys-private](https://github.com/rust-lang/surveys-private)
  - [rust-lang/rfcs](https://github.com/rust-lang/rfcs)

  Some of them are pretty quiet or obsolete, so don't worry about all of them.

  Tip: Github automatically adds you as subscriber to every repo you get write
  permission too. You can disable this in the settings
  ([here](https://github.com/settings/notifications)).

- You will also be subscribed to the `all@rust-lang.org` mailing list. See
[this file](https://github.com/rust-lang/team/blob/HEAD/teams/all.toml) to
check how subscriptions to mailing lists work. It's a very low-volume mailing
list (maybe a few emails per year), it's a way to communicate things to all
contributors. We will not send you spam from this address.

## Full members

As a contributor gains in experience, they may be asked to become a
**compiler team member**. This implies that they are not only a
regular contributor, but are actively helping to shape the direction
of the team or some part of the compiler (or multiple parts).

- Compiler team members are the ones who select when people should be
  promoted to compiler team contributor or to the level of member.
- Compiler team members are consulted on FCP decisions (which, in the
  compiler team, are relatively rare).
- There will be a distinct GitHub team containing only the compiler
  team members, but the name of this team is "to be determined".
- Working groups must always include at least one compiler team member
  as a lead (though groups may have other leads who are not yet full
  members).

## How promotion decisions are made
[hdam]: #how-promotion-decisions-are-made

Promotion decisions (from participant to contributor, and from
contributor to member) are made by having an active team member send
an e-mail to the alias `compiler-private@rust-lang.org`. This e-mail
should include:

- the name of the person to be promoted
- a draft of the public announcement that will be made

Compiler-team members should send e-mail giving their explicit assent,
or with objections. Objections should always be resolved before the
decision is made final. E-mails can also include edits or additions for the
public announcement.

To make the final decision:

- All objections must be resolved.
- There should be a "sufficient number" (see below) of explicit
  e-mails in favor of addition (including the team lead).
- The nominator (or some member of the team) should reach out to the person
  in question and check that they wish to join.
  
We do not require all team members to send e-mail, as historically
these decisions are not particularly controversial. For promotion to a
contributor, the only requirement is that the compiler team lead
agrees. For promotion to a full member, more explicit mails in favor
are recommended.

Once we have decided to promote, then the announcement can be posted
to internals, and the person added to the team repository.

## Not just code

It is worth emphasizing that becoming a contributor or member of the
compiler team does not necessarily imply writing PRs. There are a wide
variety of tasks that need to be done to support the compiler and
which should make one eligible for membership. Such tasks would
include organizing meetings, participating in meetings, bisecting and
triaging issues, writing documentation, working on the rustc-dev-guide.
The most important criteria for elevation to contributor,
in particular, is **regular and consistent** participation. The most
important criteria for elevation to member is **actively shaping the
direction of the team or compiler**.

## Alumni status

If at any time a current contributor or member wishes to take a break
from participating, they can opt to put themselves into alumni status.
When in alumni status, they will be removed from Github aliases and
the like, so that they need not be bothered with pings and messages.
They will also not have r+ privileges. **Alumni members will however
still remain members of the GitHub org overall.**

People in alumni status can ask to return to "active" status at any
time. This request would ordinarily be granted automatically barring
extraordinary circumstances.

People in alumni status are still members of the team at the level
they previously attained and they may publicly indicate that, though
they should indicate the time period for which they were active as
well.

### Changing back to contributor

If desired, a team member may also ask to move back to contributor
status. This would indicate a continued desire to be involved in
rustc, but that they do not wish to be involved in some of the
weightier decisions, such as who to add to the team. Like full alumni,
people who were once full team members but who went back to
contributor status may ask to return to full team member status. This
request would ordinarily be granted automatically barring
extraordinary circumstances.

### Automatic alumni status after 6 months of inactivity

If a contributor or a member has been inactive in the compiler for 6
months, then we will ask them if they would like to go to alumni
status. If they respond yes or do not respond, they can be placed on
alumni status.  If they would prefer to remain active, that is also
fine, but they will get asked again periodically if they continue to
be inactive.
