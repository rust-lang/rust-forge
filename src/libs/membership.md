# Membership

Members of the library team are given extra privileges to help maintain the standard library.

## Testing and triage

Library team members get extra permissions specifically for the [`rust-lang/rust`] repository, which allow them to use tools like [`@bors try`] (testing builds), [`@rust-timer`] (performance benchmarking), and [`@crater`] (ecosystem testing). Additionally, they have access to the [developer desktops](../infra/docs/dev-desktop.md) for faster build times.

[`@bors try`]: https://rustc-dev-guide.rust-lang.org/tests/ci.html#try-builds
[`@rust-timer`]: https://rustc-dev-guide.rust-lang.org/tests/perf.html#manual-perf-runs
[`@crater`]: https://rustc-dev-guide.rust-lang.org/tests/crater.html

They also are members of the [`rust-lang`] organization on GitHub and have special permissions to manage issues and pull requests in [many different repositories](./repositories.md). They also get shown with a "Member" badge in GitHub comments on the org.

[`rust-lang`]: https://github.com/rust-lang

## Merging changes

Any library team member can approve changes to the [repositories owned by the library team](./repositories.md). Depending on the situation, this may involve a `@bors r+` command or the ability to add changes to the merge queue. In general, team members are expected to adequately review changes before merging, not merge their own changes, and not merge changes to things outside the purview of the library team, within reason.

All of the above are relative to reasonable judgment, since one of the primary requirements for team members is that they know their limits and abilities and won't approve changes they can't responsibly approve. For example, a library team member who isn't strictly on the compiler team can approve a library change that involves small, related compiler changes, but in general, library team members should at least get informal approval from other teams before the merge changes that affect them.

Similarly, library team members are not expected to know every aspect of API design and target-specific implementation details, but they should know when to call in someone else to do review if there's something they're not familiar with.

## Review rotation

Any library team member can join the review rotation for repositories the team owns, particularly [`rust-lang/rust`] where most of the standard library is located. Being on the review rotation is one of the best ways for members to help the team and learn more about the standard library, and it's one of the team's important resources.

[`rust-lang/rust`]: https://github.com/rust-lang/rust

When on the review rotation, members will be randomly assigned by [triagebot](../triagebot/index.md) to new pull requests for review, subject to [individual settings](../triagebot/review-queue-tracking.md#usage). Members can also temporarily remove themselves from review rotations or specifically libs reviews in those settings.

Members are encouraged to perform reviews regardless of whether they're on the rotation or not, though this is not required. Members are also allowed to merge PRs if they have sufficiently reviewed them, although they should generally coordinate with the assigned reviewer if said reviewer may have already done some review.

More information on reviewing library changes is detailed in the [maintaining implementations] section.

[maintaining implementations]: ./impls/review.md

## FCP process

Whenever an API change is [stabilized](./apis/stabilization.md) or another large decision has to be made by the library team, it undergoes a Final Comment Period (FCP) where people are given time to comment on the change. All team members have the ability to raise blocking concerns for FCP decisions, and passing the FCP without any concerns allows the change to be made.

Before FCP, however, a smaller subset of the team, the [libs FCP team], must acknowledge the proposal to move into FCP. Per current configuration, all but two libs FCP members must check their box on the proposal before FCP can continue. Non-FCP libs members may need libs FCP members to register their concerns for them to actually block FCP from happening.

[libs FCP team]: #fcp-membership

Participation in the FCP process is voluntary, although interested members can add themselves to the [`libs-ping`] group to be notified whenever new FCPs are proposed. Members are allowed to add or remove themselves from the ping group at any time, although libs FCP members and team leads are required to be part of the group.

[`libs-ping`]: https://github.com/rust-lang/team/blob/main/teams/libs-ping.toml

## Joining the libs

All team members are encouraged to nominate members of the community to join the team in the [`#t-libs/private`] Zulip stream. While all members must be able to be trusted with the above privileges, they are explicitly not required to be an expert in everything the library team does or participate in all library team activities. While contributions of code are the most common, members who productively contribute to documentation, ACP, and FCP discussions are very appreciated.

[`#t-libs/private`]: https://rust-lang.zulipchat.com/#narrow/channel/275122-t-libs.2Fprivate

Membership is explicitly up to team discretion, and the process is left intentionally vague to allow flexibility in who to accept. Once a member is nominated, another member must *second* the nomination in order for it to proceed. Similarly, any member can also block a nomination from proceeding with an objection either in the Zulip thread or via private feedback to a team lead, which can then be anonymized.

Again, no explicit motivation is required for the initial nomination or second, although team members are encouraged to talk about what value new members can provide for the team. All objections should ideally be associated with some form of feedback that can be communicated to the team, so they can understand if a nomination should be blocked indefinitely or just delayed until some later point.

If a period of 10 days passes in which a nominee is seconded and has no outstanding objections, the nomination is tentatively approved. At this point, team leads must assess whether the nomination has been sufficiently discussed by team membership, allowing the option to delay the nomination if more feedback is needed. If the nominee is not currently part of the Rust project, the team leads should consult with the moderation team to verify there are no potential issues, which can also block a nomination. Without any outstanding issues, the team leads can extend the invitation to the nominee and merge a change to the [`rust-lang/team`] repository to approve the nomination.

[`rust-lang/team`]: https://github.com/rust-lang/team

20 days after a nomination, if there are no seconds and/or there are still outstanding objections, a countdown of 10 days begins. If objections persist, or, lacking objections, no seconds are put forward, the nomination is automatically withdrawn. Note that this requires that there exists some singular objection which is outstanding for the entirety of these 10 days.

Per the configuration of the `#t-libs/private` channel, new members only see messages after they joined, and members should be honest about their feelings on nominees while remaining respectful. Feedback can be given via a team lead and anonymized if a member doesn't feel comfortable sharing it directly.

## Expectations

Team members are expected to remain engaged with the Rust project in some capacity, although there are no explicit criteria for participation. Engagement in PR/issue discussions, being on the review rotation, activity in team meetings, activity on Zulip threads, or activity on other official platforms of the project, are all relevant ways to participate, but they should ideally at least sometimes relate to the library team.

Additionally, members of the team are bound by the Rust project itself and expected to follow [the spirit and the letter of the Code of Conduct][Code of Conduct].

[Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct

If a team member is inactive for at least twelve months, they can be asked if they wish to remain on the team or be moved into the alumni list, with privileges revoked. Team leads can decide how long to wait after giving notice before they move someone to the alumni list. An alum may at any point self-nominate to be reinstated, requiring only a second from a team member and no objections to rejoin the team, subject to the normal 10-day/20-day period restrictions.

## FCP membership

Members of the libs FCP team hold an important role in the [FCP process] and thus have higher expectations than normal libs members. All FCP members are expected to respond to outstanding FCP proposals in a timely manner, and the FCP team should regularly attempt to resolve concerns on FCP proposals. They’re also expected to regularly attend [weekly meetings](./meetings.md) or at least participate regularly enough in conversation to help ensure progress on relevant topics.

[FCP process]: #fcp-process

Every 12 months, the FCP team should be entirely reshuffled, allowing new members to join. As of 2026, this will happen in September around the time of [Leadership Council] and [Project Director] elections, but this may change over time. Similar to Council elections, the FCP team may elect to choose a facilitator to decide the makeup of the new team, although they are also allowed to make the decision as a group. Unlike Council elections, the facilitator may be a nominee on the new FCP team; the main benefit of a facilitator is to easily collect feedback and make executive decisions, not to be a completely unbiased party.

[Leadership Council]: https://github.com/rust-lang/leadership-council/blob/main/guides/representative-selection.md
[Project Director]: https://github.com/rust-lang/leadership-council/blob/main/policies/project-directorship/election-process.md

The process starts with self-nomination, which should last at least 14 days. During this point, any team member, including members of the old FCP team, are allowed to self-nominate to be chosen for the new team. After this point, the facilitator and/or the old FCP team have 14 more days to decide upon their final candidates for the new new FCP team. The facilitator and/or the old FCP team should additionally consult the moderation team for any chosen candidates who have never held a leadership role in the project in case moderation have any concerns.

The final composition of the FCP team should be `5..=8` members, although FCP members are allowed to resign, which may drop the number below 5. Below the minimum amount, unanimous consensus of the FCP team (and potential OK from the moderation team) is allowed to invite libs members immediately without any waiting period. While below the minimum amount, the FCP team is also encouraged to nominate other promising members from the project to join the libs team with the hope of recruiting them onto the FCP team.

Below the maximum amount, the FCP team is similarly allowed to invite more members to join the team via the same mechanism.

## Ad-hoc subteams

At any point, the FCP team may decide to delegate its FCP power to dedicated subteams or working groups. This is allowed with consensus from the FCP team and does not require approval from the team lead.

In general, enthusiastic members of the libs team are allowed to create their own working groups without libs FCP privileges to organize their work; the FCP team specifically has the power to delegate FCP abilities as well.

## Team leads

One or two members of the team are explicitly team leads, who have permissions to sign off on changes for the larger project on behalf of the team, like [permission changes in the `rust-lang/team` repo][`rust-lang/team`]. Team leads are also generally expected to act as backup facilitators during FCP team reshuffling and Leadership Council elections, aid in coordinating team projects, and act as a moderator in meetings, although they are always allowed to delegate these actions.

In general, team leads should be at least as active in the project as FCP members, although being a team lead does not grant FCP team membership.

Similar to the FCP team selection, when a team lead steps down, members of the team may self-nominate to the remaining team lead for the position, and the remaining team lead is the one to choose the new team lead. Team leads should consult moderation for any potential leads that have not before held a leadership role in the Rust project.

[`rust-lang/team`]: https://github.com/rust-lang/team

Once a nominee for team lead is chosen, the current team lead should wait 10 days for feedback from the team, allowing for blocking objections. Without any blocking objections for 10 days, the nominee becomes a new team lead.

Team leads may resign like FCP members, although the team must at any point have at least one team lead. A singular team lead looking to resign should nominate a new lead before resigning, to ensure longevity of the team. Team leads are strongly encouraged to ensure that the team has multiple team leads whenever possible, and leads are additionally encouraged to rotate out (alternatingly) every few years.

## Last-resort reselection

Since the FCP team and the team lead choose their own successors, ossification of team membership is a real concern. For this situation, there is a mechanism for the majority of the team to replace the FCP team and team lead, although we hope that this mechanism will never be needed, instead resolving issues without the last-resort mechanism.

At any point, a team member may initiate a vote of no confidence for team leadership. Once initiated and seconded, the team has 14 days to cast votes. Non-FCP, non-lead members can vote to abstain, replace leadership, or keep leadership. Votes may be, but aren't required to be accompanied by motivating reasoning.

If 14 days pass, at least 50% of the team has voted (including abstentions), and at least 60% of the non-abstain votes prefer replacement, the FCP team and leadership is immediately dissolved. At this point, the libs team must vote on two new leads, with 2/3 support required to elect leads. The new leads then propose a new FCP team to be selected after 10 days with no active concerns, similar to the process for selecting new team leads.

This mechanism is intentionally not perfect and may be subject to degenerate corner-cases, but it is written with the understanding that such a case likely warrants outside intervention from Rust Project leadership and/or moderation.
