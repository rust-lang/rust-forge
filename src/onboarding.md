# Onboarding to the Project

This document is a starting point for a new team member (or a refresh for
existing team members) on things that are useful to know as a member of the
project, in particular in terms of where to raise concerns or get help.

## Joining a team

Each team has different policies for joining new team members, as well as different responsibilities once you join. For now, you should talk to a team lead to learn what the team's policy is (and please encourage them to document the policy here!). In general, most teams expect you to at least:
- Contribute to the team for at least a couple months before joining
- Respond within a reasonable time to [Final Comment Periods][fcps] on RFCs and PRs.

[fcps]: https://rust-lang.github.io/rfcs/#what-the-process-is

### Joining wg-triage

One exception to the above is wg-triage, which we highly recommend as an introduction to working on the project. wg-triage works on triaging issues and PRs in the rust-lang/rust repository, and does not require prior experience with programming or compilers. Feel free to join wg-triage if you have ever interacted with the Rust project (interacting frequently enough that we recognize you is encouraged but not required).

To join this team, simply talk to [Dylan-DPC] and then open a PR to [rust-lang/team][team-repo]. See [team#1826] for an example of what changes to make.

For more information about wg-triage, see [Triage Procedure].

[team-repo]: https://github.com/rust-lang/team
[team#1826]: https://github.com/rust-lang/team/pull/1826
[Dylan-DPC]: https://rust-lang.zulipchat.com/#user/120823
[Triage Procedure]: ./release/triage-procedure.md

### Maintaining components without being on a team

If you have made large contributions to a specific module or file,
but have not been contributing long enough to be added to a team,
it may be beneficial to [set up triagebot mentions](./triagebot/mentions.md)
for the files/directories you have done major work on.
This will allow you to be notified of and provide feedback/context on any PRs that change those files,
which can help authors get feedback faster and reduce the amount of work that needs to be done by reviewers.

## Relationship to Council

The Leadership Council formally takes positions on behalf of the Project when
needed. All Rust project members are officially represented by approximately
one member of the council (some teams parent up to two or more members).

Generally speaking, any concerns you have that are either more widely
applicable (i.e., not just your team) or you feel aren't being fully handled
within your team can be escalated to the Council. Note that interpersonal
concerns and/or code of conduct violations should always be directed to the
Moderation team.

Escalating to the Council can be done via:

- GitHub, via a [new issue](https://github.com/rust-lang/leadership-council/issues/new/choose). This will get discussed at the regular council meeting (to which all project members are [invited](https://github.com/rust-lang/leadership-council/blob/main/procedures/meeting-observers.md)).
- Zulip, on [#council](https://rust-lang.zulipchat.com/#narrow/channel/392734-council).
- Zulip DM, to your council representative (see
  [council](https://www.rust-lang.org/governance/teams/leadership-council) for
  which team each representative represents).

Either can be a reasonable starting point, please select what you feel more
comfortable with, though bias towards being public.

Leadership Council positions are elected by the Project teams on a rotating
calendar; see [Council term limits] for more details.

[Council term limits]: governance/council.md#term-limits

## Relationship to Foundation

The Foundation works to support the Project, and the Project has direct
representation on the Foundation's board (the 5 Project directors). Those
directors are selected by the Leadership Council per the [bylaws].

Those directors have 50% of the vote on any Board vote in the Foundation,
regardless of the number of non-Project directors.

The website has the [current Project directors].

Please reach out whether you're just interested, have questions, or have concerns in:

- Zulip, on [#foundation](https://rust-lang.zulipchat.com/#narrow/channel/335408-foundation).
- To the Council (see above).
- To one of the Project Directors (see member list above), recommending Zulip
  DM as an initial point.

[current Project directors]: https://www.rust-lang.org/governance/teams/launching-pad#team-foundation-board-project-directors
[bylaws]: https://rustfoundation.org/policy/bylaws/#section-2.4-privileges-of-individual-membership
