# Moderation, disagreements, and conflicts

This section describes the roles of the Leadership Council and the moderation team in helping resolve disagreements and conflicts, as well as the interactions between those teams.

Disagreements and conflicts fall on a spectrum of interpersonal interaction. Disagreements are more factual and/or technical misalignments, while conflicts are more social or relational roadblocks to collaboration. Many interactions might display aspects of both disagreement and conflict. The Council can help with aspects of disagreement, while aspects of conflict are the purview of the moderation team.

This RFC does not specify moderation policy in general, only the portion of it necessary to specify interactions with the Council and the checks and balances between the Council and the moderation team. General moderation policy is out of scope for this RFC.

Much of the work of the Rust Project involves collaboration with other people, all of whom care deeply about their work. It's normal for people to disagree, and to feel strongly about that disagreement. Disagreement can also be a powerful tool for surfacing and addressing issues, and ideally, people who disagree can collaboratively and (mostly) amicably explore those disagreements without escalating into interpersonal conflicts.

Situations where disagreements and conflicts arise may be complex. Disagreements can escalate into conflicts, and conflicts can de-escalate into disagreements. If the distinction between a disagreement and a conflict is not clear in the situation, or if participants disagree, assume the situation is a conflict.

In the event of a conflict, involved parties should reach out to the moderation team to help resolve the conflict as soon as possible. Time is a critical resource in attempting to resolve a conflict before it gets worse or causes more harm.

## Disagreements among teams

Where possible, teams should attempt to resolve disagreements on their own, with assistance from the Council as needed. The Council can make judgment calls to settle disagreements, but teams need to maintain good working relationships with each other to avoid persistent disagreements or escalations into conflicts.

Potential resolution paths for disagreements between teams could include selecting a previously discussed option, devising a new option, deciding whose purview the decision falls in, or deciding that the decision is outside the purviews of both teams and leaving it to the Council to find a new home for that work.

## Conflicts involving teams or Project members

Conflicts involving teams or Project members should be brought to the moderation team as soon as possible. The Council can help mitigate the impact of those conflicts on pending/urgent decisions, but the moderation team is responsible for helping with conflicts and interpersonal issues, across teams or otherwise.

Individuals or teams may also voluntarily engage in other processes to address conflicts or interpersonal issues, such as non-binding external mediation. Individuals or teams should keep the moderation team in the loop when doing so, and should seek guidance from the moderation team regarding appropriate resources or approaches for doing so. Individuals or teams must not use resources that would produce a conflict of interest.

## Contingent moderators

The moderation team must at all times maintain a publicly documented list of "contingent moderators", who must be approved by both the moderation team and the Council via internal consent decision. The moderation team and contingent moderation team should both consist of at least three members each. The contingent moderators must be:
- Not part of the current moderation team *or* the Leadership Council.
- Widely trusted by Rust Project members as jointly determined by the Council and moderation team; this will often mean they're already part of the Project in some capacity.
- Qualified to do moderation work and [audits] as jointly determined by the Council and moderation team. More detailed criteria and guidelines will be established by moderation policy, which is out of scope for this RFC.
- Willing to serve as contingent moderators: willing to do audits, and willing to do interim moderation work if the moderation team dissolves or becomes unavailable, until they can appoint new full moderators. (The contingent moderators are not expected to be willing to do moderation work long-term.)
- Willing to stay familiar with moderation policy and procedure to the standards expected of a moderation team member (including any associated training). Contingent moderators should receive the same opportunities for training as the moderation team where possible.

The need for contingent moderators arises in a high-tension situation, and the Project and Council must be prepared to trust them to step into that situation. Choosing people known and trusted by the rest of the Project helps lower tensions in that situation.

Moderation is a high-burnout activity, and individual moderators or the moderation team may find itself wishing to step away from that work. Note that one or more individual moderators may always choose to step down, in which case the moderation team should identify and bring in new moderators to fill any gaps or shortfalls; if the moderation team asks a contingent moderator to become a full moderator, the team should then appoint a new contingent moderator. An individual moderator who stepped down *may* be selected as a contingent moderator. If the moderation team as a whole becomes simultaneously unavailable (as determined jointly by the Council and contingent moderators via internal consent decision), or chooses to step down simultaneously, the contingent moderators become the interim moderation team and must promptly appoint new contingent moderators and start seeking new full moderators.

As the contingent moderator role does not have any regular required activities outside of exceptional situations, those appointed to that role must have regular check-ins with the moderation team, to reconfirm that they're still willing to serve in that role, and to avoid a circumstance in which the contingent moderators are abruptly needed and turn out to be unavailable.

## Moderation team policies and procedures

The moderation team has a duty to have robust policies and procedures in place. The Council provides oversight and assistance to ensure that the moderation team has those policies and procedures and that they are sufficiently robust.

The Council may provide feedback to the moderation team and the moderation team is required to consider all feedback received. If the Council feels the moderation team has not followed moderation policies and procedures, the Council may [require an audit][audits] by the contingent moderators. However, the Council may not overrule a moderation decision or policy.

## Audits
[audits]: #audits

If any Council member believes a moderation decision (or series of decisions) has not followed the moderation team's policies and procedures, they should promptly inform the moderation team. The Council and moderation team should then engage with each other, discuss and understand these concerns, and work to address them.

One of the mechanisms this RFC provides for checking the moderation team's actions in a privacy-preserving manner is an audit mechanism. In any case where any Council member believes moderation team actions have not followed documented policies or procedures, the Council member may decide to initiate the audit process. (In particular, they might do this in response to a report from a community member involved in a moderation situation.) This happens *in addition* to the above engagement and conversation; it is not a replacement for direct communication between the Council and the moderation team.

In an audit, the contingent moderation team works with the moderation team to establish whether the moderation team followed documented policies and procedures. This mechanism necessarily involves the contingent moderation team using their own judgment to evaluate moderation policy, specific evidence or communications, and corresponding moderation actions or proposed actions. However, this mechanism is not intended to second-guess the actions themselves; the audit mechanism focuses on establishing whether the moderation team is acting according to its established policy and procedures, as well as highlighting unintended negative consequences of the policies and procedures themselves.

The contingent moderators also reach out to the Council to find out any additional context they might need.

Moderation processes and audits both take time, and must be performed with diligence. However, the Council, contingent moderators, and moderation team should all aim to communicate their concerns and expectations to each other in a reasonably timely fashion and maintain open lines of communication.

Contingent moderators must not take part in decisions or audits for which they have a conflict of interest. Contingent moderators must not have access to private information provided to moderation before the contingent moderator was publicly listed as part of the contingent moderation team; this gives people speaking with the moderation team the opportunity to evaluate potential concerns or conflicts of interest.

The discussions with the Council and the contingent moderation team may discover that the moderation team had to make an exception in policy for a particular case, as there was an unexpected condition in policies or that there was contextual information that couldn't be incorporated in policy. This is an expected scenario that merits additional scrutiny by the contingent moderation team on the rationale for making an exception and the process for deciding the necessity to make an exception, but is not inherently a violation of moderation team responsibilities.

As the audit process and the Council/moderation discussions proceed, the moderation team may decide to alter moderation policies and/or change the outcome of specific moderation decisions or proposed decisions. This is solely a decision for the moderation team to make.

The contingent moderation team must report the results of the audit to the moderation team and the Council for their review. This must not include any details that may reveal private information, either directly or indirectly. Together with the discussions with the moderation team, this should aim to address the concerns of the Council.

## Last-resort accountability

The Leadership Council and moderation team each have substantial power within the Rust Project. This RFC provides many tools by which they can work out conflicts. This section outlines the last-resort mechanisms by which those teams can hold each other accountable. This section is written in the hopes that it will never be needed, and that teams will make every possible effort to resolve conflicts without reaching this point.

If the Council believes there is a systemic problem with the moderation team (whether based on an audit report from the contingent moderation team or otherwise), and the Council and moderation team cannot voluntarily come to agreement on how to address the situation, then as a **last resort**, the Council (by unanimous decision) may simultaneously dissolve itself and the moderation team. The top-level teams must then appoint new representatives to the Council, and the contingent moderation team becomes the new interim moderation team.

Conversely, if the moderation team believes the Council has a systemic problem, and the Council and moderation team cannot voluntarily come to agreement on how to address the situation, then as a **last resort**, the moderation team (by unanimous decision) may simultaneously dissolve itself and the Council. This process can only be enacted if there are at least three moderation team members. The top-level teams must then appoint new representatives to the Council, and the contingent moderation team becomes the new interim moderation team.

The moderation team's representative is recused from the decision to dissolve the Council and moderation team to avoid conflicts of interest, though that representative must still step down as well.

The removed representatives and moderators may not serve on either the Council or the moderation team for at least one year.

By default, the new Council and interim moderation team will take responsibility for clearly communicating the transition.

This mechanism is an absolute last resort. It will almost certainly produce suboptimal outcomes, to say the least. If situations escalate to this outcome, many things have gone *horribly* wrong, and those cleaning up the aftermath should endeavor to prevent it from ever happening again. The indication (by either the moderation team or the Council) that the situation *might* escalate to this point should be considered a strong signal to come to the table and find a way to do "Something Else which is Not That" to avoid the situation.

## Moderation actions involving Project members
[moderation-actions-involving-Project-members]: #moderation-actions-involving-Project-members

The moderation team, in the course of doing moderation work, necessarily requires the ability to take action not just against members of the Rust community but also against members of the Rust Project. Those actions may span the ladder of escalation all the way from a conversation to removal from the Project. This puts the moderation team in a position of power and trust. This RFC seeks to provide appropriate accountability and cross-checks for the moderation team, as well as for the Council.

If the moderation team plans to enact externally visible sanctions against any member of the Rust Project (anything that would create a conspicuous absence, such as removal from a role, or exclusion from participation in a Project space for more than a week), then any party may request that an [audit][audits] take place by reaching out to either the Council or contingent moderators, and that audit will be automatically granted.

For the first year after the ratification of this RFC, audits are automatically performed even without a request, to ensure the process is functional. After that time, the Council and moderation team will jointly review and decide whether to renew this provision.

When the moderation team sends a warning to a Project member, or sends a notification of moderation action regarding a Project member, that message will mention the option of requesting an audit.

Conflicts regarding Project members should be brought to the moderation team as soon as possible.

## Conflicts involving Council representatives

Conflicts involving Council representatives, or alternates, follow the same process as conflicts involving Project members. The moderation team has the same ability to moderate representatives or alternates as any other member of the Project, including the required [audit][audits] by the contingent moderators for any externally visible sanction. This remains subject to the same accountability mechanisms as for other decisions of the moderation team.

In addition to the range of moderation actions already available, the moderation team may take the following additional actions for representatives or alternates as a near-last resort, as a lesser step on the ladder of escalation than removing a member from the Project entirely. These actions are not generally specific to the Council, and apply to other Rust teams as well.

- The moderation team may decide to remove a representative from the Council. The top-level team represented by that representative should delegate a new representative to serve the remainder of the term, starting immediately.
- The moderation team may decide to prevent a Project member from becoming a Council representative.
- The moderation team and Council (excluding the affected parties) may jointly decide (as a private operational consent decision) to apply other sanctions limiting the representative's involvement in the Council. (In this scenario, representatives are not excluded if they have a conflict of interest, as the entire Council will have to cooperate to make the sanctions effective. If the conflicts of interest thus prevent applying these partial sanctions, the moderation team always has the option of full sanctions such as removal.)

All of these also trigger a required audit. The Council must also be notified of any moderation actions involving representatives or alternates, or actions directly preventing people from becoming representatives.

## Conflicts involving moderation team members

Conflicts involving a member of the moderation team will be handled by the remaining members of the moderation team (minus any with a conflict of interest), *together with* the contingent moderation team to provide additional oversight. Any member of the moderation or contingent moderation team should confer with the Council if there is a more systemic issue within the moderation team. The contingent moderators must audit this decision and must provide an audit report to the Council and moderation team.
