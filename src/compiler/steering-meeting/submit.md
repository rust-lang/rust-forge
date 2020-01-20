# Submitting a proposal

If you would like to submit a proposal to the steering meeting for
group discussion, read on! This page has all the details.

## TL;DR

In short, all you have to do is

- [open an issue on the compiler-team repository][ct issues]
  - use the template for meeting proposals
  - you only need a few sentences to start, but by the time the meeting
    takes place we typically expect a more detailed writeup, e.g.
    using [this template][template]

You don't have to have a lot of details to start: just a few sentences
is enough. But, especially for technical design discussions, we will
typically expect that some form of more detailed overview be made
available by the time the meeting takes place.

## Examples of good candidates for discussing at the steering meeting

Here are some examples of possible technical topics that would be
suitable for the steering meeting:

- A working group has an idea to refactor the HIR to make some part of their
  job easier. They have sketched out a proposal and would like feedback.
- Someone has encountered a problem that is really hard to solve with
  the existing data structures. They would like feedback on a good
  solution to their problem.
- Someone has done major refactoring work on a PR and they would like
  to be able to explain the work they did and request review.

Steering meetings are also a good place to discuss other kinds of proposals:

- A proposal to move some part of the compiler into an out-of-tree crate.
- A proposal to start a new working group.

Note that a steering meeting is **not** required to create a new
working group or an out-of-tree crate, but it can be useful if the
proposal is complex or controversial, and you would like a dedicated
time to talk out the plans in more detail.

## Criteria for selection

When deciding the topics for upcoming meetings, we must balance a number of things:

- We don't want to spend time on design work unless there are known
  people who will implement it and support it; this includes not only
  the "main coder" but also a suitable reviewer.
- We don't want to take on "too many" tasks at once, even if there *are* people to
  implement them.
- We also don't want to have active projects that will be "stepping on
  each others' toes", changing the same set of code in deep ways.

## Meetings are not mandatory

It is perfectly acceptable to choose *not* to schedule a particular
slot. This could happen if (e.g.) there are no proposals available or
if nothing seems important enough to discuss at this moment.  Note
that, to keep the "time expectations" under control, we should
generally stick to the same 4-week cycle and simply opt to skip
meetings, rather than (e.g.) planning things at the last minute.

## Adding a proposal

Proposals can be added by opening an issue on the [compiler-team
repository][ct issues]. There is an issue template for meeting
proposals that gives directions. The basic idea is that you open an
issue with a few sentences describing what you would like to talk
about.

Some details that might be useful to include:

* how complex of a topic you think this is
* people in the compiler team that you think should be present for the meeting

### Expectations for the meeting

By the time the meeting takes place, we generally would prefer to have
a more detailed write-up or proposal. You can find a [template] for
such a proposal here. This should be created in the form of a hackmd
document -- usually we will then update this document with the minutes
and consensus from the meeting. The final notes are then stored in the
[minutes] directory of the compiler-team repository.

### Expectations for a non-technical proposal

**The requirements for non-technical proposals are somewhat looser.**  A
few sentences or paragraphs may well suffice, if it is sufficient to
understand the aims of the discussion.

## Frequently asked questions

**What happens if there are not enough proposals?** As noted above,
meetings are not mandatory. If there aren't enough proposals in some
particular iteration, then we can just opt to not discuss anything.

[ct issues]: https://github.com/rust-lang/compiler-team/issues
[template]: https://github.com/rust-lang/compiler-team/blob/master/templates/steering-meeting-proposal.md
[minutes]: https://github.com/rust-lang/compiler-team/tree/master/content/minutes
