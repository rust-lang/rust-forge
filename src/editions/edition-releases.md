# Edition Releases

This document gives an overview of how to manage an Edition release.
This assumes that you are familiar with what Editions are (see the [Edition Guide]).

[Edition Guide]: https://doc.rust-lang.org/edition-guide/

## Edition project group

[RFC 3501] established that the Leadership Council is responsible for forming a project group who takes responsibility for managing the Edition.

### Project group leads

It is recommended that the project group have 2-3 leads, to make it easier to coordinate and quickly make decisions and actions that will be needed within a relatively tight timeline.

The other consideration is commitment and time availability.
Unlike most of the work we do in Rust, Editions have a fixed timeline.
This requires a different sort of commitment.
Ordinarily our system is pretty tolerant of people coming and going or experiencing unexpected delays.
There is less room for that with the Edition.
Leads should be able to commit to meeting regularly and following up on action items in a timely fashion.
(Of course, things happen, people take vacations, whatever; we'll deal and make it work. But you get the idea.)

Another thing to consider is that the time requirements vary significantly from month to month.
There may be some months where there is nothing to do, and a few where there is a fairly high time requirement (10+ hours in a week).
This is also highly variable based on how many changes are in an Edition.

### Project group members

Additional members of the Edition Project Group can help with shorter-term action items,
or to help with specific aspects of the process (such as writing documentation, implementing migration lints, fixing bugs, authoring progress updates and blog posts, etc.).

## Phases

Running an Edition involves many steps, coordinated across the project.

1. Preparation phase.
   This is the time approximately 1-3 years before the Editions ships that involves all the preparation work.
   The sooner these tasks can be performed, the better.
    1. Preliminary support for the next edition should be added to tools.
       It might be nice to make this automated in the future. Examples:
        - [`rustc` support for `--edition` flag](https://github.com/rust-lang/rust/pull/94461)
        - [`cargo` support for `edition` setting](https://github.com/rust-lang/cargo/pull/12771)
        - [`rustfmt` support for `--edition` flag](https://github.com/rust-lang/rust/pull/94461)
        - [`clippy` support for `--edition` flag](https://github.com/rust-lang/rust/pull/94461)
        - [`rust-analyzer` support for editions](https://github.com/rust-lang/rust-analyzer/pull/7123)
    1. Teams start their proposals and implementation work.
1. The Leadership Council sets up a Project Group to run the edition (approximately 1 year before the final release).
1. Final deadline phase.
   This is the period starting about one year before the Edition release.
   This starts the series of final deadlines for anything to be added to the Edition.
   See [Sample timeline](#sample-timeline) below for the set of deadlines during the course of the year.

### Feature phases

Each feature goes through a series of phases.
This process can start at any time.
The process can take a highly variable length of time, sometimes completing very quickly and sometimes taking many years.

1. Individuals and teams propose an Edition change.
   The exact process will vary by team, but a common way to start is to post a Pre-RFC on [IRLO].
1. An RFC should be posted with the proposed Edition change.
1. The team accepts the RFC.
    This indicates that the team *wants* the idea in principle, but does not guarantee that it will make it in time for a specific Edition.
1. Either the RFC or the team should put together a migration plan that defines how migrations will be handled from the previous edition.
    It's OK for some kinds of breakage to require people to make manual edits to the code, but that has to be rare, and ideally it should be noisy (i.e., people will get compilation errors, not surprising semantics at runtime).
    **It's up to the edition leads to make the call on what is "rare enough".**
1. Implement the feature and migration support.
1. Informal testing should happen on nightly by people most interested in the feature.
   Issues should be identified and fixed during this time.
1. The team responsible for the feature should make a final call if the feature is ready for the next edition by the feature cutoff date.
   This should be done in conjunction with the Edition Project Group.
1. Document the change in places such as the [Edition Guide] and [the Reference].

[IRLO]: https://internals.rust-lang.org/
[the Reference]: https://doc.rust-lang.org/reference/

### Sample timeline

The following is a sample set of deadlines of the Edition.

This is shown as milestones relative to the release.
Previous Edition releases have hit stable late in the year (October), but it is highly encouraged for future Editions to release earlier in the year, such as June.

These dates are not very fixed (for example, Rust releases on a 6 week cadence, so the exact release date shuffles around), and the Edition Project Group should adjust these as desired.

- 1-3 years before the Edition release date
    - Teams should be planning and implementing their Edition changes.
- T-11 months
    - Leadership Council ensures the Edition Project Group is formed and ready.
    - Blog post announcing the Edition schedule.
    - Edition Project Group should start coordinating with teams for their list of changes, and set up a tracking tool to track the changes.
    - Tools should have preliminary support for the next edition (ideally this should be done soon after the previous Edition).
    - Public blog post calling for the final list of features, and to communicate the final deadlines. [Example][example-blog-edition-final-call]
- T-10 months
    - Last chance for Pre-RFC proposals.
- T-9 months
    - Last chance for RFC approvals.
- T-8 months
    - Final list of Edition changes is complete, all RFCs approved.
    - Public blog post informing what is included in the Edition. [Example][example-blog-edition-plan]
- T-7 months
- T-6 months
- T-5 months
    - All features and migrations implemented on nightly. All feature gates should be removed.
- T-4 months
    - Crater test all migrations (see [Crater migration test](#crater-migration-test) below).
    - Edition Project Group should be tracking all edition issues to ensure they get resolved in time.
- T-3 months
    - Public blog post calling for final testing on nightly. [Example][example-blog-public-test]
- T-2 months
    - Most issues have been fixed.
    - Documentation finished (the [Edition Guide], [the Reference], etc.).
    - The Edition Project Group should make a final go/no-go decision on stabilizing the Edition, or if it has to be delayed to the next release.
    - Edition is stabilized on nightly for all tools (rustc, cargo, etc.).
- T-1 months
    - Edition reaches beta, last chance for any backports.
    - Work with Release team to prepare release announcement. [Example][example-blog-release]
- T-0 months
    - Edition is released on stable.

[example-blog-edition-final-call]: https://blog.rust-lang.org/inside-rust/2021/03/04/planning-rust-2021.html
[example-blog-edition-plan]: https://blog.rust-lang.org/2021/05/11/edition-2021.html
[example-blog-public-test]: https://blog.rust-lang.org/2021/07/21/Rust-2021-public-testing.html
[example-blog-release]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

## Crater migration test

Crater does not directly support testing migration lints.
To perform a crater run, a modified version of `cargo` must be used which will perform the steps necessary to migrate a crate.
[#87190](https://github.com/rust-lang/rust/pull/87190) contains an example of what this looks like.
This roughly performs the following steps:

1. Crater runs `cargo check` using the previous master build.
1. Crater runs `cargo check` using the modified `cargo`. This modified `cargo check` will perform the following steps instead of doing a normal check.
1. Copies the package to a temp directory (since the source directory is read-only in crater).
1. Checks if the package's edition is older than the current edition. If so, skip it, since we only want to test migration of the current edition.
1. Runs `cargo fix --edition --allow-no-vcs --allow-dirty`.
1. Modifies `Cargo.toml` to set the new edition.
1. Runs `cargo check` with a special environment variable so that the real `cargo check` can run.

The modified cargo also allows setting the new edition without `cargo-features` being used.

If the final `cargo fix` or `cargo check` steps fail, and the check succeeded on the previous master build, then that signals a regression where the migration failed.

The Edition Project Group is then responsible for analyzing the report, and filing issues for any problems, and following up with teams for getting those fixed.

This process may need to be repeated several times as problems are fixed and need to be re-tested.

Beware that the process of running this and analyzing the reports may take a long time, depending on how many changes are in the Edition.
The 2021 Edition took about a month, which involved analyzing hundreds of regressions, determining root causes, and re-running crater after fixes had been implemented.

## Blog posts and announcements

It is highly encouraged for the Edition Project Group and the involved teams to communicate with everyone early and frequently.
Major milestones should be announced on the Rust blog.
Inside Rust blog posts should be made regularly (such as monthly) with updates about the overall progress and timelines.

Partly in public messaging, but ideally everywhere, **reiterate what editions are and how they work (no code breaks! no ecosystem split!)**.
There is always confusion.
Last time, if I recall, some reporters reached out to the Rust Foundation for clarification.
The Edition Project Group should coordinate with the Foundation team on public messaging, as they will get questions.

## Tracking tools

The Edition Project Group should decide which tools they want to use to track the progress of the edition.
Individual teams will also likely want to choose the tools that are best for them.
In the past, we have used a mix of different tools, such as GitHub Projects, GitHub issue labels, Google Sheets, HackMd, etc.
Use whatever you are comfortable with, just keep in mind that it should be publicly accessible.

Examples:
- [2018 Tracking Issue](https://github.com/rust-lang/rust/issues/44581)
- [2018 GitHub Project](https://github.com/rust-lang/rust/projects/3)
- [2021 Tracking Spreadsheet](https://docs.google.com/spreadsheets/d/1chZ2SL9T444nvU9al1kQ7TJMwC3IVQQV2xIv1HWGQ_k/edit#gid=1034375760)
- [2021 GitHub Project](https://github.com/orgs/rust-lang/projects/7)
- [2021 HackMd](https://hackmd.io/3eG6OZWHRbSMxoRxzwNhGQ?view)
- [2021 Tracking Issue](https://github.com/rust-lang/rust/issues/85811)
- [2024 Tracking Issue](https://github.com/rust-lang/rust/issues/117258)
- [2024 HackMd](https://hackmd.io/@klAMrLz3QN-4luTZek9cWA/H175lzyea)

Examples of individual team tracking:
- [Lang team 2024 roadmap](https://github.com/rust-lang/lang-team/blob/HEAD/src/roadmaps/roadmap-2024.md)
- [Lang team 2024 planning GitHub Project](https://github.com/orgs/rust-lang/projects/43/views/1)
- [Lang team issues label](https://github.com/rust-lang/rust/issues?q=is%3Aissue+label%3Alang-team-202x-edition)
- [Cargo 2024 planning GitHub Project](https://github.com/orgs/rust-lang/projects/30)

## Implementation notes

Individual teams and projects have their own resources for how to implement Edition changes.
The following are some links for additional information if you are looking on how Edition changes are implemented.

- [How migrations work](https://doc.rust-lang.org/nightly/edition-guide/editions/advanced-migrations.html#how-migrations-work) if you need a refresher of how the underlying system works.
- The rustc-dev-guide has an [Editions chapter](https://rustc-dev-guide.rust-lang.org/guides/editions.html) which contains information on how to implement edition-specific changes in rustc.
- The Rust Style Guide has information on [Rust style editions](https://doc.rust-lang.org/nightly/style-guide/editions.html), which define styling changes across editions.

## Historical context

- [RFC 2052] started the Edition system, and kicked off the 2018 Edition.
- [RFC 3085] set the plan for the 2021 Edition, as well as clarifying and changing the meaning of an Edition.
- [RFC 3501] kicked off the 2024 Edition, as well as formalizing the 3-year cadence and establishing the process for managing future Editions.

[RFC 2052]: https://rust-lang.github.io/rfcs/2052-epochs.html
[RFC 3085]: https://rust-lang.github.io/rfcs/3085-edition-2021.html
[RFC 3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html
