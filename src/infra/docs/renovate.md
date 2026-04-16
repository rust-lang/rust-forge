# Renovate

[Renovate](https://docs.renovatebot.com/) is the tool we (the infrastructure team) recommend to
keep dependencies such as crates, GitHub Actions and Docker base images up-to-date.

## About Dependency Updates

> Why keeping dependencies up-to-date?

To get bug fixes, performance improvements, security patches,
new features and have a better developer experience in general.

> How often should dependencies be updated?

Receiving PRs to update dependencies too often is overwhelming.
E.g. we don't recommend receiving a PR for every new version of a dependency.

Instead, we recommend receiving a few PRs on a regular schedule, e.g. once a week or once a month.
E.g. one PR for GitHub Actions updates, one PR for compatible crate updates and one PR for each incompatible crate update.

> Should dependencies updates be automatically merged?

If you have a reliable test suite, and the CI
doesn't automatically deploy to production or publish artifacts
when you merge a PR, then it should
be safe to automerge dependency updates that pass CI checks.

## How to add Renovate to a repository

### 1. Install the renovate GitHub App

Add `bots = ["renovate"]` or `bots = ["forking-renovate"]` to your repository
toml file in the [`team`](https://github.com/rust-lang/team) repository.

E.g. see [annotate-snippets-rs](https://github.com/rust-lang/team/blob/900ea95242ceff029389e5d97917345f480d8665/repos/rust-lang/annotate-snippets-rs.toml#L4)

Here are the differences between the two apps:

- The [`renovate` GitHub App](https://github.com/apps/renovate) creates
  update branches directly in the target repository. That requires write access
  to repository contents. Thanks to this permission, it also supports automerge.
- The [`forking-renovate` GitHub App](https://github.com/apps/forking-renovate)
  creates branches in its own fork and opens PRs back to the target
  repository. It doesn't require any permissions on the target repository,
  but it only works for public repositories and does not support automerge.

### 2. Configure Renovate

Create a `.github/renovate.json5` file with the following content:

```json5
{
  "$schema": "https://docs.renovatebot.com/renovate-schema.json",
  "extends": ["github>rust-lang/renovate"]
}
```

Note:
- If you don't like our default configuration, have a look at [rust-lang/renovate](https://github.com/rust-lang/renovate)
  to learn how to customize our org presets.
- If you don't like any of our org presets, you can also use your own configuration that don't extend any of our presets.
  If you have ideas on how to improve our org presets, PRs are welcome!
- Other file formats and locations are also supported, see the [Renovate documentation](https://docs.renovatebot.com/configuration-options/).
- You can take inspiration from other configuration files in the Rust organization with a
  [GitHub code search for `renovate.json` paths](https://github.com/search?q=org%3Arust-lang+path%3Arenovate.json&type=code).

### 3. Ensure Renovate is working

Check that Renovate created the
[dependency dashboard](https://docs.renovatebot.com/key-concepts/dashboard/)
GitHub issue, so that you can
trigger PRs in the repository by interacting with that issue.

### 4. Troubleshooting Renovate behavior

If you don't understand why Renovate doesn't behave as you expect,
you can run it locally in dry-run mode, and evaluate how it resolves
dependency updates:

```bash

RENOVATE_DRYRUN_TOKEN=$(gh auth token) &&\
docker run --rm -it\
    -e RENOVATE_TOKEN="$RENOVATE_DRYRUN_TOKEN"\
    -e GITHUB_COM_TOKEN="$RENOVATE_DRYRUN_TOKEN"\
    -v /tmp:/tmp\
    -v $PWD:/usr/src/app\
    renovate/renovate:latest renovate --platform=local --repository-cache=reset --dry-run
```

If you need more logs, add `-e LOG_LEVEL=debug` to the command above.  

## Support

If Renovate isn't working, or you have questions, ask in the
[`#t-infra`](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) Zulip channel.
