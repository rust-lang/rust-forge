# Leadership Council

This document defines the authority[^authority] and policies of the Rust Leadership Council ("Council") to ensure successful operation of the Rust Project.

This document serves as a living document defining the current accepted set of policies governing the Council.
The basis of this document started with the text of [RFC 3392] which established the Council,
and may be updated via the [RFC process](https://rust-lang.github.io/rfcs/).

The Council delegates much of this authority to teams (which includes subteams,
working groups, etc.[^teams]) who autonomously make decisions concerning their purviews.
However, the Council retains some decision-making authority, outlined and delimited by this document.

The Council maintains a separate home site at <https://github.com/rust-lang/leadership-council> where they document their internal processes, and coordinate their work.

The Council is composed of representatives delegated to the Council from each [top-level team][top-level-teams].

The Council is charged with the success of the Rust Project as a whole.
The Council identifies work that needs to be done but does not yet have a clear owner,
creates new teams to accomplish this work,
holds existing teams accountable for the work in their purview,
and coordinates and adjusts the organizational structure of Project teams.

[RFC 3392]: https://rust-lang.github.io/rfcs/3392-leadership-council.html

# Outline

- [Motivation](#motivation)
- [Duties, expectations, and constraints on the Council](#duties-expectations-and-constraints-on-the-council)
- [Structure of the Council](#structure-of-the-council)
  - [Top-level teams](#top-level-teams)
    - [The Launching Pad top-level team](#the-launching-pad-top-level-team)
    - [Removing top-level teams](#removing-top-level-teams)
  - [Alternates and forgoing representation](#alternates-and-forgoing-representation)
  - [Term limits](#term-limits)
  - [Limits on representatives from a single company/entity](#limits-on-representatives-from-a-single-companyentity)
  - [Candidate criteria](#candidate-criteria)
  - [Credentials](#credentials)
  - [Relationship to the Rust Foundation](#relationship-to-the-rust-foundation)
- [The Council's decision-making process](#the-councils-decision-making-process)
  - [Operational vs policy decisions](#operational-vs-policy-decisions)
  - [Repetition and exceptions](#repetition-and-exceptions)
  - [The consent decision-making process](#the-consent-decision-making-process)
    - [Approval criteria](#approval-criteria)
  - [Modifying and tuning the decision-making process](#modifying-and-tuning-the-decision-making-process)
  - [Agenda and backlog](#agenda-and-backlog)
  - [Deadlock resolution](#deadlock-resolution)
  - [Feedback and evaluation](#feedback-and-evaluation)
- [Transparency and oversight for decision making](#transparency-and-oversight-for-decision-making)
  - [Decisions that the Council may make internally](#decisions-that-the-council-may-make-internally)
  - [Decisions that the Council must necessarily make privately](#decisions-that-the-council-must-necessarily-make-privately)
  - [Decisions that the Council must make via public proposal](#decisions-that-the-council-must-make-via-public-proposal)
  - [Conflicts of interest](#conflicts-of-interest)
  - [Determining and changing team purviews](#determining-and-changing-team-purviews)
- [Mechanisms for oversight and accountability](#mechanisms-for-oversight-and-accountability)
  - [Ensuring the Council is accountable](#ensuring-the-council-is-accountable)
  - [Ensuring Council representatives are accountable](#ensuring-council-representatives-are-accountable)
  - [Ensuring teams are accountable](#ensuring-teams-are-accountable)
- [Footnotes](#footnotes)

# Motivation

The Rust project consists of hundreds of globally distributed people, organized into teams with various purviews. However, a great deal of work falls outside the purview of any established team, and still needs to get done.

The Council focuses on identifying and prioritizing work outside of team purviews. The Council primarily delegates that work, rather than doing that work itself. The Council can also serve as a coordination, organization, and accountability body between teams, such as for cross-team efforts, roadmaps, and the long-term success of the Project.

# Duties, expectations, and constraints on the Council

At a high-level, the Council is *only* in charge of the following duties:

- Identifying, prioritizing, and tracking work that goes undone due to lack of clear ownership (and not due to the owners' explicit de-prioritization, placement in a backlog, etc.).
- Delegating this work, potentially establishing new (and possibly *temporary*) teams to own this work.
- Making decisions on *urgent* matters that do not have a clear owner.
    - This should only be done in exceptional circumstances where the decision cannot be delegated either to existing teams or to newly created ones.
- Coordinating Project-wide changes to teams, structures, or processes.
- Ensuring top-level teams are accountable to their purviews, to other teams, and to the Project.
- Ensuring where possible that teams have the people and resources they need to accomplish their work.
- Establishing the official position, opinion, or will of the Rust Project as a whole.
    - This helps reduce the need for Project-wide coordination, especially when a long public polling and consensus-building process is not practical - for example, when communicating with third parties who require some understanding of what the Rust Project as a whole "wants".

In addition to these duties, the Council has additional expectations and constraints, to help determine if the Council is functioning properly:

- *Delegate work*: The Council should not take on work beyond what this document explicitly assigns to it; it must delegate to existing or new teams distinct from the Council. Such teams may include Council representatives, but such membership is not part of the duties of a Council representative.
- *Ensure the Project runs smoothly in the long term*: The Council should ensure that non-urgent Project management work is prioritized and completed with enough regularity that the Project does not accumulate organizational debt.
- *Be Accountable*: As the Council wields broad power, the Council and Council representatives must be accountable for their actions. They should listen to others' feedback, and actively reflect on whether they continue to meet the duties and expectations of the position they hold.
- *Be representational*: Council representatives should not only represent the breadth of Project concerns but also the diversity of the Rust community in as many aspects as possible (demographics, technical background, etc).
- *Share burden*: All Council representatives must share burden of Council duties.
- *Respect others' purviews*: The Council must respect the purviews delegated to teams. The Council should consult with and work together with teams on solutions to issues, and should almost never make decisions that go against the wishes of any given team.
- *Act in good faith*: Council representatives should make decisions in the best interest of the Rust Project *as a whole* even if those decisions come into conflict with their individual teams, their employers, or other outside interests.
- *Be transparent*: While not all decisions (or all aspects of a decision) can be made public, the Council should be as open and transparent about their decision-making as possible. The Council should also ensure the organizational structure of the Project is clear and transparent.
- *Respect privacy*: The Council must never compromise personal or confidential information for the sake of transparency, including adjacent information that could unintentionally disclose privileged information.
- *Foster a healthy working environment*: The Council representatives should all feel satisfied with the amount and nature of their contribution. They should not feel that their presence on the Council is merely out of obligation but rather because they are actively participating in a meaningful way.
- *Evolve*: The Council is expected to evolve over time to meet the evolving needs of teams, the Project, and the community.

Council representatives, moderation team members, and other Project members serve as examples for those around them and the broader community. All of these roles represent positions of responsibility and leadership; their actions carry weight and can exert great force within the community, and should be wielded with due care. People choosing to serve in these roles should thus recognize that those around them will hold them to a correspondingly high standard.

# Structure of the Council

The Council consists of a set of team representatives, each representing one [top-level team][top-level-teams] and its subteams.

Each top-level team designates exactly one representative, by a process of their choice.

Any member of the top-level team or a member of any of their subteams is eligible to be the representative. Teams should provide members of their subteams with an opportunity for input and feedback on potential candidates.

Each representative represents at most one top-level team, even if they're also a member of other teams. The primary responsibility of representing any Rust team falls to the representative of the top-level team they fall under.[^under-multiple-teams]

All teams in the Rust Project must ultimately fall under at least one top-level team.
The [Launching Pad team][launching-pad] serves as a temporary home for teams that do not currently have a parent team.
This ensures that all teams have representation on the Council.

## Top-level teams
[top-level-teams]: #top-level-teams

The Council establishes top-level teams via public policy decisions. In general, top-level teams should meet the following criteria:
- Have a purview that is foundational to the Rust Project
- Be the ultimate decision-makers on all aspects of that purview
- Have a purview that not is a subset of another team's purview (that is, it must not be a subteam or similar governance structure)
- Have an open-ended purview that's expected to continue indefinitely
- Be a currently active part of the Rust Project

There must be between 4 and 9 top-level teams (inclusive), preferably between 5 and 8. This number balances the desire for a diverse and relatively shallow structure while still being practical for productive conversation and consent.[^number-of-representatives]

When the Council creates a new top-level team, that team then designates a Council representative.[^bootstrapping-new-teams] When creating a new top-level team, the Council must provide justification for why it should not be a subteam or other governance structure.

The set of top-level teams is:

- Compiler
- Crates.io
- Dev tools
- Infrastructure
- Language
- Launching Pad
- Library
- Moderation
- Release

### The Launching Pad top-level team
[launching-pad]: #the-launching-pad-top-level-team

The Launching Pad team *temporarily* accepts subteams that otherwise do not have a top-level team to slot underneath of. This ensures that all teams have representation on the Council, while more permanent parent teams are found or established.

The Launching Pad team is an umbrella team: it has no direct members, only subteam representatives.

The Council should work to find or create a more appropriate parent for each subteam of the Launching Pad, and subsequently move those subteams to their new parent team.

In some cases, an appropriate parent team may exist but not yet be ready to accept subteams; the Launching Pad can serve as an interim home in such cases.

The Launching Pad also serves as a default home for subteams of a team that's removed or reorganized away, if that removal or reorganization does not explicitly place those subteams somewhere else in the organization.

The Council must review subteam membership in the Launching Pad every 6 months to ensure that proper progress is being made on finding all subteams new parent teams. As with other top-level teams, the Launching Pad team can be retired (and have its representation within the Council removed) if the Council finds it to be no longer necessary. The process for retiring the Launching Pad team is the same as with other top-level teams. Alternatively, the Council is free to give the Launching Pad team its own purview.

### Removing top-level teams

Any decision to remove a team's top-level designation (or otherwise affect eligibility for the Council) requires the consent of all Council representatives, with the exception of the representative of the top-level team being removed. Despite this caveat, the representative of the team under consideration must be invited to Council deliberations concerning the team's removal, and the Council should only remove a team over their objections in extreme cases.

The Council cannot remove the moderation team. The Council cannot change the moderation team's purview without the agreement of the moderation team.

## Alternates and forgoing representation

A representative may end their term early if necessary, such as due to changes in their availability or circumstances. The respective top-level team must then begin selecting a new representative. The role of representative is a volunteer position. No one is obligated to fill that role, and no team is permitted to make serving as a representative a necessary obligation of membership in a team. However, a representative is obligated to fulfill the duties of the position of representative, or resign that position.

A top-level team may decide to temporarily relinquish their representation, such as if the team is temporarily understaffed and they have no willing representative. However, if the team does not designate a Council representative, they forgo their right to actively participate in decision-making at a Project-wide level. All Council procedures including decision-making should not be blocked due to this omission. The Council is still obligated to consider new information and objections from all Project members. However, the Council is not obligated to block decisions to specially consider or collate a non-represented team's feedback.

Sending a representative to the Council is considered a duty of a top-level team, and not being able to regularly do so means the team is not fulfilling its duties. However, a Council representative does not relinquish their role in cases of short absence due to temporary illness, vacation, etc.

A top-level team can designate an alternate representative to serve in the event their primary representative is unavailable. This alternate assumes the full role of Council representative until the return of the primary representative. Alternate representatives do not regularly attend meetings when the primary representative is present (to avoid doubling the number of attendees).

If a team's representative *and* any alternates fail to participate in any Council proceedings for 3 consecutive weeks, the team's representative ceases to count towards the decision-making quorum requirements of the Council until the team can provide a representative able to participate. The Council must notify the team of this before it takes effect. If a team wishes to ensure the Council does not make decisions without their input or without an ability for objections to be made on their behalf, they should ensure they have an alternate representative available.

A top-level team may change their representative before the end of their term, if necessary.  However, as maintaining continuity incurs overhead, teams should avoid changing their representatives more than necessary. Teams have the primary responsibility for briefing their representative and alternates on team-specific issues or positions they wish to handle on an ongoing basis. The Council and team share the responsibilities of maintaining continuity for ongoing issues within the Council, and of providing context to alternates and other new representatives.

For private matters, the Council should exercise discretion on informing alternates, to avoid spreading private information unnecessarily; the Council can brief alternates if they need to step in.

## Term limits

Council representatives' terms are one year in length. Each representative has a soft limit of three consecutive full terms for any given representative delegation (the delegation from a particular top-level team). A representative may exceed this soft limit if and only if the Council receives explicit confirmation from the respective team that they are unable to produce a different team member as a representative (for example, due to lack of a willing alternative candidate, or due to team members having blocking objections to any other candidate).

Beyond this, there is no hard limit on the number of terms a representative can serve for other top-level teams or non-consecutive terms for a single top-level team. Teams should strive for a balance between continuity of experience and rotating representatives to provide multiple people with such experience.[^representative-selection]

Half of the representative appointments shall happen at the end of March while half shall happen at the end of September. This avoids changing all Council representatives at the same time. For the initial Council, and anytime the set of top-level teams is changed, the Council and top-level teams should work together to keep term end-dates roughly evenly divided between March and September. However, each term should last for a minimum of 6 months (temporary imbalance is acceptable to avoid excessively short terms).

If the Council and top-level teams cannot agree on appropriate term end-date changes, representatives are randomly assigned to one or the other end date (at least 6 months out) to maintain balance.

## Limits on representatives from a single company/entity

Council representatives must not disproportionately come from any one company, legal entity, or closely related set of legal entities, to avoid impropriety or the appearance of impropriety. If the Council has 5 or fewer representatives, no more than 1 representative may have any given affiliation; if the Council has 6 or more representatives, no more than 2 representatives may have any given affiliation.

Closely related legal entities include branches/divisions/subsidiaries of the same entity, entities connected through substantial ownership interests, or similar. The Council may make a judgment call in unusual cases, taking care to avoid conflicts of interest in that decision.

A Council representative is affiliated with a company or other legal entity if they derive a substantive fraction of their income from that entity (such as from an employer, client, or major sponsor). Representatives must promptly disclose changes in their affiliations.

If this constraint does not hold, whether by a representative changing affiliation, top-level teams appointing new representatives, or the Council size changing, restore the constraint as follows:
- Representatives with the same affiliation may first attempt to resolve the issue amongst themselves, such that a representative voluntarily steps down and their team appoints someone else.
  - This must be a decision by the representative, not their affiliated entity; it is considered improper for the affiliated entity to influence this decision.
  - Representatives have equal standing in such a discussion; factors such as seniority in the Project or the Council must not be used to pressure people.
- If the representatives with that affiliation cannot agree, one such representative is removed at random. (If the constraint still does not hold, the remaining representatives may again attempt to resolve the issue amongst themselves before repeating this.) This is likely to produce suboptimal results; a voluntary solution will typically be preferable.
- While a team should immediately begin the process of selecting a successor, the team's existing representative may continue to serve up to 3 months of their remaining term.
- The existing representative should coordinate the transition with the incoming representative but it is the team's choice which one is an actual representative during the up to 3 month window. There is only ever one representative from the top-level team.

## Candidate criteria

The following are criteria for deciding ideal candidates. These are similar to but not the same as the criteria for an effective team lead or co-lead. While a team lead *might* also make a good Council representative, serving as a team lead and serving as a Council representative both require a substantial time investment, which likely motivates dividing those roles among different people. The criteria are not hard requirements but can be used for determining who is best positioned to be a team's representative. In short, the representative should have:
- sufficient time and energy to dedicate to the needs of the Council.
- an interest in helping with the topics of Project operations and Project governance.
- broad awareness of the needs of the Project outside of their teams or areas of active contribution.
- a keen sense of the needs of their team.
- the temperament and ability to represent and center the needs of others above any personal agenda.
- ability and willingness to represent all viewpoints from their team, not just a subset, and not just those they agree with.

While some teams may not currently have an abundance of candidates who fit this criteria, the Council should actively foster such skills within the larger Project, as these are helpful not only for Council membership but across the entire Project.

## Credentials

The Council does not have privileged access to administrative credentials for the project.
This access solely resides with the infrastructure team[^infra-creds].
The infrastructure team's responsibilities include ensuring teams have the tools and access needed to do their work effectively, while balancing against security and maintainability of our infrastructure.
The Council can help coordinate which teams should have access through policy.

## Relationship to the Rust Foundation

The Council is responsible for establishing the process for selecting Project directors. The Project directors are the mechanism by which the Rust Project's interests are reflected on the Rust Foundation board.

The Council delegates a purview to the Project directors to represent the Project's interests on the Foundation Board and to make certain decisions on Foundation-related matters. The exact boundaries of that purview are not yet specified.

# The Council's decision-making process
[decision-making]: #the-council-s-decision-making-process

The Council make decisions of two different types: operational decisions and policy decisions. Certain considerations may be placed on a given decision depending on its classification. However, by default, the Council uses a consent decision-making process for all decisions regardless of classification.

## Operational vs policy decisions

Operational decisions are made on a daily basis by the Council to carry out their aims, including regular actions taking place outside of meetings (based on established policy). Policy decisions provide general reusable patterns or frameworks, meant to frame, guide, and support operations. In particular, policy decisions can provide partial automation for operational decisions or other aspects of operations. The council defaults to the consent decision making process for all decisions unless otherwise specified.

It is not defined precisely which decisions are operations versus policy; rather, they fall somewhere along a continuum. The purpose of this distinction is not to direct or constrain the council's decision-making procedures. Instead, this distinction provides guidance to the Council, and clarifies how the Council intends to record, review, and refine its decisions over time. For the purposes of any requirements or guidance associated with the operational/policy classification, anything not labeled as either operational or policy in this or future policy defaults to policy.

## Repetition and exceptions
[repetition-and-exceptions]: #repetition-and-exceptions

Policy decisions often systematically address what might otherwise require repeated operational decisions. The Council should strive to recognize when repeated operational decisions indicate the need for a policy decision, or a policy change. In particular, the Council should avoid allowing repeated operational decisions to constitute de facto policy.

Exceptions to existing policy cannot be made via an operational decision unless such exceptions are explicitly allowed in said policy. Avoiding ad-hoc exceptions helps avoid ["normalization of deviance"](https://en.wikipedia.org/wiki/Normalization_of_deviance).

## The consent decision-making process

Consent means that no representative's requirements (and thus those of the top-level team and subteams they represent) can be disregarded. The Council hears all relevant input and sets a good foundation for working together equitably with all voices weighted equally.

The Council uses consent decision-making where instead of being asked "do you agree?", representatives are asked "do you object?". This eliminates "pocket vetoes" where people have fully reviewed a proposal but decide against approving it without giving clear feedback as to the reason. Concerns, feedback, preferences, and other less critical forms of feedback do not prevent making a decision, but should still be considered for incorporation earlier in drafting and discussion. Objections, representing an unmet requirement or need, *must* be considered and resolved to proceed with a decision.

### Approval criteria

The consent decision-making process has the following approval criteria:
- Posting the proposal in one of the Council's designated communication spaces (a meeting or a specific channel).
- Having confirmation that at least N-2 Council representatives (where N is the total number of Council representatives) have fully reviewed the final proposal and give their consent.
- Having no outstanding explicit objections from any Council representative.
- Providing a minimum 10 days for feedback.

The approval criteria provides a quorum mechanism, as well as sufficient time for representatives to have seen the proposal. Allowing for two non-signoffs is an acknowledgement of the volunteer nature of the Project, based on experience balancing the speed of decisions with the amount of confirmation needed for consent and non-objection; this assumes that those representatives have had time to object if they wished to do so. (This is modeled after the process used today for approval of RFCs.)

The decision-making process can end at any time if the representative proposing it decides to retract their proposal. Another representative can always adopt a proposal to keep it alive.

If conflicts of interest result in the Council being unable to meet the N-2 quorum for a decision, the Council cannot make that decision unless it follows the process documented in [the "Conflicts of interest" section for how a decision may proceed with conflicts documented][conflicts-of-interest]. In such a case, the Council should consider appropriate processes and policies to avoid future recurrences of a similar conflict.

## Modifying and tuning the decision-making process

Using the public policy process, the Council can establish different decision-making processes for classes of decisions.

When deciding on which decision-making process to adopt for a particular class of decision, the Council balances the need for quick decisions with the importance of confidence in full alignment. Consent decision-making processes fall on the following spectrum:

- Consensus decision making (prioritizes confidence in full alignment at the expense of quick decision making): team members must review and prefer the proposal over all others, any team members may raise a blocking objection
- Consent decision making (default for the Council, balances quick decisions and confidence in alignment): team members must review and may raise a blocking objection
- One second and no objections (prioritizes quick decision making at the expense of confidence in alignment): one team member must review and support, any team member may raise a blocking objection

Any policy that defines decision-making processes must at a minimum address where the proposal may be posted, quorum requirements, number of reviews required, and minimum time delay for feedback. A lack of objections is part of the approval criteria for all decision-making processes.

If conflicts of interest prevent more than a third of the Council from participating in a decision, the Council cannot make that decision unless it follows the process documented in [the "Conflicts of interest" section for how a decision may proceed with conflicts documented][conflicts-of-interest]. (This is true regardless of any other quorum requirements for the decision-making process in use.) In such a case, the Council should consider appropriate processes and policies to avoid future recurrences of a similar conflict.

The Council may also delegate subsets of its own decision-making purviews via a public policy decision, to teams, other governance structures, or roles created and filled by the Council, such as operational lead, meeting facilitator, or scribe/secretary.

Note that the Council may delegate the drafting of a proposal without necessarily delegating the decision to approve that proposal. This may be necessary in cases of Project-wide policy that intersects the purviews of many teams, or falls outside the purview of any team. This may also help when bootstrapping a new team incrementally.

## Agenda and backlog

The Council's agenda and backlog are the primary interface through which the Council tracks and gives progress updates on issues raised by Project members throughout the Project.

To aid in the fairness and effectiveness of the agenda and backlog, the Council must:

- Use a tool that allows Project members to submit requests to the Council and to receive updates on those requests.
- Use a transparent and inclusive process for deciding on the priorities and goals for the upcoming period. This must involve regular check-ins and feedback from all representatives.
- Strive to maintain a balance between long-term strategic goals and short-term needs in the backlog and on the agenda.
- Be flexible and adaptable and be willing to adjust the backlog and agenda as needed in response to changing circumstances or priorities.
- Regularly review and update the backlog to ensure that it accurately reflects the current priorities and goals of the Council.
- Follow a clear and consistent process for moving items from the backlog to the agenda, such as delegating responsibility to roles (e.g. meeting facilitator and scribe), and consenting to the agenda at the start of meetings. Any agenda items rejected during the consent process must have their objections documented in the published meeting minutes of the Council.

## Deadlock resolution

In some situations the Council might need to make an decision urgently and not feel it can construct a proposal in that time that everyone will consent to. In such cases, if everyone agrees that a timely decision they disagree with would be a better outcome than no timely decision at all, the Council may use an alternative decision-making method to attempt to resolve the deadlock. The alternative process is informal, and the council members must still re-affirm their consent to the outcome through the existing decision making process. Council members may still raise objections at any time.

For example, the Council can consent to a vote, then once the vote is complete all of the council members would consent to whatever decision the vote arrived to. The Council should strive to document the perceived advantages and disadvantages for choosing a particular alternative decision-making model.

There is, by design, no mandatory mechanism for deadlock resolution. If the representatives do not all consent to making a decision even if they don't prefer the outcome of that decision, or if any representative feels it is still possible to produce a proposal that will garner the Council's consent, they may always maintain their objections.

If a representative withdraws an objection, or consents to a decision they do not fully agree with (whether as a result of an alternative decision-making process or otherwise), the Council should schedule an evaluation or consider shortening the time until an already scheduled evaluation, and should establish a means of measuring/evaluating the concerns voiced. The results of this review are intended to determine whether the Council should consider changing its prior decision.

## Feedback and evaluation

All policy decisions should have an evaluation date as part of the policy. Initial evaluation periods should be shorter in duration than subsequent evaluation periods. The length of evaluation periods should be adjusted based on the needs of the situation. Policies that seem to be working well and require few changes should be extended so less time is spent on unnecessary reviews. Policies that have been recently adjusted or called into question should have shortened evaluation periods to ensure they're iterating towards stability more quickly. The Council should establish standardized periods for classes of policy to use as defaults when determining periods for new policy. For instance, roles could have an evaluation date of 3 months initially then 1 year thereafter, while general policy could default to 6 months initially and 2 years thereafter.

- New policy decisions can always modify or replace existing policies.
- Policy decisions must be published in a central location, with version history.
- Modifications to the active policy docs should include or link to relevant context for the policy decision, rather than expecting people to find that context later.

# Transparency and oversight for decision making

Decisions made by the Council will necessarily require varying levels of transparency and oversight based on the kind of decision being made. This section gives guidance on how the Council will seek oversight for its decisions, and what qualifies decisions to be made in private or in public.

This RFC places certain decisions into each category. All decisions not specifically enumerated must use the public policy process. The Council may evolve the categorization through the [public policy process][decisions-that-the-council-must-make-via-public-proposal].

Decisions made by the Council fall into one of three categories, based on the level of oversight possible and necessary:

- Decisions that the Council may make internally
- Decisions that the Council must necessarily make privately
- Decisions that the Council must make via public proposal

## Decisions that the Council may make internally

Some types of operational decisions can be made internally by the Council, with the provision that the Council has a mechanism for community feedback on the decision after it has been made.

Adding a new decision to the list of decisions the Council can make internally requires a public policy decision. Any decisions that impact the structure, decision-makers, or oversight of the Council itself should not be added to this list.

The Council should also strive to avoid establishing de facto unwritten policy via repeated internal decisions in an effort to avoid public proposal. See ["Repetition and exceptions"][repetition-and-exceptions] for more details.

This list exhaustively enumerates the set of decisions that the Council may make internally:

- Deciding to start a process that itself will play out in public (e.g. "let's start developing and posting the survey", "let's draft an RFC for this future public decision").
- Expressing and communicating an official position statement of the Rust Project.
- Expressing and communicating the position of the Rust Project directly to another entity, such as the Rust Foundation.
- Communicating via Rust Project communication resources (via the blog or all@).
- Making most operational decisions about the Council's own internal processes, including how the Council coordinates, the platforms it uses to communicate, where and when it meets, templates used for making and recording decisions (subject to requirements elsewhere in this document).
- Appointing officers or temporary roles within the Council, for purposes such as leading/facilitating meetings, recording and publishing minutes, obtaining and collating feedback from various parties, etc.[^council-roles] Note that any such roles (titles, duties, and current holders) must be publicly disclosed and documented.
- Inviting specific attendees other than Council representatives to specific Council meetings or discussions, or holding a meeting open to the broader community. (In particular, the Council is encouraged to invite stakeholders of a particular decision to meetings or discussions where said decision is to be discussed.)
- Making decisions requested by one or more teams that would be within the normal purviews of those teams to make without a public proposal. (Note that teams can ask for Council input without requesting a Council decision.)
- Making one-off judgment calls in areas where the purviews of teams overlap or are ambiguous (though *changing* the purviews of those teams must be a public policy decision).
- Any decision that this document or future Council policy specifies as an operational decision.

See the [accountability section][accountability] for details on the feedback mechanism for Council decisions.

## Decisions that the Council must necessarily make privately

Some decisions necessarily involve private details of individuals or other entities, and making these details public would have a negative impact both on those individuals or entities (e.g. safety) and on the Project (eroding trust).

This additional constraint should be considered an exceptional case. This does not permit making [decisions that would require a public proposal per the next section][decisions-that-the-council-must-make-via-public-proposal]. However, this does permit decisions that the Council makes internally to be kept private, without full information provided for public oversight.

The Council may also decline to make a decision privately, such as if the Council considers the matter outside their purview (and chooses to defer to another team) or believes the matter should be handled publicly. However, even in such a case, the Council still cannot publicly reveal information shared with it in confidence (since otherwise the Council would not be trusted to receive such information). Obvious exceptions exist for imminent threats to safety.

Private decisions must not establish policy. The Council should also strive to avoid establishing de facto unwritten policy via repeated private decisions in an effort to avoid public proposal. See ["Repetition and exceptions"][repetition-and-exceptions] for more details.

This list exhaustively enumerates the set of decisions that the Council may make either partly or entirely in private:

- Determining relationships with new industry / Open Source initiatives, that require confidentiality before launching.
- Discussing the personal aspects of a dispute between teams that involves some interpersonal dynamics/conflicts.
- Participating in contract negotiations on behalf of the Project with third parties (e.g. accepting resources provided to the Project).
- Decisions touching on Project-relevant controversial aspects of politics, personal safety, or other topics in which people may not be safe speaking freely in public.
- Discussing whether and why a team or individual needs help and support, which may touch on personal matters.
- Any decision that this document or future Council policy specifies as a private decision.

The Council may pull in members of other teams for private discussions leading to either a private or public decision, unless doing so would more broadly expose private information disclosed to the Council without permission. When possible, the Council should attempt to pull in people or teams affected by a decision. This also provides additional oversight.

Some matters may not be fit for full public disclosure while still being fine to share in smaller, more trusted circles (such as with all Project members, with team leads, or with involved/affected parties). The Council should strive to share information with the largest appropriate audiences for that information.

The Council may decide to withhold new decisions or aspects of decisions when it's unclear whether the information is sensitive. However, as time progresses and it becomes clearer who the appropriate audience is or that the appropriate audience has expanded, the council should revisit its information-sharing decisions.

The Council should always loop in the moderation team for matters involving interpersonal conflict/dispute, both because such matters are the purview of the moderation team, and to again provide additional oversight.

The council should evaluate which portions of a decision or its related discussions necessarily need to be private, and should consider whether it can feasibly make non-sensitive portions public, rather than keeping an entire matter private just because one portion of it needs to be. This may include the existence of the discussion, or the general topic, if those details are not themselves sensitive.

Private matters may potentially be able to become public, or partially public, at a later date if they're no longer sensitive. However, some matters may potentially *never* be able to become public, which means they will never become subject to broader review and oversight. Thus, the Council must exercise caution and prudence before making a private decision.

The Council should make every effort to not make private decisions. The Council should have appropriate additional processes in place to encourage representatives to collectively review such decisions and consider their necessity.

## Decisions that the Council must make via public proposal
[decisions-that-the-council-must-make-via-public-proposal]: #decisions-that-the-council-must-make-via-public-proposal

Decisions in this category require the Council to publicly seek feedback from the broader Rust Project *in advance* of the decision being made. Such decisions are proposed and decided via the appropriate public decision process, currently the RFC process (though the Council may adopt a different public proposal process in the future). The public decision process must require the consent of representatives (either affirmatively or via non-objection), must allow for blocking objections by Council representatives, must provide reasonable time for public evaluation and discussion, and must provide a clear path for public feedback to the Council.

Following the existing RFC process, public proposals must have a minimum time-delay for feedback before the decision takes effect. Any representative may request that the feedback period for a particular decision is extended to at most 20 days total. The Council may make an internal operational decision to extend the feedback period beyond 20 days. The time-delay for feedback starts only when the necessary threshold for approval is otherwise met, including there not being any raised objections. If objections are raised and resolved during the time-delay, the waiting period starts again.

The Council is expected to evolve over time to meet the evolving needs of the teams, the Rust Project, and the community. Such evolutionary changes may be small or large in scope and require corresponding amounts of oversight. Changes that materially impact the shape of the Council would need to be part of a public decision process.

As an exception to the above, modifications or removals of a single top-level team (other than the moderation team) may occur with the unanimous agreement of the Council absent the representative delegated by that top-level team.

The Council is permitted to have private *discussions* even on something that ultimately ends up as a public proposal or a publicly disclosed internal decision. The Council may wish to do this if the discussions are sensitive to allow decision participants to speak more frankly and freely. Additionally, in some cases, private information that can't be disclosed may impact an otherwise public decision/proposal; the Council should strive to be as transparent and non-misleading as possible and avoid having opaque decisions where all rationale is private.

Note that all decisions fall into this category unless explicitly designated (via this document or future public proposals) to fall into another category, so this list (unlike those in the other two categories) is intentionally vague/broad: it is intended to give guidance on what likely should belong in this category without necessarily being prescriptive.

- Any decision that has the effect of modifying the list of decision-makers on the Council or the decision-making process of the Council. For instance:
    - Changing this list (or this document in general).
    - Modifying the publication and approval process used for the Council's public proposals. Such a proposal must use the existing established process, not the proposed process.
    - Adding, modifying, or removing policies affecting eligibility for Council representatives.
    - Adding, modifying, or removing one or more top-level teams. This includes:
        - modifying the purview of a top-level team to such an extent that it meaningfully becomes a different team.
        - reorganizing the Project such that top-level teams move underneath other teams.
    - Adding other types of Council representatives other than those delegated by top-level teams.
    - Adding, modifying, or removing policies regarding Council quorums or the locations in which binding decisions can be made.
- Any policy decision, as opposed to a one-off operational decision. (See the [decision-making section][decision-making] for details on policy decisions versus operational decisions.) This includes any decision that binds the decisions of other parts of the Project (e.g. other teams or individuals), effectively serving as an exception to the normal purviews of all teams. Some examples of policy decisions:
    - Modifying or extending existing policies, including those previously made via RFC.
    - A legal/licensing policy affecting Rust Project software or other work of the Rust Project.
    - A change to the Code of Conduct.
    - A policy affecting eligibility for membership in the Rust Project or any team thereof.
    - A change to how the moderation team moderates Council representatives or the Council as a whole. Such decisions must be made jointly with the moderation team.
    - An agreement with another project or organization that makes any ongoing commitments on behalf of the Rust Project. (One-off commitments involving teams that have agreed to those commitments are fine.)
    - Creating or substantially modifying legal structures (e.g. additional Foundations, changing relationship with the Rust Foundation, partnering with other legal entities).
    - Making policy decisions requested by one or more teams that would be within the normal purviews of those teams. (Note that teams can ask for Council input without requesting a Council decision.)
    - Deciding that a class of future decisions always belongs within the Council, rather than being delegated to any other team.
- Any decision that this document or future Council policy specifies as a public policy decision.

## Conflicts of interest
[conflicts-of-interest]: #conflicts-of-interest

A Council representative must not take part in or influence a decision in which they have a conflict of interest.

Potential sources of conflicts of interest include, but are not limited to:
- Personal: a decision about themselves
- Financial: a decision with any substantive financial impact on the representative
- Employment or equivalent: a decision involves another person at the same company, or would benefit/harm that company disproportionately more than others
- Professional or other affiliation: a decision involves an organization the representative is associated with, such as an industry/professional/standards/governmental organization
- Familial/Friendship: a decision about a person the representative cannot be expected to be impartial about, including a conflict of interest of another type through that person (such as a family member's business)

Council representatives must promptly disclose conflicts of interest and recuse themselves from affected decisions. Council representatives must also proactively disclose likely sources of potential conflict annually to other representatives and to the moderation team.

Note that conflicts of interest can arise even if a proposal does not name a specific entity. Council representatives cannot, for instance, use their position to tailor requirements in a proposal to disproportionately benefit their employer.

A proposal favored widely across the Rust community does not automatically represent a conflict of interest for a representative merely because that representative's employer or equivalent also favors the general area of that proposal, as long as the proposal does not favor any particular entities. For example, a proposal to improve the security of a particular Rust component is not a conflict of interest for representatives just because their employers generally care about Rust security; however, a proposal to engage specific developers or security experts, or one's compensation being predicated on such a proposal, might still raise a conflict.

The Council may not waive a conflict of interest if one applies, even if the Council considers it minor. However, the Council may evaluate *whether* a conflict exists at all. Council representatives must raise potential conflicts so that the Council can make such a determination.

The Council may request specific information from a recused representative, and the recused representative may provide that information upon request.

Where possible and practical, the Council should separate decisions to reduce the scope of a conflict of interest. For instance, the Council could separate a decision to arrange access to a class of hardware (without setting specific requirements or selecting vendors) from the decision of which exact hardware to purchase and where to purchase it, if doing so made a conflict of interest only apply to the latter decision.

A representative simultaneously considering the interests of the Rust Project and the interests of any Project team is not necessarily a conflict of interest. In particular, representatives are *expected* to regularly take part in decisions involving their teams, as delegates from those teams.

In the unlikely event that a proposed decision produces a conflict of interest with enough representatives that the remainder cannot meet a previously established quorum requirement, and the decision must still be made, then either top-level teams must provide alternate representatives for the purposes of the specific decision, or (for public decisions only) the Council may elect to proceed with the decision while publicly documenting all conflicts of interest. (Note that proceeding with a public decision, even with conflicts documented, does not actually eliminate the conflicts or prevent them from influencing the decision; it only allows the public to judge whether the conflicts might have influenced the decision. Eliminating the conflicts entirely is always preferable.) In such a case, the Council should consider appropriate processes and policies to avoid future recurrences of a similar conflict.

## Determining and changing team purviews

The Council can move an area or activity between the purviews of top-level teams either already existing or newly created (other than the moderation team). Though the purview of a given top-level team may be further sub-divided by that team, the Council only moves or adjusts top-level purviews. If a sub-divided purview is moved, the Council will work with the involved teams to coordinate the appropriate next steps. This mechanism should be used when the Council believes the existing team's purview is too broad, such that it is not feasible to expect the team to fulfill the full purview under the current structure.  However, this should not happen when a team only *currently* lacks resources to perform part of its duties.

The Council also must approve expansions of a top-level team's purview, and must be notified of reductions in a top-level team's purview. This most often happens when a team self-determines that they wish to expand or reduce their purview. This could also happen as part of top-level teams agreeing to adjust purviews between themselves. Council awareness of changes to a purview is necessary, in part, to ensure that the purview can be re-assigned elsewhere or intentionally left unassigned by the Council.

However, teams (individually or jointly) may further delegate their purviews to subteams without approval from the Council. Top-level teams remain accountable for the full purviews assigned to them, even if they delegate (in other words, teams are responsible for ensuring the delegation is successful).

The Council should favor working with teams on alternative strategies prior to shifting purviews between teams, as this is a relatively heavyweight step. It's also worth noting that one of the use cases for this mechanism is shifting a purview previously delegated to a team that functionally no longer exists (for instance, because no one on the team has time), potentially on a relatively temporary basis until people arrive with the time and ability to re-create that team. This section intentionally does not put constraints on the Council for exactly how (or whether) this consultation should happen.

# Mechanisms for oversight and accountability
[accountability]: #mechanisms-for-oversight-and-accountability

The following are various mechanisms that the Council uses to keep itself and others accountable.

## Ensuring the Council is accountable

The Council must publicly ensure that the wider Project and community's expectations of the Council are consistently being met. This should be done both by adjusting the policies, procedures, and outcomes of the Council as well as education of the Project and community when their expectations are not aligned with the reality.

To achieve this, in addition to rotating representatives and adopting a "public by default" orientation, the Council must regularly (at least on a quarterly basis) provide some sort of widely available public communication on their activities as well as an evaluation of how well the Council is functioning using the list of duties, expectations, and constraints as the criteria for this evaluation.

Each year, the Council must solicit feedback on whether the Council is serving its purpose effectively from all willing and able Project members and openly discuss this feedback in a forum that allows and encourages active participation from all Project members. To do so, the Council and other Project members consult the high-level duties, expectations, and constraints listed in this document and any subsequent revisions thereof to determine if the Council is meeting its duties and obligations.

In addition, it is every representative's *individual* responsibility to watch for, call out, and refuse to go along with failures to follow this document, other Council policies and procedures, or any other aspects of Council accountability. Representatives should strive to actively avoid ["diffusion of responsibility"](https://en.wikipedia.org/wiki/Diffusion_of_responsibility), the phenomenon in which a group of people collectively fail to do something because each individual member (consciously or subconsciously) believes that someone else will do so. The Council may also wish to designate a specific role with the responsibility of handling and monitoring procedural matters, and in particular raising procedural points of order, though others can and should still do so as well.

If any part of the above process comes to the conclusion that the Council is *not* meeting its obligations, then a plan for how the Council will change to better be able to meet their obligations must be presented as soon as possible. This may require an RFC changing charter or similar, a rotation of representatives, or other substantive changes. Any plan should have concrete measures for how the Council and/or Rust governance as a whole will evolve in light of the previous year's experience.

## Ensuring Council representatives are accountable

Council representatives should participate in regular feedback with each other and with their respective top-level team (the nature of which is outside the scope of this document) to reflect on how well they are fulfilling their duties as representatives. The goal of the feedback session is to help representatives better understand how they can better serve the Project. This feedback must be shared with all representatives, all members of the representative's top-level team, and with the moderation team. This feedback should ask for both what representatives have done well and what they could have done better.

Separately, representatives should also be open to private feedback from their teams and fellow representatives at any time, and should regularly engage in self-reflection about their role and efficacy on the Council.

Artifacts from these feedback processes must never be made public to ensure a safe and open process. The Council should also reflect on and adjust the feedback process if the results do not lead to positive change.

If other members of the Council feel that a Council representative is not collaborating well with the rest of the Council, they should talk to that representative, and if necessary to that representative's team. Council representatives should bring in moderation/mediation resources as needed to facilitate those conversations. Moderation can help resolve the issue, and/or determine if the issue is actionable and motivates some level of escalation.

While it is out of scope for this document to specify how individual teams ensure their representatives are held accountable, we encourage teams to use the above mechanisms as inspiration for their own policies and procedures.

## Ensuring teams are accountable

Teams regularly coordinate and cooperate with each other, and have conversations about their needs; under normal circumstances the Council must respect the autonomy of individual teams.

However, the Council serves as a means for teams to jointly hold each other accountable, to one another and to the Project as a whole. The Council can:

- Ask a team to reconsider a decision that failed to take the considerations of other teams or the Project as a whole into consideration.
- Encourage teams to establish processes that more regularly take other teams into consideration.
- Ensure a shared understanding of teams' purviews.
- Ensure teams are willing and able to fulfill those purviews.
- Establish new teams that split a team's purview up into more manageable chunks.

The accountability process must not be punitive, and the process must be done with the active collaboration of the teams in question.

In extreme circumstances where teams are willfully choosing to not act in good faith with regards to the wider Project, the Council has the authority to change a team's purview, move some subset of a team's purview to another team, or remove a team entirely. This is done through the Council's regular decision making process. (This does not apply to the moderation team; see the next section for accountability between the Council and moderation team.)

# Footnotes

[^authority]: The term 'authority' here refers to the powers and responsibilities the Council has to ensure the success of the Rust Project. This document lays out the limits of these powers, so that the Council will delegate the authority it has to teams responsible for the concerns of the Project. These concerns may include - but are not limited to - product vision, day-to-day procedures, engineering decisions, mentoring, and marketing.

[^teams]: Throughout this document, "teams" includes subteams, working groups, project groups, initiatives, and all other forms of official collaboration structures within the Project. "Subteams" includes all forms of collaboration structures that report up through a team.

[^under-multiple-teams]: Subteams or individuals that fall under multiple top-level teams should not get disproportionate representation by having multiple representatives speaking for them on the Council. Whenever a "diamond" structure like this exists anywhere in the organization, the teams involved in that structure should strive to avoid ambiguity or diffusion of responsibility, and ensure people and teams know what paths they should use to raise issues and provide feedback.

[^bootstrapping-new-teams]: The Council consists only of the representatives provided to it by top-level teams, and cannot appoint new ad hoc members to itself. However, if the Council identifies a gap in the project, it can create a new top-level team. In particular, the Council can bootstrap the creation of a team to address a problem for which the Project doesn't currently have coordinated/organized expertise and for which the Council doesn't know the right solution structure to charter a team solving it. In that case, the Council could bring together a team whose purview is to explore the solution-space for that problem, determine the right solution, and to return to the Council with a proposal and charter. That team would then provide a representative to the Council, who can work with the Council on aspects of that problem and solution.

[^number-of-representatives]: This also effectively constrains the number of Council representatives to the same range. Note that this constraint is independently important.

[^representative-selection]: Being a Council representative is ultimately a position of service to the respective team and to the Project as a whole. While we hope that the position is fulfilling and engaging to whomever fills it, we also hope that it is not viewed as a position of status to vie for.

[^council-roles]: The Council is not required to assign such roles exclusively to Council representatives; the Council may appoint any willing Project member. Such roles do not constitute membership in the Council for purposes such as decision-making.

[^infra-creds]: In practice the infrastructure team as a whole does not have access to all credentials and internally strives to meet the principle of least privilege.
