# Prioritization WG - Procedure

This document details the procedure the WG-prioritization follows to fill the agenda for the weekly meeting of `T-compiler`.
The working group focuses mainly on triaging `T-compiler` regressions, identifying possibly critical (and thus potential release blocker) issues and building the agenda for the weekly `T-compiler` meeting summarizing the main points to be discussed.

## General issues review process

- Check the status of the issue
- Try moving it forward if possible (ex. stimulate further comments from the issue author / reviewer)
- Ask for more info if it's needed
- Is there an MCVE for the issue already?
- Check if it's a regression and label it accordingly (`regression-*` labels)
- Figure out the area the issue belongs and label it accordingly (`A-*` labels)
- [Ping notify groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html) or relevant teams
- Assign if possible
- Nominate the issue if it's unclear and needs to be discussed

## Generating the T-compiler meeting's agenda

The `T-compiler` agenda is generated from a template (available on [HackMD](https://hackmd.io/WQW0yzDDS16YvtBNurmj6A) or [Github](https://github.com/rust-lang/compiler-team/blob/master/templates/T-compiler%20Meeting%20Agenda%20YYYY-MM-DD.md)). We suggest working the following steps in this order:

### Prepare agenda content

#### 1. Add `T-compiler` labels where appropriate

- [Issues labeled with `I-prioritize`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the stable release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the beta release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues labeled `I-compiler-nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AI-nominated+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui) (i.e. needing a T-compiler discussion)
- [Pull requests waiting on a team's feedback](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AS-waiting-on-team+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues classified with priority `P-high`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AP-high+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)

#### 2. Assign a priority label to issues where needed

Regressions labeled with `I-prioritize` are signaling that a priority assessment is waiting. When this label is added to an issue, the `triagebot` creates automatically a notification for @*WG-prioritization* members on [the Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/245100-t-compiler.2Fwg-prioritization.2Falerts).

To assign a priority, we replace the `I-prioritize` label with one of `P-critical`, `P-high`, `P-medium` or `P-low` and adding a succinct comment to link the Zulip discussion where the issue prioritization occurred, example of a template for the comment:

> WG-prioritization assigning priority ([Zulip discussion](#)).
>
> @rustbot label -I-prioritize +P-XXX

Ideally, we want all [`T-compiler` issues with a `I-prioritize` label](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize) to have a priority assigned, or strive to reach this goal: sometimes different factors are blocking issues from being assigned a priority label, either because the report or the context is unclear or because cannot be reproduced and an MCVE would help. Don't hesitate to ask for clarifications to the issue reporter or ping the `ICEbreaker` team when an ICE ("Internal Compiler Errors") needs a reduction (add a comment on the issue with `@rustbot ping icebreakers-cleanup-crew`)

Keep an eye also on regressions ([stable](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-stable+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc), [beta](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-beta+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc) and [nightly](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-nightly+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc)), ideally they should an assignee.

#### 3. Accept MCPs

An MCP is a [Major Change Proposal](https://forge.rust-lang.org/compiler/mcp.html), in other words a change to the rust compiler that needs a bit more thought and discussion within the compiler team than a pull request. The life cycle of an MCP is described in the documentation. The relevant part for the WG-Prioritization is keeping an eye on them and accept all [MCPs that have been on `final-comment-period`](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+label%3Afinal-comment-period) for 10 or more days.

To accept an MCP, remove `final-comment-period` label, add `major-change-accepted` label and close the issue. A notification to the relevant Zulip topic ([in this stream](https://rust-lang.zulipchat.com/#narrow/stream/233931-t-compiler.2Fmajor-changes)) will be automatically sent by the `triagebot`.

### Generate the meeting's agenda

Run triagebot's CLI to generate the agenda. You need to clone [https://github.com/rust-lang/triagebot](https://github.com/rust-lang/triagebot) (there is no official prepackaged release for this tool) and export two environment variables: `GITHUB_API_TOKEN` and optionally a `GOOGLE_API_KEY` to access a public Google calendar (if this env var is not found, meetings should be manually copy&pasted [from here](https://calendar.google.com/calendar/embed?src=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com)).

To generate the meeting's agenda, run:

```
$ cargo run --bin prioritization-agenda
```

Copy the content of the generated agenda on HackMD. This will be our starting point.

#### Add performance logs

Paste the markdown file of this week [performance triage logs](https://github.com/rust-lang/rustc-perf/tree/master/triage#triage-logs) to the agenda and clean it up a little bit removing emojis (to make the text readable when pasted on Zulip).

### Announce the meeting on Zulip

About two hours before the scheduled meeting, create a new topic on the Zulip stream `#t-compiler/meetings` titled "[weekly] YYYY-MM-DD" using the the following message template:

```text
Hi @*T-compiler/meeting*; the triage meeting will happen tomorrow in about 2 hours.
*WG-prioritization* has done pre-triage in #**t-compiler/wg-prioritization/alerts**
@*WG-prioritization* has prepared the [meeting agenda](link_to_hackmd_agenda)

Working group checkins for today:
- @**WG-foo** by @**person1**
- @**WG-bar** by @**person2**
```

Working Group checkins rotation are generated by a script [at this page](https://rust-lang.github.io/compiler-team/about/triage-meeting) (TODO: script is outdated and could probably be merged into the `triagebot` CLI code).

Checkins about the progress of working groups are not mandatory but we rotate them all to be sure we don't miss on important progresses.

### Add details to the Agenda

#### 1. Summarize stable/beta nominations

These are pull requests that the compiler team *might* want to backport to a release channel. Example a `stable-to-beta-regression` fix might want to be backported to the beta release channel. A `stable-to-stable-regression` fix particularly annoying might warrant a point release (i.e. release a `1.67.1` after a `1.67.0`).

Follow the [General issues review process](#general-issues-review-process).

#### 2. Summarize PRs waiting on team

These are pull requests waiting on a discussion / decision from `T-compiler` (sometimes more than one team).

Try to follow the [General issues review process](#general-issues-review-process). Explicitly nominate any issue that can be *quickly* resolved in a triage meeting.

#### 3. Fill up the "Oldest PRs waiting for review"

This is probably the less automatable part of the agenda (and likely the least fun). The `triagebot` will emit a list of 50 pull requests ordering them by least recent update. The idea is to issue mentions to assigned reviewers during the meeting ensuring that they stay on top of them. We usually try to keep the number of these mentions to around 5 for each meeting.

There are two human factors here to keep in consideration:
- Pull requests reviewers are volunteers, we respect and appreciate their work. We don't want to remind them *too often* that there is a pile of pull requests waiting on them. Therefore we usually wait 2 or 3 weeks before reminding them about that pull requests. It seems like a long time to wait but let's not forget what contributors accomplish in the meanwhile! Anyway, we are trying to find ways to improve on these metrics.
- Contributors taking their time to submit a pull request deserve equally our appreciation so we try to not have them wait *too long* for a review or they will lose context about their work (or motivation to drive the contribution to completion).

Striking a balance between these two diverging forces requires some empathy and "tribal knowledge" that comes with practice. Other factors can be blocking a pull request progress:
- The review is shared with another team (i.e. Team 1 says "OK", now waiting on Team 2)
- The alternating labels `S-waiting-on-review` and `S-waiting-on author` handling the life cycle of a pull request are not promptly applied. A pull request that is ready to be reviewed but it's *not* labeled `S-waiting-on-review` is idling for no purpose.

#### 4. Add some context to `P-critical` and `P-high` regressions without an assignee

Try to follow the [General issues review process](#general-issues-review-process).

#### 5. Summarize I-compiler-nominated issues

Issues labeled with `I-compiler-nominated` generally are nominated to specifically have the compiler team dedicate them a special slice of the meeting (generally towards the end). After the discussion, add a comment on Github linking the Zulip message where the discussion started (so everyone can read). `T-compiler` sometimes writes a summary of the discussion on the issue itself.

Try to follow the [General issues review process](#general-issues-review-process):
- Check if an issue needs a discussion and add the label `I-compiler-nominated`
- When added to the agenda, add some context:
  - Who the assignee is
  - Is this an issue or a pull request: if it's an issue, does it have a pull request that fixes it?
  - Why was it nominated
  - Other important details

### 6. Final review before the meeting

Re-run the triagebot CLI script and update the agenda on HackMD with new data (if any). This is useful when there are last second changes affecting the agenda content.

## Follow-ups after meeting

The meeting is over! Time to cleanup a little bit.

- Lock the agenda file on HackMD assigning write permissions to `Owners`. Download the markdown file and commit [it to this repository](https://github.com/rust-lang/compiler-team/issues?q=is%3Aall+label%3Amajor-change+label%3Ato-announce).

- Remove the `to-announce` label from [MCPs](https://github.com/rust-lang/compiler-team/issues?q=is%3Aall+label%3Amajor-change+label%3Ato-announce), unless this label was added exactly during the meeting (and therefore will be seen during the following meeting).
- Remove `to-announce` FCPs from [rust repo](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Afinished-final-comment-period+label%3Ato-announce), [compiler-team repo](https://github.com/rust-lang/compiler-team/issues?q=is%3Aall+label%3Afinished-final-comment-period+label%3Ato-announce) and [forge repo](https://github.com/rust-lang/rust-forge/issues?q=is%3Aall+label%3Afinished-final-comment-period+label%3Ato-announce), same disclaimer as before.
- Accept or decline [`beta nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted) and [`stable nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted) backports that have been accepted during the meeting. For more info check [T-release backporting docs](https://forge.rust-lang.org/release/backporting.html)
  - To accept a backport, add a `{beta,stable}-accepted` label and keep the `{beta,stable}-nominated` label. Other automated procedures will process these pull requests, it's important to leave both labels. Add a comment on Github linking the Zulip discussion.
  - To decline a backport, simply remove `{beta,stable}-nominated` label. Add a comment on Github explaining why the backport was declined and link the Zulip discussion.
- Remove [`I-compiler-nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AI-nominated+label%3AT-compiler) label from issues that were discussed. Sometimes not all nominated issues are discussed (because of time constraints). In this case the `I-compiler-nominated` will stick until next meeting.
- Create a new agenda stub for the following week using [our template](https://hackmd.io/WQW0yzDDS16YvtBNurmj6A) and post the link on Zulip, so it's available for people if they want to add content during the week.
