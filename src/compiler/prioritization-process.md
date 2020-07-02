# Prioritization Process

The prioritization WG has a process that iterates every two weeks at the end of which we fill up a document with [meeting notes](https://hackmd.io/pHb6eTZ2Sjy6XZmwXZHIBA?view) of `T-compiler` working group (TODO: what is the correct cadence of these meetings?).

## A typical triaging workflow

#### 1. Issue gets filed on the [rust-lang repository](https://github.com/rust-lang/rust)

#### 2. Release team applies labels to issues

One or more "area" labels are applied to identify the area of responsability. Area labels are expressed as `A-*` (ex. "A-error-handling", "A-ffi", ... full list [here](https://github.com/rust-lang/rust/labels?q=A-))

One or more "team" labels are applied to assign responsability for that issue to the relevant team(s). Team labels are expressed as `T-*` (ex. "T-cargo", "T-compiler", ... full list [here](https://github.com/rust-lang/rust/labels?q=T-))

If bisection is needed, a [MCVE](https://stackoverflow.com/help/minimal-reproducible-example) is requested as well:

* label `needs-bisection`, `needs-mcve` are added to the issue
* label `ICEBreaker-Cleanup-Crew` is added to the issue (they can help improving the quality of the report, by producing a MCVE or the bisection)

In general, if the scope and impact of the issue is straightforward to understand, it is directly prioritized or sent to the relevant area, otherwise it is  "nominated" for compiler team meeting (that is, appointed for discussion during the meeting).

#### 3. Release team nominates for compiler team to further process (TODO: improve this section)

* also cc folks to bisect
* (usually ICE), usually includes needs-bisection and needs-mcve

#### 4. Compiler team triage group analysis

If the issue is found to be a critical bug, the label [P-critical](priority-levels.html#p-critical) will be applied and a discussion will happen whether the issue need to be delegated to another team.

## The process in detail

Summary of the important moments of the biweekly iteration:

- [Drafting the meeting agenda](#drafting-the-meeting-agenda)
- [Day before the meeting](#day-before-the-meeting)
- [After the meeting](#after-the-meeting)

We check issues for `T-compiler` and `libs-impl` and also stable/beta nominations for `T-rustdoc`.

We are in the process of automating some of our work and we execute some other things in an async way. In particular, some Zulip topics are created when certain things happen to some issue.

## Async process proposal (TODO: what is this section and what to do with it?)

1. The agenda is created as soon as we finish our thursday's weekly meeting.
2. Unprioritized issues: a Zulip topic is created requesting prioritization when an issue is labelled with `I-prioritize`. Regressions and `I-unsound` issues that do not `requires-nightly` are automatizally tagged with `I-prioritize`.
3. stable/beta nominations, `I-nominated` and PRs `S-waiting-on-team`: a Zulip topic is created requesting addition of these items to the agenda.
4. `P-critical` issues and `P-high` regressions: A Zulip topic is created requesting addition of these items to the agenda.
5. Once we do all this, some things from the triagebot script could be removed
6. We probably want the script to integrate with triagebot
   1. we could run `@triagebot prioritization pendings` on wednesdays mornings and have that figuring out what's pending an opening issues or bumping existing ones to notify us what we need to do
   2. we could run `@triagebot prioritization agenda` and have that dumping out the agenda. I'm not 100% sure about this we are going to be filling the agenda as we go. So maybe it's more appropriate to have stuff like `@triagebot prioritization agenda issues-of-note`, `@triagebot prioritization agenda nominations` and things like that so the script could dump parts of the agenda we need to update.

## Drafting the meeting agenda

The draft is currently prepared by running a CLI tool, the [triagebot prioritization script](https://github.com/rust-lang/triagebot). Run it with `cargo run --bin prioritization`. The CLI will output a sequence of steps formatted as a markdown snippet. Each step represents a section of the meeting (TODO: is it?).

Note: In general for issues that not labeled with `T-compiler` or `libs-impl`, we want to first check if these labels are relevant and apply them (TODO: why? what does it mean?)

These steps are:

- [Unprioritized I-prioritize](#1-unprioritized-i-prioritize)
- [Regressions](#2-regressions)
- [Stable/Beta nominations](#3-stablebeta-nominations)
- [Pull requests waiting for team](#4-pull-requests-waiting-for-team)
- [High priority and critical unassigned regressions](#5-p-critical-and-unassigned-p-high-regressions)
- [Agenda](#6-agenda)
- [Final meeting review and announcements](#7-final-review)

Below each step of the Triagebot CLI is explained.

### 1. Unprioritized I-prioritize

We need all `I-prioritize` issues for `T-compiler` and `libs-impl` to be actually prioritized. To do so, we add one of the `P-critical`, `P-high`, `P-medium` or `P-low` labels and remove `I-prioritize` and also add a text like: "Assigning `P-XXX` as [discussed as part of the Prioritization Working Group process](link_to_zulip_conversation) and removing `I-prioritize`."

Note: These lists should typically be empty when we are close to the meeting.

We should:

- Prioritize these issues
- Tag regressions accordingly
- [Notify the appropriate groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
- Nominate the issues that needs some dicussion

### 2. Regressions

We should not have unprioritized regressions and ideally regressions should be assigned to somebody.

#### Stable/Beta/Nightly regressions without P-label

For stable regressions, we are trying to get this list down to zero unprioritized issues right now. The idea is that on every iteration we add `I-prioritize` to 4 issues from this list, so soon we will have zero elements here.

We should:

- Prioritize these issues
- [Notify the appropriate groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
- Nominate the ones worth discussing
- Assign if possible; if it remains unassigned, add it to agenda so we can assign during the meeting

### 3. Stable/Beta nominations

- Add them to the agenda explaining:
  - Why was it nominated
  - Who is assigned
  - Add important details

Issues labeled with `I-nominated` are important issues that we decide deserve discussion during the weekly meeting.

We should:

- Check if they were already discussed and in that case remove `I-nominated` label
- Check if each issue is worth the discussion time
- Add them to the agenda explaining:
  - Why was it nominated
  - Who is assigned
  - Is this an issue or a PR: if an issue, does it have a PR that fixes it?
  - Add important details

### 4. Pull requests waiting for team

These are PRs waiting for some decision by our team (T-compiler or libs-impl).

We should:

- Add them to the agenda explaining:
  - What are they waiting for
  - Add important details
- Explicitly nominate any that you think may be able to be resolved *quickly* in triage meeting.

### 5. P-critical and Unassigned P-high regressions

- [Notify the appropriate groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html)
- Push them forward, if possible
- Assign if possible; if the issue remains unassigned, add it to agenda so we can assign it during the meeting

### 6. Agenda

At this point of the script the agenda is generated.

#### Nominate issues

Check how packed the agenda looks like and if there's room for more, consider important issues:

- [All P-critical](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AP-critical+)
- [T-compiler P-high](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3AP-high+)
  - [unassigned](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3AP-high+no%3Aassignee)
- [libs-impl P-high](https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen+is%3Aissue+label%3Alibs-impl+label%3AP-high+)

### 7. Final review

- Check the compiler calendar to see if there's an outstanding event to announce and add them to the agenda.
- Add Major Changes Proposals

#### Toolstate

TODO: add a short description?

- Check [toolstate](https://rust-lang-nursery.github.io/rust-toolstate/) for outstanding tool breakage.
  - Notify teams in the corresponding channels

#### Performance regressions

- Add Triage Logs to the agenda
  - https://github.com/rust-lang/rustc-perf/tree/master/triage#triage-logs
- Check [perf regressions](http://perf.rust-lang.org/index.html).
  - Notify involved actors.

## Day before the meeting

TODO: improve this section

[Check which working groups should do their check-ins.](https://rust-lang.github.io/compiler-team/about/triage-meeting/)
`YYYY-MM-DD` is the date of the meeting.

A topic on Zulip will be created with the following format

Title:

`[weekly meeting] YYYY-MM-DD #54818` in `#t-compiler/meetings`

Content:

```text
Hi @*T-compiler/meeting*; the triage meeting will be starting in ~ X hours Y minutes
The @*WG-prioritization* have done pre-triage in #**t-compiler/wg-prioritization**
@*WG-prioritization* have prepared the [meeting agenda](link_to_hackmd_agenda)
We will have checkins from @*WG-X* and @*WG-Y*
@**person1** do you have something you want to share about @*WG-X*?
@**person2** do you have something you want to share about @*WG-Y*?
```

## After the meeting

- Remove `I-nominated` tags of already discussed issues
- Add stable/beta-accepted labels or reject nomination for backport nominations
- Close all accepted MCPs
- Start agenda for next meeting using [this template](https://hackmd.io/WQW0yzDDS16YvtBNurmj6A?both)
