# docs.rs

[docs.rs](https://docs.rs/) is a website that hosts documentation for crates published to [crates.io](https://crates.io/).

## External Links

* Source code: [rust-lang/docs.rs][repo]
* Hosted on: `docsrs.infra.rust-lang.org` (behind the bastion -- [how to connect][bastion-connect])
* Maintainers: [docs.rs team]
* [Instance metrics][grafana-instance] (only available to infra team members).
* [Application metrics][grafana-app] (only available to infra team members).

[repo]: https://github.com/rust-lang/docs.rs
[grafana-instance]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=docsrs.infra.rust-lang.org:9100
[grafana-app]: https://grafana.rust-lang.org/d/-wWFg2cZz/docs-rs?orgId=1
[bastion-connect]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/bastion.md#logging-into-servers-through-the-bastion
[docs.rs team]: https://github.com/rust-lang/team/blob/master/teams/docs-rs.toml
