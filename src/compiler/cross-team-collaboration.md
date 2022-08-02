# Cross-team Collaboration
If you are a member of another team and would like to raise an issue with the
compiler team..

## ..for discussion
Write a comment on a GitHub issue describing the reason for the nomination
(i.e. what decision needs to be made/what opinion is sought; what are the
relevant parts to the compiler team, etc) and add the `I-compiler-nominated`
label to a issue (you can include `@rustbot label +I-compiler-nominated` in
your comment to do this).

Once nominated, the issue will be discussed in a upcoming [triage
meeting](./triage-meeting.html). The compiler team doesn't always get through
all nominated issues each week, so it can take more than one meeting for your
issue to be discussed.

Once discussed, a member of the team will comment on the issue with the
conclusion of the discussion and linking to the relevant Zulip chat.

## ..to be fixed
If there is an existing working relationship between a member of the requesting
team and a contributor to the compiler, then the first option that a team has
for requesting tasks be completed is to ping that contributor and ask if they
can complete the task. It is recommended that pings take place in public Zulip
channels so that..

- ..other contributors that have free time have the opportunity to
  offer their help.
- ..other compiler team members/leadership can ensure that requests being made
  are reasonable (see the rest of this section for the types of issues that the
  compiler team commits to prioritizing on behalf of other teams).

It is worth considering the available bandwidth of the contributor that the
request is being made of, and whether their areas of expertise in the compiler
are relevant.

When there is not a appropriate contact in the compiler team to reach out to
directly, write a comment on a GitHub issue (or create an issue) describing the
task that needs completed. Teams should nominate issues for the compiler team
when issues..

- ..are not already tracked by/part of an existing initiative or working group
  and..
- ..are blocking/impeding the work of the other team (e.g. a feature or bug
  preventing the stabilization of something otherwise complete), but..
- ..aren't absolutely mission-critical - a soundness bug or otherwise critical
  issue will be prioritized by the [prioritization working
  group](./prioritization.html) and addressed through the compiler team's other
  processes for these bugs. If the issue lacks a prioritization label, you can
  add the `I-prioritize` label and it will be enqueued for prioritization.

A detailed description of the feature being requested or the bug to be fixed is
helpful wherever possible (so that the compiler contributor does not need to
make a guess as to a solution that would solve the problem for the requesting
team). If a member of the requesting team isn't explicitly listed as the
point-of-contact for the issue, then the author of the comment will be assumed
to be the point-of-contact.

Add the `I-compiler-nominated` label to a issue (you can use `@rustbot label
+I-compiler-nominated` to do this).

Once nominated, the issue will be discussed in a upcoming [triage
meeting](./triage-meeting.html). The compiler team doesn't always get through
all nominated issues each week, so it can take more than one meeting for your
issue to be discussed. In the compiler team's discussion, the issue may..

- ..be accepted, in which case it will be assigned to a contributor and the
  nomination label removed. Once assigned, a member of the team will work on
  the issue. If no work is completed after a reasonable time, then re-nominate
  the issue and the compiler team will find someone else to complete the work.
- ..or not accepted (e.g. due to insufficient bandwidth, other
  critical/high-priority bugs, being unable to find an appropriate contributor,
  or the issue lacking feasibility). In this case, the compiler team will reply
  to the nomination with an explanation and will remove the nomination label.
