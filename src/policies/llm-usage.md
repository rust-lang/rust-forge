## LLM Usage Policy

For additional information about the policy itself, see [the appendix](#appendix).

### Overview

Using LLMs while working on `rust-lang/rust` is conditionally allowed, when done with care.
LLMs are not a substitute for thought,
and we do not allow them to be used in ways that risk losing our shared social and technical understanding of the project,
nor in ways that hurt our goals of creating a strong community.

The policy's guidelines are roughly as follows:

> It's fine to use LLMs to answer questions, analyze, distill, refine, check, suggest, review. But not to **create**.

> We carve out a space for "experimentation" to inform future revisions to this policy.

### Rules
#### Legend
- ✅ Allowed
- ❌ Banned
- ⚠️ Allowed with caveats. Must disclose that an LLM was used.
- ℹ️ Adds additional detail to the policy. These bullets are normative.

#### ✅ Allowed
The following are allowed.
- Any use of an LLM where you are the only one who sees the output. For example:
  - Asking an LLM questions about an existing codebase.
  - Asking an LLM to summarize comments on an issue, PR, or RFC.
      - ℹ️ This does not allow reposting the summary publicly. This only includes your own personal use.
  - Asking an LLM to privately review your code or writing.
      - ℹ️ This does not apply to public comments. See "review bots" under ⚠️ below.
  - Writing dev-tools for your own personal use using an LLM.
  - Using an LLM to generate possible solutions to an issue, learning from them, and then writing something from scratch in your own style.
- Syncing code and documentation into `rust-lang/rust` (e.g., using submodules, subtrees, [josh](https://github.com/josh-project/josh), etc.) from other repositories that do not follow this policy.
- Using an LLM in the creation of experimental code changes that are not meant to be reviewed and will never be merged but must live as draft PRs on `rust-lang/rust` for tooling reasons, such as to run crater or perf.

#### ❌ Banned
The following are banned.
- Comments from a personal user account that are originally authored by an LLM.
    - ℹ️ This also applies to issue bodies and PR descriptions.
    - ℹ️ This does not apply if the LLM content is clearly quoted and marked, you can post that.
         However, the content of the comment must stand on its own even without the LLM content; it's not a substitute for your own words.
    - ℹ️ See also "machine-translation" in ⚠️ below.
- Documentation that is originally authored by an LLM.
    - ℹ️ This includes non-trivial source comments, such as doc-comments, safety comments, or multiple paragraphs of non-doc-comments.
    - ℹ️ This includes compiler diagnostics.
         LLMs are conditionally allowed to assist with the *logic* surrounding a diagnostic (see "code changes" under ⚠️ below),
         but they must not be used to author the message itself.
- Treating an LLM review as a sufficient condition to merge or reject a change.
  LLM reviews, if enabled, **must** be advisory-only.
  Teams can have a policy that code can be merged without review, and they can have a policy that code must be reviewed by at least one person,
  but they may not have a policy that an LLM review substitutes for a human review.
    - ℹ️ See "review bots" in ⚠️ below.
    - ℹ️ An LLM review does not substitute for self-review. Authors are expected to review their own code before posting and after each change.

#### ⚠️ Allowed with caveats
The following are decided on a case-by-case basis.
In general, new contributors will be scrutinized more heavily than existing contributors,
since they haven't yet established trust with their reviewers.

- Using machine-translation (e.g. Google Translate) from your native language without posting your original message.
  Doing so can introduce new miscommunications that weren't there originally, and prevents someone who speaks the language from providing a better translation.
    - ℹ️ Posting both your original message and the translated version is always ok, but you must still disclose that machine-translation was used.
- "Trivial" code changes that do not meet the [threshold of originality](https://fsfe.org/news/2025/news-20250515-01.en.html).
    - ℹ️ Be cautious about PRs that consist solely of trivial changes.
      See also [the compiler team's typo fix policy](https://rustc-dev-guide.rust-lang.org/contributing.html#writing-documentation:~:text=Please%20notice%20that%20we%20don%E2%80%99t%20accept%20typography%2Fspellcheck%20fixes%20to%20internal%20documentation).
- Using an LLM to discover bugs, as long as you personally verify the bug, write it up yourself, and disclose that an LLM was used.
  Please refer to [our guidelines for fuzzers](https://rustc-dev-guide.rust-lang.org/fuzzing.html#guidelines).
    - ℹ️ This also includes reviewers who use LLMs to discover flaws in unmerged code.
- Using an LLM as a "review bot" for PRs.
    - ℹ️ Review bots that post without being approved by a maintainer will be banned.
    - ℹ️ Review bots **must** have a separate GitHub account that marks them as an LLM.
      You **must not** post (or allow a tool to post) LLM reviews verbatim on your personal account unless clearly quoted with your own personal interpretation of the bot's analysis.
    - ℹ️ Review bot accounts must be blockable by individual users via the standard GitHub user-blocking mechanism. (Note that some GitHub "app" accounts post comments that look like users but cannot be blocked.)
    - ℹ️ If a more reliable tool, such as a linter or formatter, already exists for the language you're writing, we strongly suggest using that tool instead of or in addition to the LLM.
    - ℹ️ Configure LLM review tools to reduce false positives and excessive focus on trivialities, as these are common, exhausting failure modes.
    - ℹ️ LLM comments **must not** be blocking; reviewers must indicate which comments they want addressed. It's ok to require a *response* to each comment but the response can be "the bot's wrong here".
        - In other words, reviewers must explicitly endorse an LLM comment before blocking a PR. They are responsible for their own analysis of the LLM's comment and cannot treat it as a CI failure.
    - ℹ️ This does not apply to private use of an LLM for reviews; see ✅ above.

All of these **must** disclose that an LLM was used.

#### Experiment: LLM-authored code changes
Solicited, non-critical, high-quality, well-tested, and well-reviewed code changes that are originally authored by an LLM are allowed, with disclosure.
1. "Solicited" means that a reviewer has communicated *ahead of time* that they are willing to review an LLM-authored PR.
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
    - ℹ️ LLM-authored PRs will be held to a higher standard than human-authored PRs, because LLMs make it easier to write tests.
    - ℹ️ If there is no existing test suite for a section of code, you must either write a new test suite or close the PR.
          There are no exceptions for "writing the tests seems hard".
5. "Well-reviewed" means the author and reviewer both commit to fully understanding the code.
    - ℹ️ All review requirements in [our existing review policy](../compiler/reviews.md#basic-reviewing-requirements) still apply.
    - ℹ️ A review from a project member does not substitute for self-review.
          Authors are expected to review their own code before posting and after each change.
    - ℹ️ We recommend, but do not require, using a second LLM for adversarial local review before publishing your changes.

LLM-authored PRs must be tagged with a new `ai-assisted` label.
All such PRs will be posted to a new (private) Zulip channel, which will be accessible to all members of the `rust-lang` organization.
The goal of the channel is *not* to act as an additional gate-keeper on LLM-authored PRs.
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
## Appendix
### Motivation and guiding principles

There is not a consensus within the Rust project—and likely never will be—about when/how/where it is acceptable to use AI-based tools.
Many members of the Rust project and community find value in AI;
many others feel that its negative impact on society and the climate are severe enough that no use is acceptable.
Still others are working out their opinion.

Despite these differences, there are many common goals we all share:

- Building a community of deep experts in our collective projects.
- Building an inclusive community where all feel welcome and respected.

To achieve those goals, this policy is designed with the following points in mind:

- Many people find LLM-generated code and writing deeply unpleasant to read or review.
- Many people find LLMs to be a significant aid to learning and discovery.
- LLMs are a new technology, and we are still learning how to use, moderate, and improve them.
  Since we're still learning, we have chosen an intentionally conservative policy that lets us maintain the standard of quality that Rust is known for;
  but leave space open to experiment with LLMs to inform future policies.


### Moderation policy
#### It's not your job to play detective
["The optimal amount of fraud is not zero"](https://www.bitsaboutmoney.com/archive/optimal-amount-of-fraud/).
Don't try to be the police for whether someone has used an LLM.
If it's clear they've broken the rules, point them to this policy; if it's borderline, report it to the mods and move on.

#### Be honest
Conversely, lying about whether or how you've used an LLM is considered a [code of conduct](https://rust-lang.org/policies/code-of-conduct/) violation.
If you are not sure where something you would like to do falls in this policy, please talk to the [moderation team](mailto:rust-mods@rust-lang.org).
Don't try to hide it.

#### Penalties
The policies marked with a 🔨 follow the same guidelines as the code of conduct:
Violations will first result in a warning, and repeated violations may result in a ban.
- 🔨 Violations of the "Be honest" section

Other violations are left up to the discretion of reviewers and moderators.
For minor violations we recommend telling the author that we can't review the PR until it complies with the policy, with pointers to exactly what they need to do.
For major violations or extractive PRs, we recommend closing the PR or issue.

Using an LLM does **not** mean it's ok to harrass a contributor.
All contributors must be treated with respect.
The code-of-conduct applies to *all* conversations in the Rust project.

### Responsibility

Your contributions are your responsibility; you cannot place any blame on an LLM.
- ℹ️ This includes when asking people to address review comments originally authored by an LLM. See "review bots" under ⚠️ above.

### The meaning of "originally authored"

This document uses the phrase "originally authored" to mean "text that was generated by an LLM (and then possibly edited by a human)".
No amount of editing can change authorship; authorship sets the initial style and it is very hard to change once it's set.

For more background about analogous reasoning, see ["What Colour are your bits?"](https://ansuz.sooke.bc.ca/entry/23)

### Non-exhaustive policy

This policy does not aim to be exhaustive.
If you have a use of LLMs in mind that isn't on this list, judge it in the spirit of this overview:
- Usages that do not use LLMs for creation and do not show LLM output to another human are likely allowed ✅ 
- Usages that use LLMs for creation or show LLM output to another human are likely banned ❌

### Conditions for modification or dissolution
This policy is not set in stone, and we can evolve it as we gain more experience working with LLMs.

Minor changes, such as typo fixes, only require a normal PR approval.
Major changes, such as adding a new rule or cancelling an existing rule, require
a simple majority of members of teams using rust-lang/rust (without concerns).

This policy can be dissolved in a few ways:

- An accepted FCP by teams using rust-lang/rust.
- An objective concern raised about active harm the policy is having on the reputation of Rust, with evidence, as decided by a leadership council FCP.
