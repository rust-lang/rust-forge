## LLM Usage Policy

This is a moderation policy for how LLMs are used in `rust-lang/rust`.
For additional information about the policy itself, see [the appendix](#appendix).

### Overview

Using LLMs while working on `rust-lang/rust` is conditionally allowed, when done with care.
LLMs are not a substitute for thought,
and we do not allow them to be used in ways that risk losing our shared social and technical understanding of the project,
nor in ways that hurt our goals of creating a strong community.

The policy's guidelines are roughly as follows:

> It's fine to use LLMs to answer questions, analyze, distill, refine, check, suggest, review. But not to **create**.

> LLMs work best when used as a tool to write *better*, not *faster*.

> We carve out a space for "experimentation" to inform future revisions to this policy.

#### Moderation vs guidance

This document only contains our moderation policy.
For technical guidance and suggestions, see the [rustc-dev-guide][llm-guidance].

[llm-guidance]: https://rustc-dev-guide.rust-lang.org/llm-guidance.html

### Rules
#### Legend
- ✅ Allowed.
- ❌ Banned.
- ⚠️ Allowed with caveats. Must disclose that an LLM was used.
- ℹ️ Adds additional detail to the policy. These bullets are normative.
- 💡 Indicates that there are suggestions for this bullet in the dev-guide.

#### Non-exhaustive policy

This policy does not aim to be exhaustive.
If you have a use of LLMs in mind that isn't on this list, talk to people about it, and judge it in the spirit of this overview:
- Using an LLM for your own personal use is likely allowed ✅
- Showing LLM output to another human without solicitation is likely banned ❌
- Making a decision based on LLM output requires disclosure ⚠️

#### ✅ Allowed
The following are allowed.
- Any use of an LLM where you are the only one who sees the output. For example:
  - Asking an LLM questions about an existing codebase.
  - Asking an LLM to summarize comments on an issue or PR.
      - ℹ️ This does not allow reposting the summary publicly. This only includes your own personal use.
  - Asking an LLM to privately review your code or prose.
      - ℹ️ This does not apply to public comments by the LLM. See "review bots" under ⚠️ below.
  - Writing dev-tools for your own personal use using an LLM.
  - Using an LLM to generate possible solutions to an issue, learning from them, and then writing something from scratch in your own style.
- Using an LLM in the creation of clearly experimental code changes that are not meant to be reviewed but must live as PRs on `rust-lang/rust` for tooling reasons, such as to run crater or perf.
    - "Clearly experimental" PRs includes things such as `S-experimental` labels, `[PERF]` titles, or `r? ghost` comments.
    - Such PRs should ideally still disclose LLM usage, in case others wish to pick up and build on the experiment. This is a strong recommendation, rather than a requirement, to avoid adding friction to experiments.
    - ℹ️ If a PR is no longer marked as clearly experimental, at that point disclosure is required.

#### ❌ Banned
The following are banned.
- Comments from a personal user account that are originally created by an LLM.
    - ℹ️ This also applies to issue bodies and PR descriptions.
    - ℹ️ This does not apply if the LLM content is clearly quoted and marked, you can post that.
         However, the content of the comment must stand on its own even without the LLM content; it's not a substitute for your own words.
    - ℹ️ See also "machine-translation" in ⚠️ below.
    - ℹ️ See also "Scope" in the appendix below.
- Documentation that is originally created by an LLM.
    - ℹ️ This includes non-trivial source comments, such as doc-comments, safety comments, or multiple paragraphs of non-doc-comments.
    - ℹ️ This includes compiler diagnostics.
         LLMs are conditionally allowed to assist with the *logic* surrounding a diagnostic (see "code changes" under ⚠️ below),
         but they must not be used to create the message itself.
- Policies or processes that are written such that an LLM is required to execute them.
    - For example, you must not *only* document how to take meeting notes with an `AGENTS.md`.
      Documentation must be authored for humans primarily, and LLM documentation may only summarize it, not add new detail.
- Treating an LLM review as a sufficient condition to merge or reject a change.
  LLM reviews, if enabled, **must** be advisory-only.
  Teams can have a policy that code can be merged without review, and they can have a policy that code must be reviewed by at least one person,
  but they may not have a policy that an LLM review substitutes for a human review.
    - ℹ️ See "review bots" in ⚠️ below.
    - ℹ️ An LLM review does not substitute for self-review. Authors are expected to review their own code before posting and after each change.

#### ⚠️ Allowed with caveats
The following are decided on a case-by-case basis.
If you are a new contributor, you should expect to be scrutinized more heavily than existing contributors,
since you haven't yet established trust with your reviewers.

All uses under "⚠️ Allowed with caveats" **must** disclose that an LLM was used.

- Using machine-translation (e.g. Google Translate) from your native language without posting your original message.
  Doing so can introduce new miscommunications that weren't there originally, and prevents someone who speaks the language from providing a better translation.
    - ℹ️ Posting both your original message and the translated version is always ok, but you must still disclose that machine-translation was used.
- "Trivial" code or prose changes.
    - ℹ️ Changes are trivial if there is no other way to write them, or the other ways to write them are nearly identical. For example, the following are all trivial:
        - Typo fixes
        - Markdown links
        - Changing a word to a synonym
        - Type signatures for a trait implementation
    - ℹ️ Be cautious about PRs that consist solely of trivial changes.
      See also [the compiler team's typo fix policy](https://rustc-dev-guide.rust-lang.org/contributing.html#writing-documentation:~:text=Please%20notice%20that%20we%20don%E2%80%99t%20accept%20typography%2Fspellcheck%20fixes%20to%20internal%20documentation).
    - 💡 See the [dev-guide][llm-guidance] for additional suggestions.
    - For more background about concepts that inspired this policy, see
      [threshold of originality](https://fsfe.org/news/2025/news-20250515-01.en.html)
      and [the Google v Oracle ruling](https://en.wikipedia.org/wiki/Google_LLC_v._Oracle_America,_Inc.) that copying API declarations are fair use.
- Using an LLM to discover bugs, as long as you personally verify the bug.
  Please refer to [our guidelines for fuzzers](https://rustc-dev-guide.rust-lang.org/fuzzing.html#guidelines).
    - ℹ️ This also includes reviewers who use LLMs to discover flaws in unmerged code.
    - ℹ️ See also "Comments from a personal user account" under ❌ above.
- Using an LLM as a "review bot" for PRs.
    - ℹ️ Review bots that post without being approved by a maintainer will be banned.
         This is a one-time approval; maintainers don't need to approve individual reviews.
    - ℹ️ Review bots **must** have a separate GitHub account that clearly marks them as an LLM.
      You **must not** post (or allow a tool to post) LLM reviews verbatim on your personal account unless clearly quoted with your own personal interpretation of the bot's analysis.
    - ℹ️ Review bot accounts must be blockable by individual users via the standard GitHub user-blocking mechanism. (Note that some GitHub "app" accounts post comments that look like users but cannot be blocked.)
    - ℹ️ LLM comments **must not** be blocking; reviewers must indicate which comments they want addressed.
        - In other words, reviewers must explicitly endorse an LLM comment before blocking a PR. They are responsible for their own analysis of the LLM's comment and cannot treat it as a CI failure.
    - ℹ️ This does not apply to private use of an LLM for reviews; see ✅ above.
    - 💡 See the [dev-guide][llm-guidance] for additional suggestions.

#### Experiment: LLM-created code changes
We leave space open to experiment with LLMs to inform future policies.
Pre-arranged, non-critical, high-quality, well-tested, and well-reviewed code changes that are originally created by an LLM are allowed, **with disclosure**.

1. "Pre-arranged" means that a reviewer has communicated *ahead of time* that they are willing to review an LLM-created PR.
    - ℹ️ New contributors cannot use an LLM unless they first talk with a reviewer.
          This must be the *same* reviewer who will be assigned to the PR.
2. "Non-critical" means that it is extremely unlikely for the PR to cause a [soundness](https://jacko.io/safety_and_soundness.html) regression.
    - ℹ️ Examples:
      - Changes to internal tooling like `tidy`, `x setup`, and  `linkchecker` are probably ok.
      - Changes that have a strong soundness impact, like the trait system, MIR building, or the query system are probably not ok.
3. "High-quality" means that it is held to at least the same standard as other code changes.
    Everyone reads code, not just the author and reviewer;
    we are not interested in "vibe-coded" PRs that degrade the quality of the codebase.
4. "Well-tested" means that you have covered all edge-cases that either you or the reviewer can think of.
    - ℹ️ LLM-created PRs will be held to a higher standard than human-created PRs, because LLMs make it easier to write tests.
    - ℹ️ If there is no existing test suite for a section of code, you must either write a new test suite or close the PR.
          There are no exceptions for "writing the tests seems hard".
5. "Well-reviewed" means the author and reviewer both commit to fully understanding the code.
    - ℹ️ All review requirements in [our existing review policy](../compiler/reviews.md#basic-reviewing-requirements) still apply.
    - ℹ️ A review from a project member does not substitute for self-review.
          Authors are expected to review their own code before posting and after each change.
    - 💡 See the [dev-guide][llm-guidance] for additional suggestions.

LLM-created PRs must be tagged with a new `ai-assisted` label.
All such PRs will be posted to a new (private) Zulip channel, which will be accessible to all members of the `rust-lang` organization.
The goal of the channel is *not* to act as an additional gate-keeper on LLM-created PRs.
Instead, it's to collect information about *whether this experiment is working*:
Are people doing interesting and useful things with LLMs? Are they learning? Are they making repeat contributions?

Because the new channel is private, it will have higher-than-normal standards for what counts as on-topic.
For example, the following are on-topic:
- Whether a PR meets the criteria for the experiment exception
- Whether a PR follows the policy in general

And the following are off-topic:
- Technical and design discussions. These should be posted directly on the PR or in a public Zulip channel.
- Discussions about effort, communication style, or intent
- General discussions about the LLM policy

#### Circuit breaker

To avoid the risk of LLMs "overwhelming" the codebase, or becoming de-facto required, we set a limit on how many LLM PRs can be merged.
If more than half of PRs in a 6-week window are LLM-authored, we disallow merging new LLM PRs until we go back below 50%, with a minimum pause of 10 days to provide hysteresis and encourage discussion.
(We expect to be able to automate this.)
This window is chosen to align with our existing release cycle.
Such a pause also provides space to discuss the progress of the experiment, potential changes to the experiment parameters or other policy changes, and the sustainability and inclusivity of Rust's AI adoption, in order to avoid excluding contributors who choose not to use LLMs.

## Appendix
### Scope

This policy only applies to `rust-lang/rust`, and only to the teams that have ratified it: compiler, libs, types, rustdoc, bootstrap, and their subteams.
The following are not in scope and are free to set their own policies:
- Other repositories in `rust-lang`
- Submodules, subtrees, and crates.io dependencies
- Teams that have not ratified the policy, such as lang and edition

For example, the following do not fall under the policy:
- Tracking issues for T-lang
- T-lang proposals
- T-lang stabilization reports
- Language documentation
- The style guide
- Names of compiler lints. This only applies to the names themselves; the diagnostic messages are still covered under this policy.
- Direct quotes from any of the above in documentation or diagnostics.

### Motivation and guiding principles

There is not a consensus within the Rust project—and likely never will be—about when/how/where it is acceptable to use AI-based tools.
Many members of the Rust project and community find value in AI;
many others feel that its negative impact on society and the climate are severe enough that no use is acceptable.
Still others are working out their opinion.

Despite these differences, there are many values we all share:

- Building a community of deep experts in our collective projects.
- Building an inclusive community where all feel welcome and respected.

And many facts we agree on:

- Many people find LLM-generated code and writing deeply unpleasant to read or review.
- Many people find LLMs to be a significant aid to learning and discovery.
- LLMs are a new technology, and we are still learning how to use, moderate, and improve them.

With those facts and values in mind, the policy is designed with the following goals:

- Build an intentionally conservative policy that lets us maintain the standard of quality that Rust is known for.
- Limit LLM contributions to the very highest standard of quality, to show that our guideline of "better, not faster" isn't just words.
- Make the policy enforceable and easy to moderate.
- Make the policy consistent and easy to understand and summarize, even for people who haven't read it in detail.
- Avoid making LLMs a requirement to contribute, so that Rust does not become "pay-to-play".

### Moderation policy
#### It's not your job to play detective
["The optimal amount of fraud is not zero"](https://www.bitsaboutmoney.com/archive/optimal-amount-of-fraud/).
Don't try to be the police for whether someone has used an LLM.
If it's clear they've broken the rules, point them to this policy;
if it's borderline, [report it to the mods](https://rust-lang.org/policies/code-of-conduct/) and move on.
You are not required to "actively look" for whether an LLM was involved.

Reporting to moderation is not intended to be a penalty.
The mod team is interested in seeing non-violations as well as violations.
As always, the mod team is free to exercise their own judgement and discretion.

#### Be honest
Conversely, you're expected to be honest about your use of LLMs and the extent of that use.
If you are not sure where something you would like to do falls in this policy, please talk to the [moderation team](mailto:rust-mods@rust-lang.org).
Don't try to hide it.

Deliberately misrepresenting your use of LLMs is not welcome and [may result in moderation action](https://rust-lang.org/policies/code-of-conduct/#moderation).

#### Penalties
The policies marked with a 🔨 follow the same guidelines as the code of conduct:
Violations will first result in a warning, and repeated violations may result in a ban.
- 🔨 Violations of the "Be honest" section

Other violations are left up to the discretion of reviewers and moderators.
For minor violations we recommend telling the author that we can't review the PR until it complies with the policy, with pointers to exactly what they need to do.
For major violations or extractive PRs, we recommend closing the PR or issue.

It is **not** ok to harass a contributor for using an LLM.
All contributors must be treated with respect.
The code-of-conduct applies to *all* conversations in the Rust project.

### Responsibility

Your contributions are your responsibility; you cannot place any blame on an LLM.
- ℹ️ This includes when asking people to address review comments originally created by an LLM. See "review bots" under ⚠️ above.

### The meaning of "originally created by an LLM"

This document uses the phrase "originally created by an LLM" to mean "text that was generated by an LLM (and then possibly edited by a human)".
No amount of editing can change how it was originally created; how it was generated sets the initial style and it's very hard to change once it's set.

For more background about analogous reasoning, see ["What Colour are your bits?"](https://ansuz.sooke.bc.ca/entry/23)

This policy makes no distinction between LLM output that comes from a chat interface and output that comes from editor auto-completion.
In most cases the output is "trivial" (see above under ⚠️), but regardless, it is not treated specially by this policy.

### Conditions for modification or dissolution
This policy is not set in stone, and we can evolve it as we gain more experience working with LLMs.

Minor changes, such as typo fixes, only require a normal PR approval.
Major changes, such as adding a new rule or canceling an existing rule, require a successful MCP (2 approvals and no concerns) from each team that ratified the policy.

This policy can be dissolved in a few ways:

- An accepted FCP by the teams that ratified the policy.
- An objective concern raised about active harm the policy is having on the reputation of Rust, with evidence, as decided by a leadership council FCP.

Changes to the guidance in the rustc-dev-guide have no special requirements for modification.
