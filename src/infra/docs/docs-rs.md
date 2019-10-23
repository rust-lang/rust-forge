# docs.rs

* Source code: [rust-lang/docs.rs][repo]
* Hosted on: `docsrs.infra.rust-lang.org` (behind the bastion -- [how to connect][bastion-connect])
* Maintainers: [onur][onur], [QuietMisdreavus][QuietMisdreavus]
* [Instance metrics][grafana-instance] (only available to infra team members).
* [Application metrics][grafana-app] (only available to infra team members).

## Common maintenance procedures

### Temporarily remove a crate from the queue

It might happen that a crate fails to build repeatedly due to a docs.rs bug,
clogging up the queue and preventing other crates to build. In this case it's
possible to temporarily remove the crate from the queue until the docs.rs's bug
is fixed. To do that, log into the machine and open a PostgreSQL shell with:

```
$ psql
```

Then you can run this SQL query to remove the crate:

```
UPDATE queue SET attempt = 100 WHERE name = '<CRATE_NAME>';
```

To add the crate back in the queue you can run in the PostgreSQL shell this
query:

```
UPDATE queue SET attempt = 0 WHERE name = '<CRATE_NAME>';
```

### Pinning a version of nightly

Sometimes the latest nightly might be broken, causing doc builds to fail. In
those cases it's possible to tell docs.rs to stop updating to the latest
nightly and instead pin a specific release. To do that you need to edit the
`/home/cratesfyi/.docs-rs-env` file, adding or changing this environment
variable:

```
CRATESFYI_TOOLCHAIN=nightly-YYYY-MM-DD
```

Once the file changed docs.rs needs to be restarted:

```
systemctl restart docs.rs
```

To return to the latest nightly simply remove the environment variable and
restart docs.rs again.

### Adding all the crates failed after a date back in the queue

After an outage you might want to add all the failed builds back to the queue.
To do that, log into the machine and open a PostgreSQL shell with:

```
psql
```

Then you can run this SQL query to add all the crates failed after `YYYY-MM-DD
HH:MM:SS` back in the queue:

```
UPDATE queue SET attempt = 0 WHERE attempt >= 5 AND build_time > 'YYYY-MM-DD HH:MM:SS';
```

[repo]: https://github.com/rust-lang/docs.rs
[grafana-instance]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=docsrs.infra.rust-lang.org:9100
[grafana-app]: https://grafana.rust-lang.org/d/-wWFg2cZz/docs-rs?orgId=1
[bastion-connect]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/bastion.md#logging-into-servers-through-the-bastion
[onur]: https://github.com/onur
[QuietMisdreavus]: https://github.com/QuietMisdreavus
