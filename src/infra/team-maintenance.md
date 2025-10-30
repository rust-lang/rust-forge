# Team Maintenance

The roster of the Rust teams is always in flux. From time to time, new people
are added, but also people sometimes opt into "alumni status", meaning that
they are not currently an active part of the decision-making process.
Unfortunately, whenever a new person is added or someone goes into alumni
status, there are a number of disparate places that need to be updated.

# Team repo

Membership of teams is primarily driven by the config files in the
[rust-lang/team repo][team repo]. See the README of that repository for the
systems integrated with it.

# Rules for changes to team repo

Pull requests to the repository are merged by the [team-repo-admins], who use these rules to merge PRs:

### `people` and `teams` directories

If a change is related to an individual and does not expand permissions, then only the individual's approval is required. If the change has already been made outside of the team repo (e.g., GitHub username change) then it is considered implicitly approved. This non-exhaustively includes:

* Changing team membership to alumni or full removal
* Changing email address
* Adding zulip-id

If a change will grant additional permissions, then a team lead needs to
approve the change. Any team lead in the "parent team" chain may do so. This includes:

* Adding new subteams under an existing team
* Changing other metadata (website descriptions, Zulip groups, etc.)

### `repo` directory

The `repo` directory differs slightly from the other directories in that it is
not only used to manage access to repositories, but also configures them and
manages their automation.

The following changes must be approved and merged by a `team-repo-admin`:

* Changing access to repositories owned by their team
  * For repositories ownership is not currently formally tracked. Until that is
    added, the team-repo-admins are expected to exercise their understanding of
    which team owns the repository, when in doubt asking for clarification and
    codifying in a comment in the relevant repository.
* Changing `bors.rust.review` permissions
* Changing `perf` permissions
* Changing `crater` permissions

On the other hand, changes to the configuration or automation of repositories
may be approved and merged by `infra-admins`:

* Changing the [general repository settings](https://github.com/rust-lang/team/blob/master/docs/toml-schema.md#general-repository-settings)
  * This includes granting bots access to the repository.
* Changing the [repository branch protections](https://github.com/rust-lang/team/blob/master/docs/toml-schema.md#repository-branch-protections)

Granting a team write access to `rust-lang/rust` requires approval from both an
`infra-admin` and a `team-repo-admin` as that repository is particularly
sensitive.

### Source code changes

The team repository additionally contains code to transform and validate the
TOML user-edited files. This is owned by the Infrastructure team and approval
should be sought for changes.

### Who belongs to team-repo-admins?

This group of people is nominated & approved by the Leadership Council, but is
not selected through any formal criteria. Eventually, we hope that the need for
this group to exist will diminish as additional automation is added to enforce
the above policies.

Note also that the [infra-admins] team maintains "root" credentials to Rust
infrastructure, including the team repo, in order to make changes if needed to
keep infrastructure operational. Those rights should only be exercised when
required though, with team-repo-admins being the first point of contact for changes.
(There may be overlap between the two teams).

# Extra steps for changes

### Full team membership

To make a full team member, the following places need to be modified:

- the [team repo]
- if the member is going to join the review rotation, they will need to be
  added to the `[assign.owners]` section of `triagebot.toml` of the repos
  where they will be reviewing

### Team member departure

Remove the team member from any and all places:

- [team repo]
- `triagebot.toml` files of all repos they were involved in
- 1password

[gh-team]: https://github.com/orgs/rust-lang/teams
[gh-nursery-team]: https://github.com/orgs/rust-lang-nursery/teams
[team repo]: https://github.com/rust-lang/team/tree/master/teams
[team website]: https://www.rust-lang.org/governance
[team-repo-admins]: https://github.com/rust-lang/team/blob/master/teams/team-repo-admins.toml
[infra-admins]: https://github.com/rust-lang/team/blob/master/teams/infra-admins.toml
