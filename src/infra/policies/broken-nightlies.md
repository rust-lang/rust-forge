# Policy on broken nightlies

Sometimes the nightlies released automatically by our CI ends up being broken
for some people or even everyone. This policy defines what the infra team
response will be in those cases.

## Which nightly will be rolled back

A nightly can only be rolled back in the following cases:

* If it contains destructive code, for example if the included compiler deletes
  all the users files.
* If an infra problem caused it to be broken for a big percentage of users on
  any Tier 1 platform. Issues affecting only lower tier platforms are not
  worthy of a roll back, since we don't guarantee working builds for those
  platforms anyway.

A nightly will **not** be rolled back if it's broken by a critical compiler
bug: those bugs are supposed to be caught by CI, and nightly can have compiler
regressions anyway. There are no exceptions, even if big projects are broken
because of this.

## What are we going to fix

Once any member of the infra team decides to roll back a nightly under this
policy we will roll back to the most recent working nightly. The roll back has
to fix installing the nightly with rustup:

```
$ rustup toolchain install nightly
```

It's not required to roll back other things like the documentation or the
manually downloadable artifacts. After the nightly is rolled back we have to
announce the roll back on the `@rustlang` twitter account and on the status
page.
