The roster of the Rust subteams is always in flux. From time to time, new people are added, but also people sometimes opt to into "alumni status", meaning that they are not currently an active part of the decision-making process. Unfortunately, whenever a new person is added or someone goes into alumni status, there are a number of disparate places that need to be updated. This page aims to document that list. If you have any questions, or need someone with more privileges to make a change for you, a good place to ask is #rust-infra (or possibly #rust-internals).

### R+ rights

If just giving r+ rights, the following places need to be modified:

- the [homu template on rust-central-station](https://github.com/alexcrichton/rust-central-station/blob/master/homu.toml.template)

### Full feam membership

To make a full team member, the following places need to be modified:

- the [team roster page](https://github.com/rust-lang/rust-www/blob/master/team.md)
- the [rust-lang/TEAM](https://github.com/orgs/rust-lang/teams) and (in some cases) [rust-lang-nursery/TEAM](https://github.com/orgs/rust-lang-nursery/teams) teams on github must be updated
- rfcbot has a separate list of people on a team that is maintained in a database
    - you can ping dikaiosune on IRC, or else prepare a migration
    - [here is an example migration that was adding Carol to the tools team](https://github.com/dikaiosune/rust-dashboard/tree/master/migrations/20170222224139_carols10cents_tools_team)
    - to remove someone, simply reverse the up/down steps
- the easydns service has an e-mail alias (`compiler-team@rust-lang.org`) that needs to be updated
    - best here is to ask around in #rust-infra
- the [internals discussion board has per-team groups](https://internals.rust-lang.org/admin/groups/custom)
