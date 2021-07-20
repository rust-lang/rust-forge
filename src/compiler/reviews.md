Review Policy
=============

It is the purpose of code reviews to

- **reduce** the risk of introducing **bugs** and usability and performance **regressions**,
- keep our code **maintainable**, **readable**, and **documented**, and
- ensure that changes are made with the big picture and **appropriate context in mind**.

Reviewing accomplishes this by bringing in another set of eyes
to look at a proposed change from a different perspective,
which increases the chance of catching mistakes early and
uncovering potential blind spots in the reasoning behind the change.


## Basic Reviewing Requirements

There are a number of requirements that need to be met in order for reviewing to be effective:

- Reviewers must have a sufficient **understanding of the code** under review.
    > This is important to help spot non-obvious, unintentional side effects of a given change.
- Pull requests must provide (1) a concise high-level **description of the change** and (2) the **rationale** behind it.
  For the rational to be even more useful, authors are encouraged to list potential **points of contention**.
    > Reviewing code is difficult and reviewers only have a limited amount of time to do it.
    > Jump-starting the review process by not making the reviewer puzzle together the intention and context
    > of a pull request will not only speed things up but also improve the quality of the review.
- Reviewers must have a good idea on whether they are the **right person to approve** the change.
    > Knowledge of the code under review is an obvious but not the only criterium for answering this question.
    > The reviewer also needs to decide if they can make the decision alone or if the PR needs to go through
    > the [major change process][mcp], if they can perform the review in a timely fashion,
    > and if they are impartial enough to provide a sufficiently unbiased perspective.



## Reviewing Checklist

The following list of questions (to be posted as part highfive bot's automatic PR response)
will help reviewers and PR authors alike to bring PRs into good shape and meet the above criteria:

> #### Checklist for PR authors and reviewers
> * Does the PR message have
>    * a concise **high-level description** of the changes? (*what* is being changed)
>    * a clear **rationale** for doing them? (*why* is it being changed)
>    * a list of potential **points of contention**?
>    * **links to relevant issues**, RFCs, MCPs, etc?
> * Does the PR need a **regression test**?
> * Does the change need to be covered by a **[major change proposal][mcp]**? Is it already covered?
> * Would someone trying to understand the PR in a year's time be able to quickly **reconstruct what's going on**?
> * Is the new code **properly documented**? Is existing documentation still up-to-date?
> * Does the PR introduce a **regression** of any of the following:
>   * error message quality
>   * maintainability (e.g. complex code, no documentation, unsafe)
>   * any specific target platforms
>   * downstream tooling (e.g. linkers, debuggers)
>   * compile times
>   * memory usage
>
> #### Checklist for reviewers
> * Am I the **right person to review** this PR:
>   * Do I understand the code well enough?
>   * Would I be able to spot non-obvious side effects?
>   * Would I be able to fix a bug introduced by this PR?
>   * Can I do the review in a timely fashion?
>   * Do I feel pressure to quickly approve the PR for some reason?
>   * Am I impartial enough?
> * Before merging:
>   * Is the **PR message still accurate**?
>   * Is the **commit history clean** enough?



## Guidance for dealing with common situations

In most cases common sense is enough for deciding how to apply this policy.
However, sometimes there are gray areas where it is not immediately clear how to proceed.
This section lists a few common cases together with guidance on how to deal with them.

* ### I don't think I am a good fit for reviewing - what now?
  It is completely normal that you get assigned a PR (via highfive or otherwise) but don't feel comfortable reviewing it.
  Here is what you can do, depending on the concrete case:

  - If the change seems really big or contentious, consider asking for an MCP (see below).
  - If you know just the right person for the review, assign them via `r? @<github-name>`.
    It's polite to leave a comment asking them if they can take over --
    but you don't have to make sure beforehand that they can actually do it.
  - If you don't know the code well or already have too much on your plate,
    ask highfive to roll the dice again via `r? @rust-lang/compiler-contributors`.

  You can also always ask specific compiler team members for help finding a reviewer.

* ### It is unclear if something constitutes a major change
  Deciding if something is a "major change" requiring an [MCP][mcp] is not always straightforward.
  The official guidelines are [here][whats-a-major-change].
  When in doubt, err on side of treating something as a major change.
  You can also nominate the PR for discussion in the compiler team's triage meeting by tagging it `I-nominated`.
  If you nominate a PR please make sure to state a concrete question for the compiler team to discuss.

* ### Intransparent discussion and rationale
  Sometimes there are PRs that seem to be the result of some prior discussion, with no description or rationale.
  They usually have a title like "Change X" and the only content of the PR message is "r? @xyz".
  Even though the change might make sense and may even have been suggested by a compiler team member this is not good form.
  The PR message should give a self-contained description of what is being changed,
  why it is being changed and anything else that might be of interest.
  Try to put yourself in the shoes of someone who, a few years down the road,
  needs to fix a bug related to the code touched by the PR and needs to reconstruct the rationale for the way things are.

* ### Reviewer and PR author report to the same entity / work for the same employer
  There is no rule that prevents two employees of the same company from reviewing each other's PRs.
  The concerns in such a case are no different than for any other two reviewers.
  We expect the mechanisms and principles we articulated above to be respected by ALL reviewers, whatever their employer.
  Does the PR concisely describe the changes that are being made?
  Does it give a clear, transparent rationale for why the changes make sense
  so that contributors down the line can follow the reasoning and reconstruct what's going on?
  Have points of contention been discussed and cleared up?
  Then you are in the clear.

  If you are in doubt if something is contentious, give a heads up to `@rust-lang/compiler-contributors` and ask for another opinion.
  If the proposed change is large and/or potentially has a big impact, create a [major change proposal][mcp].

* ### Reviewing and Mentoring
  In the course of mentoring someone through a PR it often happens that the reviewer has ended up effectively co-writing the changes.
  This can be a tricky case because the reviewer is effectively approving their own changes.
  There are a number of considerations to take into account when deciding how to proceed:

  - If the general direction of the changes has already been approved as part of an MCP and the concrete advice given
    during mentoring was only concerned with resolving minor technical issues, then no further review is required.
  - Similarly, if any contentious decisions have visibly been discussed and resolved on the PR with other
    compiler team contributors and the rest of the changes don't deviate from the general direction that has been
    agreed upon then no further review is required either.
  - If the PR was opened as a response to a concrete suggestion by the reviewer (and the changes are not entirely trivial)
    then it is advisable that the final review is done by someone else.
    However, the initial reviewer/mentor is encouraged to help bring the PR into good shape before handing it off.

  In general, it is advisable to ask for a second opinion by someone knowledgable in the field in such cases,
  just to increase the chance of uncovering oversights and blindspots a mentor might have.

* ### Nobody understands the code that's being changed
  Sometimes there is a bug in some code that nobody understands anymore.
  The original authors are unavailable and it is hard to gauge the implications of a proposed fix.
  In such a case it is a good idea for reviewers to nominate the PR (by tagging it with `I-nominated`)
  in order to get it in front of more eyes during the compiler team's triage meeting.
  In such cases it is also especially valuable to gather and document as much information as possible on the issue,
  such as a description of the problem being fixed, points of unclarity, potential risks,
  alternatives that have been considered, et cetera.
  Reviewers should ask PR authors to add this kind of information as comments in the code
  and/or to the PR message (which will become part of the git commit history).


[mcp]: https://forge.rust-lang.org/compiler/mcp.html
[whats-a-major-change]: https://forge.rust-lang.org/compiler/mcp.html#what-constitutes-a-major-change


## Technical Aspects of Reviewing

Every PR that lands in the compiler and its associated crates must be
reviewed by at least one person who is knowledgeable with the code in
question.

When a PR is opened, you can request a reviewer by including `r?
@username` in the PR description. If you don't do so, rustbot
will automatically assign someone.

It is common to leave a `r? @username` comment at some later point to
request review from someone else. This will also reassign the PR.

### bors

We never merge PRs directly. Instead, we use bors. A qualified
reviewer with bors privileges (e.g., a [compiler
contributor](./membership.md)) will leave a comment like `@bors r+`.
This indicates that they approve the PR.

People with bors privileges may also leave a `@bors r=username`
command. This indicates that the PR was already approved by
`@username`. This is commonly done after rebasing.

Finally, in some cases, PRs can be "delegated" by writing `@bors
delegate+` or `@bors delegate=username`. This will allow the PR author
to approve the PR by issuing `@bors` commands like the ones above
(but this privilege is limited to the single PR).

### Reverts

If a merged PR is found to have caused a meaningful unanticipated regression,
the best policy is to revert it quickly and re-land it later once a fix and
regression test are added.

A "meaningful regression" in this case is up to the judgment of the person
approving the revert. As a rule of thumb, this would be a bug in a stable
or otherwise important feature that causes code to stop compiling, changes
runtime behavior, or triggers a (warn-by-default or higher) lint incorrectly in
real-world code.

When these criteria are in doubt, and especially if real-world code is affected,
revert the PR. This allows bleeding edge users to continue to use and report
bugs on HEAD with a higher degree of certainty about where new bugs are introduced.

Before being reverted, a PR should be shown to cause a regression with a fairly
high degree of certainty (e.g. bisection on commits, or bisection on nightlies
with one or more compiler team members pointing to this PR, or it's simply
obvious to everyone involved). Only revert with lower certainty if the issue is
particularly critical or urgent to fix.

#### Creating reverts

The easiest method for creating a revert is to use the "Revert" button on
Github. This appears next to the "bors merged commit abcd" message on a pull
request, and creates a new pull request.

![Location of the "Revert" button](revert-button.png)

Alternatively, a revert commit can be created using the git CLI and then
uploaded as a pull request:

```terminal
$ git revert -m 1 62d5bee
```

It's polite to tag the author and reviewer of the original PR so they know
what's going on. You can use the following message template:

```
Reverts rust-lang/rust#123456
cc @author @reviewer

This revert is based on the following report of a regression caused by this PR:
<link to issue or comment(s)>

In accordance with the compiler team [revert policy], PRs that cause meaningful
regressions should be reverted and re-landed once the regression has been fixed
(and a regression test has been added, where appropriate).
[revert policy]: https://forge.rust-lang.org/compiler/reviews.html#reverts

Fear not! Regressions happen. Please rest assured that this does not
represent a negative judgment of your contribution or ability to contribute
positively to Rust in the future. We simply want to prioritize keeping existing
use cases working, and keep the compiler more stable for everyone.

r? compiler
```

If you have r+ privileges, you can self-approve a revert.

Generally speaking, reverts should have elevated priority and match the `rollup`
status of the PR they are reverting. If a non-rollup PR is shown to have no
impact on performance, it can be marked `rollup=always`.

#### Forward fixes

Often it is tempting to address a regression by posting a follow-up PR that,
rather than reverting the regressing PR, instead augments the original in
small ways without reverting its changes overall. However, if real-world users
have reported being affected, this practice is strongly discouraged unless one
of the following is true:

* A high-confidence fix is already in the bors queue.
* The regression has made it to a release branch (beta or stable) and a
  [backport] is needed. Often the "smallest possible change" is desired for a
  backport. The offending PR may or may not still be reverted on the main
  branch; this is left to the discretion of someone who can `r+` it.

[backport]: ../release/backporting.md

While it can feel like a significant step backward to have your PR reverted, in
most cases it is much easier to land the PR a second time once a fix can be
confirmed. Allowing a revert to land takes pressure off of you and your
reviewers to act quickly and gives you time to address the issue fully.

### Rollups

All reviewers are strongly encouraged to explicitly mark a PR as to whether or
not it should be part of a [rollup]. This is usually done either when approving a 
PR with `@bors r+ $ROLLUP_STATUS` or with `@bors $ROLLUP_STATUS` where `$ROLLUP_STATUS` 
is substituted with one of the following:

- `rollup=always`: These PRs are very unlikely to break tests or have performance
  implications. Example scenarios:
    - Changes are limited to documentation, comments, etc. that is highly
      unlikely to fail a build.
    - Changes cannot have performance implications.
    - Your PR is not landing possibly-breaking or behavior altering changes.
        - Feature stabilization without other changes is likely fine to
          rollup, though.
    - When in doubt do not use this option!
- `rollup=maybe`: This is the default if `@bors r+` does not specify any rollup 
  status at all. Use this if you have some doubt that the change won't break 
  tests. This can be used if you aren't sure if it should be one of the other 
  categories. Since this is the default, there is usually no need to explicitly 
  specify this, unless you are un-marking the rollup level from a previous command.
- `rollup=iffy`: Use this for mildly risky PRs (more risky than "maybe").
  Example scenarios:
    - The PR is large and non-additive (note: adding 2000 lines of completely
      new tests is fine to rollup).
    - Messes too much with:
        - LLVM or code generation
        - bootstrap or the build system
        - build-manifest
    - Has platform-specific changes that are not checked by the normal PR checks.
    - May be affected by MIR migrate mode.
- `rollup=never`: This should *never* be included in a rollup (**please**
  include a comment explaining why you have chosen this). Example scenarios:
    - May have performance implications.
    - May cause unclear regressions (we would likely want to bisect to this PR
      specifically, as it would be hard to identify as the cause from a
      rollup).
    - Has a high chance of failure.
    - Is otherwise dangerous to rollup.
- `rollup`: this is equivalent to `rollup=always`
- `rollup-`: this is equivalent to `rollup=maybe`

### Priority

Reviewers are encouraged to set one of the rollup statuses listed above
instead of setting priority. Bors automatically sorts based on the rollup
status (never is the highest priority, always is the lowest), and also by PR
age. If you do change the priority, please use your best judgment to balance
fairness with other PRs.

The following is some guidance for setting priorities:

- 1-5
    - P-high issue fixes
    - Toolstate fixes
    - Reverts containing the above
    - Beta-nominated PRs
    - Submodule/Subtree updates
- 5+
    - P-critical issue fixes
- 10+
    - Bitrot-prone PRs (particularly very large ones that touch many files)
    - Urgent PRs
    - Beta backports
- 20+
    - High priority that needs to jump ahead of any rollups
    - Fixes or changes something that has a high risk of being re-broken by
      another PR in the queue.
- 1000
    - Absolutely critical fixes
    - Release promotions

### Expectations for r+

bors privileges are binary: the bot doesn't know which code you are
familiar with and what code you are not. They must therefore be used
with discretion. Do not r+ code that you do not know well -- you can
definitely **review** such code, but try to hand off reviewing to
someone else for the final r+.

Similarly, never issue a `r=username` command unless that person has
done the review, and the code has not changed substantially since the
review was done.  Rebasing is fine, but changes in functionality
typically require re-review (though it's a good idea to try and
highlight what has changed, to help the reviewer).

[rollup]: ../release/rollups.md
