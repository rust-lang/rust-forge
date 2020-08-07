# Prioritization WG - Procedure

This document details the procedure the WG-prioritization follows to fill the agenda for the weekly meeting of `T-compiler`.
The working group focuses mainly on triaging `T-compiler` and `libs-impl` bugs, deciding if bugs are critical (potential release blockers) or not and building the agenda for the most important things `T-compiler` needs to discuss.

## General issues review process

- Check the status of the issue
- Try moving it forward if possible (ex. stimulate further comments from the issue author or a reviewer)
- Ask for more info if it's needed
- Is there an MCVE for the issue already?
- Check if it's a regression and label it accordingly (`regression-*` labels)
- Figure out the area the issue belongs and label it accordingly (`A-*` labels)
- [Ping notify groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html) or relevant teams
- Assign if possible
- Nominate the issue if it needs to be discussed

## The procedure in detail

High level overview:

- [Follow ups from previous meeting](#follow-ups-from-previous-meeting)
  - Remove `I-nominated` tags of already discussed issues
  - Notify @pnkfelix about not properly tagged stable/beta-nominated issues
  - Create the next meeting agenda using the weekly agenda template
- [Prepare agenda content](#prepare-agenda-content)
  - Add `T-compiler` and `libs-impl` to unlabelled T-compiler and libs-impl issues
  - Assign priority to unprioritized issues with "I-prioritize" label
  - Process MCPs/FCPs
- [Generate Agenda](#generate-agenda)
  - Run cli to generate agenda
  - Fill agenda announcements
  - Add performance logs
- [Notify the team about the meeting](#notify-the-team-about-the-meeting)
  - Figure out which WGs need to check-in
  - Notify @T-compiler/meeting about the meeting on Zulip
- [Add details to the Agenda](#add-details-to-the-agenda)
  - Summarize stable/beta nominations
  - Summarize PR's waiting on team
  - Summarize `P-critical` and unassigned `P-high` regressions
  - Summarize I-nominated issues
- [Final reviews](#final-reviews)
  - Check toolstate
  - Check performance stats
  - Nominate issues
  - Re-sync and check the agenda right before the meeting

### Follow ups from previous meeting

- Remove [`I-nominated`](https://github.com/rust-lang/rust/labels/I-nominated) tags of already discussed issues. For that check previous week agenda and Zulip meeting
- Notify [@pnkfelix](https://rust-lang.zulipchat.com/#narrow/pm-with/116083-user116083) about accepted [`beta nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted) and [`stable nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted) without `beta-accepted` and `stable-accepted` label. For that compare these issues with list meeting's accepted nominations.
- Notify @pnkfelix about rejected [`beta nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted) and [`stable nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted) still with the nominated label. For that compare these issues with last meeting's rejected nominations.
- Create an empty agenda using [our template](https://hackmd.io/WQW0yzDDS16YvtBNurmj6A), as soon as our Thursday's weekly meeting ends. After creating the meeting change document permissions to Write -> Owners.

### Prepare agenda content

#### Add `T-compiler` and `libs-impl` labels

Add `T-compiler` and `libs-impl` labels to corresponding issues that are missing these labels.

- [All unprioritized I-prioritize](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize)
- [All stable nominations](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted)
- [All beta nominations](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted)
- [All I-nominated](https://github.com/rust-lang/rust/labels/I-nominated)
- [All PR's waiting on team](https://github.com/rust-lang/rust/labels/S-waiting-on-team)

#### Assign priority to unprioritized issues with "I-prioritize" label

We need all [`I-prioritize T-compiler`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize) and all [`I-prioritize libs-impl`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3Alibs-impl+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize) to be actually prioritized. To do so, we add one of the `P-critical`, `P-high`, `P-medium` or `P-low` labels and remove `I-prioritize` and also add a text such as:

> Assigning `P-XXX` as [discussed as part of the Prioritization Working Group procedure](link_to_zulip_conversation) and removing `I-prioritize`.

The procedure here follows the [General issues review process](#General-issues-review-process).

Note: triagebot automatically creates a topic and notify @*WG-prioritization* members once an issue is labelled with `I-prioritize`

Note #2: These lists should typically be empty when we are close to the meeting.

Note #3: we should not have unprioritized regressions ([stable](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-stable+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-release+-label%3AT-rustdoc), [beta](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-beta+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-release+-label%3AT-rustdoc) and [nightly](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-nightly+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-release+-label%3AT-rustdoc)) and ideally regressions should have an assignee.

#### Accept MCPs

Accept all [MCPs that have been on `final-comment-period`](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+label%3Afinal-comment-period) for 10 or more days. Basically check that `final-comment-period` label was added more than 10 days ago.
To accept, remove `final-comment-period`, add `major-change-accepted` and close the issue.

### Generate Agenda

#### Run cli to generate agenda

Run triagebot's prioritization cli to generate the agenda.
For that you need to clone https://github.com/rust-lang/triagebot if you haven't done so already.
You need to export your `GITHUB_API_TOKEN` on Linux that's typically done by adding

`export GITHUB_API_TOKEN=<your key>`

to your `~/.profile` file.

And then run:

```
$ cargo run --bin prioritization
```

Copy the content of the generated agenda into the agenda on HackMD.

#### Remove `to-announce` from MCPs/FCPs

As a quick reminder:

MCP = Major Change Proposal
FCP = Final Comment Period

Remove all [`to-announce` MCPs](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aall+label%3Amajor-change+label%3Ato-announce) as they were already added in the agenda in their corresponding place.

FIXME: We need to add `to-announce` also to FCPs and here we would need to also remove the [FCPs `to-announce`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aall+label%3Afinished-final-comment-period+label%3Ato-announce).
For now fix announcements output manually. Remove the nonsense no fcps kind of lines when there's content and remove old Finalized FCPs.

#### Fill agenda announcements

Check the compiler calendar to see if there's an outstanding event to announce and add it to the agenda.

#### Add performance logs

Add [Triage Logs](https://github.com/rust-lang/rustc-perf/tree/master/triage#triage-logs) to the agenda.

### Notify the team about the meeting

[Figure out which working groups' check-ins follow](https://rust-lang.github.io/compiler-team/about/triage-meeting/).
Create `[weekly meeting] YYYY-MM-DD #54818` topic in `#t-compiler/meetings` Zulip's stream and send the following messages:

```text
Hi @*T-compiler/meeting*; the triage meeting will happen tomorrow at 2pm UTC
The @*WG-prioritization* have done pre-triage in #**t-compiler/wg-prioritization/alerts**
@*WG-prioritization* have prepared the [meeting agenda](link_to_hackmd_agenda)
We will have checkins from @*WG-X* and @*WG-Y*
@**person1** do you have something you want to share about @*WG-X*?
@**person2** do you have something you want to share about @*WG-Y*?
```

### Add details to the Agenda

#### Summarize stable/beta nominations

- Add them to the agenda explaining:
  - Who the author of the PR is
  - Who the assignee is
  - Which issue fixes, it is a regression? what's the priority?
  - Why was it nominated
  - Add other important details

Note: triagebot automatically creates a topic and notify @*WG-prioritization* members requesting addition to the agenda.

#### Summarize PR's waiting on team

These are PRs waiting for some decision by our team (`T-compiler` or `libs-impl`).

Try to follow the [General issues review process](#general-issues-review-process).

We should:

- Add them to the agenda explaining:
  - Who is the author of the PR
  - Who is the assignee
  - What is the issue waiting for
  - Add other important details
- Explicitly nominate any issue that can be *quickly* resolved in a triage meeting.

Note: triagebot automatically creates a topic and notify @*WG-prioritization* members requesting addition to the agenda.

#### Summarize `P-critical` and unassigned `P-high` regressions

Try to follow the [General issues review process](#general-issues-review-process).

We should:

- [Notify the appropriate groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
- Push them forward, if possible
- Assign if possible

- Add `P-critical`s and unassigned `P-high`s to the agenda explaining:
  - If it's assigned or not and to whom
  - Does it needs MCVE and/or bisection?
  - Are there identified culprits?
  - Do already have a PR open that fixes the issue?
  - Add other important details

Note: triagebot automatically creates a topic and notify @*WG-prioritization* members requesting addition to the agenda.

#### Summarize I-nominated issues

Issues labeled with `I-nominated` are important issues that we decide deserve discussion during the weekly meeting.

Try to follow the [General issues review process](#general-issues-review-process).

We should:

- Check if these issues were already discussed and in that case remove `I-nominated` label
- Check if each issue is worth being discussed
- Add them to the agenda explaining:
  - Who is the assignee
  - Is this an issue or a PR: if an issue, does it have a PR that fixes it?
  - Why was it nominated
  - Add other important details

Note: triagebot automatically creates a topic and notify @*WG-prioritization* members requesting addition to the agenda.

### Final reviews

#### Check toolstate

Check [toolstate](https://rust-lang-nursery.github.io/rust-toolstate/) for tool breakage and notify teams in the corresponding channels.

#### Check performance stats

Check [perf regressions](http://perf.rust-lang.org/index.html) and notify involved actors.

#### Nominate P-high issues

Check how packed the agenda looks like and if there's room for more nominations.

- [Other team's P-critical](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AP-critical+-label%3AT-compiler+-label%3Alibs-impl)
- [T-compiler P-high](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3AP-high+)
- [libs-impl P-high](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3Alibs-impl+label%3AP-high+)

#### Re-sync and check the agenda right before the meeting

Re-run the script and re-synchronize contents of the agenda with new information.
