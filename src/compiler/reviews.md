# Review policies

Every PR that lands in the compiler and its associated crates must be
reviewed by at least one person who is knowledgeable with the code in
question. 

When a PR is opened, you can request a reviewer by including `r?
@username` in the PR description. If you don't do so, the highfive bot
will automatically assign someone.

It is common to leave a `r? @username` comment at some later point to
request review from someone else. This will also reassign the PR.

## bors

We never merge PRs directly. Instead, we use bors. A qualified
reviewer with bors privileges (e.g., a [compiler
contributor](./membership.md) will leave a comment like `@bors r+`.
This indicates that they approve the PR.

People with bors privileges may also leave a `@bors r=username`
command. This indicates that the PR was already approved by
`@username`. This is commonly done after rebasing.

Finally, in some cases, PRs can be "delegated" by writing `@bors
delegate+` or `@bors delegate=username`. This will allow the PR author
to approve the PR by issuing `@bors` commands like the ones above
(but this privilege is limited to the single PR).

## High priority issues

When merging high priority issues (`P-critical` and `P-high`) it's
recommended to avoid rollups and bump a bit the priority of the PR in
the homu queue by issuing `@bors r+ rollup=never p=1`.

## Expectations for r+

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
