## LLM Usage Policy

For additional information about the policy itself, see [the appendix](#appendix).

### Overview

Using an LLM while working on `rust-lang/rust` is conditionally allowed.
However, we find it important to keep the following points in mind:

- Many people find LLM-generated code and writing deeply unpleasant to read or review.
- Many people find LLMs to be a significant aid to learning and discovery.

Therefore, the guidelines are roughly as follows:

> It's fine to use LLMs to answer questions, analyze, distill, refine, check, suggest, review. But not to **create**.

> LLMs work best when used as a tool to write *better*, not *faster*.

#### Legend

- ✅ Allowed
- ❌ Banned
- ⚠️ Allowed with caveats. Must disclose that an LLM was used.
- ℹ️ Adds additional detail to the policy. These bullets are normative.

### Rules

#### ✅ Allowed
The following are allowed.
- Asking an LLM questions about an existing codebase.
- Asking an LLM to summarize comments on an issue, PR, or RFC.
    - ℹ️ This does not allow reposting the summary publicly. This only includes your own personal use.
- Asking an LLM to privately review your code or writing.
    - ℹ️ This does not apply to public comments. See "review bots" under ⚠️ below.
- Writing dev-tools for your own personal use using an LLM, as long as you don't try to merge them into `rust-lang/rust`.
- Using an LLM to discover bugs, as long as you personally verify the bug, write it up yourself, and disclose that an LLM was used.
  Please refer to [our guidelines for fuzzers](https://rustc-dev-guide.rust-lang.org/fuzzing.html#guidelines).
    - ℹ️ This also includes reviewers who use LLMs to discover flaws in unmerged code.

#### ❌ Banned
The following are banned.
- Comments from a personal user account that are originally authored by an LLM.
    - ℹ️ This also applies to issue bodies and PR descriptions.
    - ℹ️ See also "machine-translation" in ⚠️ below.
- Documentation that is originally authored by an LLM.
    - ℹ️ This includes non-trivial source comments, such as doc-comments or multiple paragraphs of non-doc-comments.
    - ℹ️ This includes compiler diagnostics.
- Code changes that are originally authored by an LLM.
    - ℹ️ This does not include "trivial" changes that do not meet the [threshold of originality](https://fsfe.org/news/2025/news-20250515-01.en.html), which fall under ⚠️ below.
    - ℹ️ Be cautious about PRs that consist solely of trivial changes.
      See also [the compiler team's typo fix policy](https://rustc-dev-guide.rust-lang.org/contributing.html#writing-documentation:~:text=Please%20notice%20that%20we%20don%E2%80%99t%20accept%20typography%2Fspellcheck%20fixes%20to%20internal%20documentation).
    - See also "learning from an LLM's solution" in ⚠️ below.
- Treating an LLM review as a sufficient condition to merge or reject a change.
  LLM reviews, if enabled by a team, **must** be advisory-only.
  Teams can have a policy that code can be merged without review, and they can have a policy that code must be reviewed by at least one person,
  but they may not have a policy that an LLM counts as a person.
    - ℹ️ See "review bots" in ⚠️ below.
    - ℹ️ An LLM review does not substitute for self-review. Authors are expected to review their own code before posting and after each change.

#### ⚠️ Allowed with caveats
The following are decided on a case-by-case basis.
Please avoid them where possible.
In general, existing contributors will be treated more leniently here than new contributors,
since they've already established trust with their reviewers.
We may ask you for the original prompts or design documents that went into the LLM's output;
please have them on-hand, and be available to personally answer questions about your process.
We may also ask for the exact LLM model used to generate the output.

- Using an LLM to generate a solution to an issue, learning from its solution, and then rewriting it from scratch in your own style.
- Using machine-translation (e.g. Google Translate) from your native language without posting your original message.
  Doing so can introduce new miscommunications that weren't there originally, and prevents someone who speaks the language from providing a better translation.
    - ℹ️ Posting both your original message and the translated version is always ok, but you must still disclose that machine-translation was used.
- Using an LLM as a "review bot" for PRs.
    - ℹ️ Review bots **must** have a separate GitHub account that marks them as an LLM.
      You **must not** post (or allow a tool to post) LLM reviews verbatim on your personal account unless clearly quoted with your own personal interpretation of the bot's analysis.
    - ℹ️ Review bot accounts must be blockable by individual users via the standard GitHub user-blocking mechanism. (Note that some GitHub "app" accounts post comments that look like users but cannot be blocked.)
    - ℹ️ Review bots that post without being approved by a maintainer will be banned.
    - ℹ️ If a more reliable tool, such as a linter or formatter, already exists for the language you're writing, we strongly suggest using that tool instead of or in addition to the LLM.
    - ℹ️ Configure LLM review tools to reduce false positives and excessive focus on trivialities, as these are common, exhausting failure modes.
    - ℹ️ LLM comments **must not** be blocking; reviewers must indicate which comments they want addressed. It's ok to require a *response* to each comment but the response can be "the bot's wrong here".
        - In other words, reviewers must explicitly endorse an LLM comment before blocking a PR. They are responsible for their own analysis of the LLM's comment and cannot treat it as a CI failure.
    - ℹ️ This does not apply to private use of an LLM for reviews; see ✅ above.

All of these **must** disclose that an LLM was used.

## Appendix

### It's not your job to play detective
["The optimal amount of fraud is not zero"](https://www.bitsaboutmoney.com/archive/optimal-amount-of-fraud/).
Do not try to be the police for whether someone has used an LLM.
If it's clear they've broken the rules, point them to this policy; if it's borderline, report it to the mods and move on.

Conversely, lying about whether you've used an LLM is an instant [code of conduct](https://rust-lang.org/policies/code-of-conduct/) violation.
If you are not sure where something you would like to do falls in this policy, please talk to us.
Don't try to hide it.

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

This policy is not set in stone.
We can evolve it as we gain more experience working with LLMs.
