# Team Maintenance

The roster of the Rust teams is always in flux. From time to time, new people
are added, but also people sometimes opt to into "alumni status", meaning that
they are not currently an active part of the decision-making process.
Unfortunately, whenever a new person is added or someone goes into alumni
status, there are a number of disparate places that need to be updated. This
page aims to document that list. If you have any questions, or need someone with
more privileges to make a change for you, a good place to ask is `#infra` on
Discord.

### Team repo

Membership of teams is primarily driven by the config files in the
[rust-lang/team repo][team repo]. Several systems use the team repo data to
control access:

- the [team website]
- bors r+ rights
- rfcbot interaction
- Mailgun email lists

Team membership is duplicated in a few other places listed below, but the
long-term goal is to centralize on the team repo.

### Full team membership

To make a full team member, the following places need to be modified:

- the [team repo]
- the [rust-lang/TEAM][gh-team] and (in some cases)
  [rust-lang-nursery/TEAM][gh-nursery-team] teams on github must be updated
- the
  [internals discussion board has per-team groups](https://internals.rust-lang.org/admin/groups/custom)
- if the member is going to join the review rotation, they will need to be
  added to the `[assign.owners]` section of `triagebot.toml` of the repos
  where they will be reviewing

### Team member departure

Remove the team member from any and all places:

- 1password
- The [GitHub team][gh-team], [GitHub nursery team][gh-nursery-team]
- [team repo]
- [toolstate notifications]
- `triagebot.toml` files of all repos they were involved in

[gh-team]: https://github.com/orgs/rust-lang/teams
[gh-nursery-team]: https://github.com/orgs/rust-lang-nursery/teams
[team repo]: https://github.com/rust-lang/team/tree/master/teams
[team website]: https://www.rust-lang.org/governance
[toolstate notifications]: https://github.com/rust-lang/rust/blob/master/src/tools/publish_toolstate.py
