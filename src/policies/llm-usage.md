## Policy

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
    - ℹ️ This also includes reviewers who use LLMs to discover bugs in unmerged code.

#### ❌ Banned
The following are banned.
- Comments from a personal user account that are originally authored by an LLM.
    - ℹ️ This also applies to issue bodies and PR descriptions.
    - ℹ️ See also "machine-translation" in ⚠️ below.
- Documentation that is originally authored by an LLM.
    - ℹ️ This includes non-trivial source comments, such as paragraph+ doc-comments or multiple inline paragraphs.
    - ℹ️ This includes compiler diagnostics.
- Code changes that are originally authored by an LLM.
  - This does not include "trivial" changes that do not meet the [threshold of originality](https://fsfe.org/news/2025/news-20250515-01.en.html), which fall under ⚠️ below.
    We understand that while asking an LLM research questions it may, unprompted, suggest small changes where there really isn't another way to write it.
    However, you must still type out the changes yourself; you cannot give the LLM write access to your source code.
    - We do not accept PRs made up solely of trivial changes.
      See [the compiler team's typo fix policy](https://rustc-dev-guide.rust-lang.org/contributing.html#writing-documentation:~:text=Please%20notice%20that%20we%20don%E2%80%99t%20accept%20typography%2Fspellcheck%20fixes%20to%20internal%20documentation).
  - See also "learning from an LLM's solution" in ⚠️ below.

#### ⚠️ Allowed with caveats
The following are decided on a case-by-case basis.
Please avoid them where possible.
In general, existing contributors will be treated more leniently here than new contributors.
We may ask you for the original prompts or design documents that went into the LLM's output;
please have them on-hand, and be available yourself to answer questions about your process.

- Using an LLM to generate a solution to an issue, learning from its solution, and then rewriting it from scratch in your own style.
- Using machine-translation from your native language without posting your original message.
  Doing so can introduce new miscommunications that weren't there originally, and prevents someone who speaks the language from providing a better translation.
    - ℹ️ Posting both your original message and the translated version is always ok, but you must still disclose that machine-translation was used.
    - ℹ️ This policy also applies to non-LLM machine translations such as Google Translate.
- Using an LLM as a "review bot" for PRs.
    - ℹ️ Review bots **must** have a separate GitHub account that marks them as an LLM. They **must not** post under a personal account.
    - ℹ️ Review bots that post without being approved by a maintainer will be banned.
    - ℹ️ If a linter already exists for the language you're writing, we strongly suggest using that linter instead of or in addition to the LLM.
    - ℹ️ Please keep in mind that it's easy for LLM reviews to have false positives or focus on trivialities. We suggest configuring it to the "least chatty" setting you can.
    - ℹ️ LLM comments **must not** be blocking; reviewers must indicate which comments they want addressed. It's ok to require a *response* to each comment but the response can be "the bot's wrong here".
        - In other words, reviewers must explicitly endorse an LLM comment before blocking a PR. They are responsible for their own analysis of the LLM's comment and cannot treat it as a CI failure.
    - ℹ️ This does not apply to private use of an LLM for reviews; see ✅ above.

All of these **must** disclose that an LLM was used.

## Appendix

### No witch hunts
["The optimal amount of fraud is not zero"](https://www.bitsaboutmoney.com/archive/optimal-amount-of-fraud/).
Do not try to be the police for whether someone has used an LLM.
If it's clear they've broken the rules, point them to this policy; if it's borderline, report it to the mods and move on.

Conversely, lying about whether you've used an LLM is an instant code of conduct violation.
If you are not sure where you fall in this policy, please talk to us.
Don't try to hide it.

### Responsibility

All contributions are your responsibility; you cannot place any blame on an LLM.
- ℹ️ This includes when asking people to address review comments originally authored by an LLM. See "review bots" under ⚠️ above.

### "originally authored"

This document uses the phrase "originally authored" to mean "text that was generated by an LLM (and then possibly edited by a human)".
No amount of editing can change authorship; authorship sets the initial style and it is very hard to change once it's set.

For more background about analogous reasoning, see ["What Colour are your bits?"](https://ansuz.sooke.bc.ca/entry/23)

### Non-exhaustive policy

This policy does not aim to be exhaustive.
If you have a use of LLMs in mind that isn't on this list, judge it in the spirit of this overview:
- Usages that do not use LLMs for creation and do not show LLM output to another human are likely allowed ✅ 
- Usages that use LLMs for creation or show LLM output to another human are likely banned ❌
