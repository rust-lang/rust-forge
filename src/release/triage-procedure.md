# Triage Procedure

## Pull Request Triage

### Status Tags

- [S-waiting-on-author] - Author needs to make changes to address reviewer
  comments, or merge conflicts/test failures are present. This also covers more
  obscure cases, like a PR being blocked on another (usually with the S-blocked
  label in addition), or waiting for a [crater] run -- it is the author's
  responsibility to push the PR forward.

  Also used for work-in-progress PRs, sometimes the PR will also be marked as
  draft in GitHub.
- [S-waiting-on-review] - Review is incomplete
- [S-waiting-on-t-lang] - Waiting for T-lang feedback.
- [S-waiting-on-t-compiler] - Waiting for T-compiler feedback.
- [S-waiting-on-t-libs-api] - Waiting for T-libs-api feedback.
- [S-waiting-on-bors] - Currently approved, waiting to merge. Managed by [bors].
- [S-waiting-on-crater] - Waiting to see what the impact the PR will have on the
  ecosystem
- [S-waiting-on-bikeshed] - Waiting on the consensus over a minor detail
- [S-waiting-on-perf] - Waiting on the results of a perf run
- [S-waiting-on-ACP] - Waiting on API change proposal (ACP)
- [S-blocked] - Waiting for another PR to be merged or for discussion to be
  resolved
- [S-inactive] - Hasn't had activity in a while
- [S-experimental] - An experimental PR that shouldn't be triaged.
  [S-waiting-on-author] used to be used for this, but [S-experimental]
  communicates that the PR is an experiment to test out some changes.

Also: [PRs with no status tags][no-status-tags]. This is useful to find PRs
where rustbot conked out and didn't assign a reviewer and thus didn't assign
[S-waiting-on-review]. These PRs can get lost otherwise. (Note that you should
likely **not** triage PRs that have `r? @ghost` since that means the author does
not need a review yet.)

[s-waiting-on-author]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author+-label%3AI-nominated+-label%3Aneeds-fcp
[s-waiting-on-review]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review+-label%3AI-nominated+-label%3Aneeds-fcp
[s-waiting-on-t-lang]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-t-lang+sort%3Aupdated-asc
[s-waiting-on-t-compiler]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-t-compiler+sort%3Aupdated-asc
[s-waiting-on-t-libs-api]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-t-libs-api+sort%3Aupdated-asc
[s-waiting-on-bors]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bors+sort%3Aupdated-asc
[s-waiting-on-crater]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-crater+sort%3Aupdated-asc
[s-waiting-on-bikeshed]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bikeshed+sort%3Aupdated-asc
[s-waiting-on-perf]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-perf+sort%3Aupdated-asc
[s-waiting-on-acp]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-ACP+sort%3Aupdated-asc
[s-blocked]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-blocked+sort%3Aupdated-asc
[s-inactive]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-inactive+sort%3Aupdated-asc
[s-experimental]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-experimental+sort%3Aupdated-asc
[no-status-tags]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+-label%3AS-waiting-on-author+-label%3AS-waiting-on-review+-label%3AS-waiting-on-team+-label%3AS-waiting-on-bors+-label%3AS-waiting-on-crater+-label%3AS-waiting-on-bikeshed+-label%3AS-waiting-on-perf+-label%3AS-blocked+-label%3AS-inactive+-label%3AS-waiting-on-fcp+-label%3AS-waiting-on-ACP+-label%3AS-experimental
[crater]: https://github.com/rust-lang-nursery/crater
[bors]: https://github.com/rust-lang/homu

### Procedure

We primarily triage three status labels: S-waiting-on-review,
S-waiting-on-author, and (once in a while) S-blocked. Here is the procedure for
each:

#### S-waiting-on-review

Click [this link][S-waiting-on-review] to see all PRs with the
S-waiting-on-review label. Only triage PRs that were last updated 15 days or
more ago (give or take a day).

For each PR:

1. If the PR has new conflicts, CI failed, or a new review has been made then
   change the label to S-waiting-on-author and ping the author.

2. Add the PR to your [report].

#### S-waiting-on-author

Click [this link][S-waiting-on-author] to see all PRs with the
S-waiting-on-author label. Only triage PRs that were last updated 15 days or
more ago (give or take a day).

For each PR:

1. If the author did what the PR was waiting on them for then update the
   label to S-waiting-on-review.

   Otherwise, if the author still needs to do something, then ping the author if
   they are **not** a member of a Rust team (does not include working groups —
   only teams like `T-compiler`, `T-lang`, `T-rustdoc`, etc.).

2. Add the PR to your [report].

#### S-blocked

You only need to check S-blocked PRs occasionally (e.g., once a month).  Click
[this link][S-blocked] to see all PRs with the S-blocked label.

For each PR:

1. If it is still blocked then leave it as-is.

   Otherwise, if it is no longer blocked, then remove S-blocked (and add a
   status label like S-waiting-on-review if appropriate).

2. Add the PR to your [report].

#### Triage Report
[report]: #triage-report

You should record information about each PR you triage in a report. The report
is just a small document that looks like:

> #### S-waiting-on-review
>
> #12345 20 days - still waiting on review - author: ferris, assignee: bors
>
> \[...\]

Your report can look different, just make sure you include this information for
each PR:

1. The PR number (e.g., `#12345`). No need to manually add a link; the Rust
   Zulip will autolink PR (and issue) numbers.

2. Number of days since last activity. "Activity" means:

   - author, reviewer, or team member commented or reviewed; or
   - bors commented about merge conflicts; or
   - PR was pushed to;
   - etc.

3. Author, reviewer, and who or what (person, team, other PR, etc.) the PR is
   waiting on.

4. Current status and what the most recent activity was (e.g., merge conflicts,
   reviewer commented).

Once you are done triaging PRs, post your report in the topic for the current
week's triage in the `#t-release/triage` Zulip stream. the topic should have a
name like `YYYY-MM-DD to YYYY-MM-DD`. Note that this uses a monday-sunday week.

If a topic does not exist, you can generate its title with the following `bash`
one-liner (requires GNU date):

```bash
echo "$(date -I --date="$([ "z$(date +%a)" = "zMon" ] && echo 'today' || echo 'last monday')") to $(date -I --date="$([ "z$(date +%a)" = "zSun" ] && echo 'today' || echo 'next sunday')")"
```

#### Avoiding duplicate work

Since triaging is sometimes done by looking at oldest issues first, re-applying
one of the `S-*` labels will update an issue/PR's last-modified timestamp,
signaling to other triagers that it has already been taken care of.
