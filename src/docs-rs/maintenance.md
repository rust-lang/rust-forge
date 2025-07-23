# Common maintenance procedures

## Temporarily remove a crate from the queue

It might happen that a crate fails to build repeatedly due to a docs.rs bug,
clogging up the queue and preventing other crates to build. In this case it's
possible to temporarily remove the crate from the queue until the docs.rs's bug
is fixed. To do that, log into the machine and open a PostgreSQL shell with:

```console
$ psql
```

Then you can run this SQL query to remove the crate:

```psql
UPDATE queue SET attempt = 100 WHERE name = '<CRATE_NAME>';
```

To add the crate back in the queue you can run in the PostgreSQL shell this
query:

```psql
UPDATE queue SET attempt = 0 WHERE name = '<CRATE_NAME>';
```

## Pinning a version of nightly

Sometimes the latest nightly might be broken, causing doc builds to fail. In
those cases it's possible to tell docs.rs to stop updating to the latest
nightly and instead pin a specific release. To do that you need to edit the
`/home/cratesfyi/.docs-rs-env` file, adding or changing this environment
variable:

```console
CRATESFYI_TOOLCHAIN=nightly-YYYY-MM-DD
```

Once the file changed docs.rs needs to be restarted:

```console
systemctl restart docs.rs
```

To return to the latest nightly simply remove the environment variable and
restart docs.rs again.

## Rebuild a specific crate

If a bug was recently fixed, you may want to rebuild a crate so that it builds with the latest version.
From the docs.rs machine:

```console
cratesfyi queue add <crate> <version>
```

This will add the crate with a lower priority than new crates by default, you can change the priority with the `-p` option.

## Raise the limits for a specific crate

Occasionally crates will ask for their build limits to be raised.
You can raise them from the docs.rs machine with `psql`.

Raising a memory limit to 8 GB:

```psql
# memory is measured in bytes
cratesfyi=> INSERT INTO sandbox_overrides (crate_name, max_memory_bytes)
  VALUES ('crate name', 8589934592);
```

Raising a timeout to 15 minutes:

```psql
cratesfyi=> INSERT INTO sandbox_overrides (crate_name, timeout_seconds)
  VALUES ('crate name', 900);
```

Raising limits for multiple crates at once:

```psql
cratesfyi=> INSERT INTO sandbox_overrides (crate_name, max_memory_bytes)
  VALUES ('stm32f4', 8589934592), ('stm32h7', 8589934592), ('stm32g4', 8589934592);
```

## Set a group of crates to be automatically de-prioritized

When many crates from the same project are published at once, they take up a
lot of space in the queue. You can de-prioritize groups of crates at once like
this:

```psql
cratesfyi=> INSERT INTO crate_priorities (pattern, priority)
  VALUES ('group-%', 1);
```

The `pattern` should be a `LIKE` pattern as documented on
<https://www.postgresql.org/docs/current/functions-matching.html>.

Note that this only sets the default priority for crates with that name.
If there are crates already in the queue, you'll have to update those manually:

```psql
cratesfyi=> UPDATE queue SET priority = 1 WHERE name LIKE 'group-%';
```

## Adding all the crates failed after a date back in the queue

After an outage you might want to add all the failed builds back to the queue.
To do that, log into the machine and open a PostgreSQL shell with:

```console
psql
```

Then you can run this SQL query to add all the crates failed after `YYYY-MM-DD
HH:MM:SS` back in the queue:

```psql
UPDATE queue SET attempt = 0 WHERE attempt >= 5 AND build_time > 'YYYY-MM-DD HH:MM:SS';
```

## Removing a crate from the website

Sometimes it might be needed to remove all the content related to a crate from
docs.rs (for example after receiving a DMCA). To do that, log into the server
and run:

```console
cratesfyi database delete-crate CRATE_NAME
```

The command will remove all the data from the database, and then remove the
files from S3.

## Blacklisting crates

Occasionally it might be needed to prevent a crate from being built on docs.rs,
for example if we can't legally host the content of those crates. To add a
crate to the blacklist, preventing new builds for it, you can run:

```console
cratesfyi database blacklist add <CRATE_NAME>
```

Other operations (such as `list` and `remove`) are also supported.

> **Warning:** blacklisting a crate doesn't remove existing content from the
> website, it just prevents new versions from being built!
