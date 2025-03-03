# Meetings
The compiler team host various regular meetings to keep on top of regular business necessary for
the running of the team and delivery of a high-quality compiler toolchain.

## Triage Meeting
The weekly triage meeting takes place on Zulip: considering any backports, reviewing
performance triage reports and discussing nominated issues. Triage meetings are held in the
[`#t-compiler/meetings`][meetings_channel] on Zulip. You can find the up-to-date meeting times in
[the team calendar](./calendar.md). Anyone can attend and it is recommended that compiler team
members do.

Agendas of triage meetings are stored on [HackMD][meeting_triage_notes].

### Generating the triage meeting agenda
See [*Prioritization*](./prioritization.md) for documentation on generating the triage meeting
agenda.

## Steering/Planning Meeting
The weekly steering/planning meeting also takes place on Zulip and is intended to host high-level
discussions. Steering/planning meetings operate on a repeating schedule:

- **Week 1:** Planning meeting
  - Select the topics for the next three meetings from the team's proposed meetings.
- **Week 2-4:** Steering meeting
  - Discuss the planned topic.

During planning meetings, the team lead running the meeting will attempt to identify topics which
are relevant for discussion. Some topics may require more investigation before a discussion or may
be out-of-date or more relevant to another team. Depending on the availabilities of the meeting
proposer and relevant team members, the meetings will then be scheduled into the steering meeting
slots of the following weeks. Not all meeting slots need to be scheduled.

Meetings are proposed by opening issues on [the compiler team's repository][team_repo] using the
"meeting proposal" template. Steering meetings are good opportunities to discuss issues with the
wider team that require a decision/vibe check and would take longer than is typically available
when discussing nominated issues in a triage meeting.

It is expected that the proposer of a steering meeting prepare a short and informal document
describing the topic and including all necessary context for the discussion, but this does not
need to be prepared until the day of the meeting, it is not necessary for the initial meeting
proposal.

Any contributor can propose a meeting topic. Some examples of good steering meeting topics include:

- Proposals which require feedback from the team
  - i.e. to refactor subsystems of the compiler or create new out-of-tree dependencies
- In-depth review of large contributions
  - i.e. the author describing and answering questions about their work
- Coordination between the team and the rest of the project
  - i.e. reviewing proposed project goals
- Deciding on a policy for the team
  - i.e. how to handle blockers from external dependencies

Scheduled planning and steering meetings can be found on the [compiler team's
calendar](./calendar.md).

Minutes of steering meetings are stored on [HackMD][meeting_steering_notes].

[team_repo]: https://github.com/rust-lang/compiler-team
[meetings_channel]: https://rust-lang.zulipchat.com/#narrow/channel/238009-t-compiler.2Fmeetings
[meeting_steering_notes]: https://hackmd.io/team/rust-compiler-team?nav=overview&tags=%5B%22steering%22%5D&tagtree-filter=true
[meeting_triage_notes]: https://hackmd.io/team/rust-compiler-team?nav=overview&tags=%5B%22weekly%22%5D&tagtree-filter=true
