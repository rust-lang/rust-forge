There's a steady trickle of patches that need to be ported to the beta
branch. Only a few people are even aware of the process, but this is
actually something anybody can do. Here's how it works:

When somebody identifies a PR that should be backported to beta they
tag it
[`beta-nominated`](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aclosed+label%3Abeta-nominated). That
means they want one of the teams to evaluate whether the patch should
be backported. I also suggest applying the `I-nominated` and and a
`T-` (team) tag as appropriate: that'll _really_ get their
attention. Anybody with triage access is free to make these
tags. Backports are mostly done to fix regressions. If the team thinks
it should be backported they'll then additionally tag it
[`beta-accepted`](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aclosed+label%3Abeta-accepted).

At that point the PR is ready to be backported. So the list of patches
ready for a backport is those tagged [both `beta-nominated` and
`beta-accepted`](https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3Abeta-accepted+is%3Aclosed+label%3Abeta-nominated).

So now somebody needs to go through those PR's and cherry-pick their
commits to the beta branch. Those cherry-picks are then submitted as a
PR _against the beta branch_, with a title started with `beta` (so
reviewers can see its specialness). The OP of that PR should contain
links to all the PRs being backported. [Here's an
example](https://github.com/rust-lang/rust/pull/36634). Anybody can
make these PRs!

After that a reviewer needs to verify that the backport looks correct,
that it's submitted to the beta branch, and then hit the merge
button. Finally, they need to follow the links to the original PRs and
_remove the `beta-nominated` tag_ (people forget to do this a
lot). This last step indicates that the backport has been completed,
so the `beta-nominated` and `beta-accepted` tags have three states.
