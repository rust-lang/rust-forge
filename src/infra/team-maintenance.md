# Team Maintenance

The roster of the Rust teams is always in flux. From time to time, new people
are added, but also people sometimes opt to into "alumni status", meaning that
they are not currently an active part of the decision-making process.
Unfortunately, whenever a new person is added or someone goes into alumni
status, there are a number of disparate places that need to be updated. This
page aims to document that list. If you have any questions, or need someone with
more privileges to make a change for you, a good place to ask is #rust-infra (or
possibly #rust-internals).

### R+ rights

If just giving r+ rights, the following places need to be modified:

- the [homu template on rust-central-station][homu]

### Full team membership

To make a full team member, the following places need to be modified:

- the [team roster page][team-roster]
- the [rust-lang/TEAM][gh-team] and (in some cases)
  [rust-lang-nursery/TEAM][gh-nursery-team] teams on github must be updated
- rfcbot has a separate list of people on a team that is maintained in a
  configuration file
  - [here is an example commit that was adding Centril to the lang
    team][rfcbot-example]
- the easydns service has an e-mail alias (`compiler-team@rust-lang.org`) that
  needs to be updated
  - best here is to ask around in #rust-infra
- the
  [internals discussion board has per-team groups](https://internals.rust-lang.org/admin/groups/custom)
- the list of reviewers highfive uses is set in [nrc/highfive][highfive]
  - the configs are set per-repo; some teams are listed in `rust.json`, whereas
    those that span multiple repos are set in `_global.json`

### Team member departure

Remove the team member from any and all places:

- [highfive][]
- [reviewers list][homu]
- rfcbot ([example][rfcbot-example])
- 1password
- The [GitHub team][gh-team], [GitHub nursery team][gh-nursery-team]
- email aliases (as above)
- [team roster page][team-roster]

[homu]: https://github.com/alexcrichton/rust-central-station/blob/master/homu.toml.template
[team-roster]: https://github.com/rust-lang/rust-www/blob/master/_data/team.yml
[gh-team]: https://github.com/orgs/rust-lang/teams
[gh-nursery-team]: https://github.com/orgs/rust-lang-nursery/teams
[highfive]: https://github.com/nrc/highfive/tree/master/highfive/configs
[rfcbot-example]: https://github.com/anp/rfcbot-rs/commit/cd54241359cf65742ed5a0fba58ea5114de06f2b#diff-100115fcbdda685c37ba8f73727b0987
